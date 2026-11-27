use amm_client::link::LinkState;
use amm_core::structs::event::{KeybindPhase, ModEvent};
use amml_api::amml_abi::AmmlReleaseReason;
use amml_api::{Input, KeyCode, Logger};
use toggles_core::config::TogglesConfig;
use toggles_core::services::ToggleController;
use toggles_core::structs::{ResetCause, ToggleInput, ToggleKind};

use crate::services::{KeyHolds, TogglesLink, TogglesLog};

// the toggle state of one enable: the manager link, the controller and its settings, the held game keys and the
// log. The listeners share it behind one lock; every listener runs on the game thread, so the lock is not contended
pub struct TogglesRuntime {
    link: Option<TogglesLink>,
    controller: ToggleController,
    config: TogglesConfig,
    holds: KeyHolds,
    log: TogglesLog,
    configured: bool,
    in_gameplay: bool,
}

impl TogglesRuntime {
    // the keybinds of the descriptor, in order
    const SNEAK_ACTION: u16 = 0;
    const WALK_ACTION: u16 = 1;

    pub fn new(link: TogglesLink, input: Input, log: Logger) -> Self {
        Self {
            link: Some(link),
            controller: ToggleController::new(),
            config: TogglesConfig::default(),
            holds: KeyHolds::new(input),
            log: TogglesLog::new(log),
            configured: false,
            in_gameplay: false,
        }
    }

    // once per client tick: reads the settings once the link is connected, then the manager's events, then the menu
    // and death checks. `alive` is the local player's state, `None` when it cannot be read (no player, no area)
    pub fn tick(&mut self, in_gameplay: bool, alive: Option<bool>) {
        self.in_gameplay = in_gameplay;
        let Some(link) = self.link.as_mut() else { return };
        let state = link.state();
        if state == LinkState::Connected && !self.configured {
            self.configured = true;
            self.config = link.settings();
            self.log.linked();
        }
        let events = link.drain();
        for event in events {
            self.event(event);
        }
        if state == LinkState::Gone && self.link.is_some() {
            self.stop("manager_gone");
        }
        if !in_gameplay {
            self.apply(ToggleInput::Reset(ResetCause::Menu), |_| "menu");
        }
        if let Some(alive) = alive {
            self.log.alive(alive);
        }
        if alive == Some(false) {
            self.apply(ToggleInput::Reset(ResetCause::Death), |_| "death");
        }
    }

    // a real key down the game gets: a cancel key ends auto-walk, the game sneak key ends sneak
    pub fn key_down(&mut self, key: KeyCode) {
        let vk = u32::from(key.generic().0);
        self.apply(ToggleInput::KeyDown { vk, repeat: false }, |kind| match kind {
            ToggleKind::Walk => "cancel_key",
            ToggleKind::Sneak => "real_key",
        });
    }

    pub fn focus_lost(&mut self) {
        self.apply(ToggleInput::Reset(ResetCause::FocusLost), |_| "focus_lost");
    }

    pub fn world_left(&mut self) {
        self.apply(ToggleInput::Reset(ResetCause::WorldExit), |_| "world_left");
        self.log.world_left();
    }

    // the loader released injected input of `lease`: one key (`key`, after a real up or a key reset of the game) or
    // every input. Only the toggles whose keys went up turn off; the game has the keys up already
    pub fn released(&mut self, lease: u64, reason: AmmlReleaseReason, key: Option<KeyCode>) {
        if !self.holds.owns(lease) {
            return;
        }
        let why = Self::release_reason(reason);
        if let Some(key) = key {
            let vk = u32::from(key.generic().0);
            for kind in [ToggleKind::Sneak, ToggleKind::Walk] {
                if self.controller.is_on(kind) && self.game_key(kind) == vk {
                    self.controller.forget(kind);
                    self.log.off(kind, why);
                }
            }
            self.holds.forget(vk);
            return;
        }
        self.apply(ToggleInput::ManagerReleased, |_| why);
        self.holds.forget_all();
    }

    // the loader lost `lease`: its keys are up in the game
    pub fn lease_lost(&mut self, lease: u64) {
        if self.holds.owns(lease) {
            self.apply(ToggleInput::ManagerReleased, |_| "released");
            self.holds.forget_all();
        }
    }

    // disable: the keys go up, the lease ends and the link closes
    pub fn detach(&mut self) {
        self.apply(ToggleInput::Reset(ResetCause::Disabled), |_| "disabled");
        self.holds.forget_all();
        self.link = None;
    }

    // whether the toggles hold the input lease now
    pub fn has_lease(&self) -> bool {
        self.holds.has_lease()
    }

    fn event(&mut self, event: ModEvent) {
        match event {
            ModEvent::Keybind { action, phase: KeybindPhase::Pressed } => {
                let kind = match action {
                    Self::SNEAK_ACTION => ToggleKind::Sneak,
                    Self::WALK_ACTION => ToggleKind::Walk,
                    _ => return,
                };
                self.apply(ToggleInput::Chord(kind), |_| "chord");
            }
            ModEvent::SettingsChanged { .. } => {
                if let Some(link) = &self.link {
                    self.config = link.settings();
                }
                self.apply(ToggleInput::ConfigChanged, |_| "config");
            }
            ModEvent::Disabled => self.apply(ToggleInput::Reset(ResetCause::Disabled), |_| "disabled"),
            ModEvent::ManagerGone => self.stop("manager_gone"),
            _ => {}
        }
    }

    // the manager is gone: the keys go up and the toggles stay off until the next enable
    fn stop(&mut self, reason: &'static str) {
        self.apply(ToggleInput::Reset(ResetCause::Disabled), |_| reason);
        self.holds.forget_all();
        self.link = None;
        self.log.stopped(reason);
    }

    // runs one input through the controller, presses and releases the keys it asks for and logs each toggle that
    // went on or off. A key the loader refuses holds nothing, so that toggle is forgotten
    fn apply(&mut self, input: ToggleInput, reason: impl Fn(ToggleKind) -> &'static str) {
        let before = self.on_state();
        let commands = self.controller.apply(input, self.in_gameplay, &self.config);
        for command in commands {
            if !command.down {
                self.holds.release(command.vk);
                continue;
            }
            if let Err(error) = self.holds.press(command.vk) {
                for kind in [ToggleKind::Sneak, ToggleKind::Walk] {
                    if self.controller.is_on(kind) && self.game_key(kind) == command.vk {
                        self.controller.forget(kind);
                        self.log.refused(kind, &error.to_string());
                    }
                }
            }
        }
        let after = self.on_state();
        for (index, kind) in [ToggleKind::Sneak, ToggleKind::Walk].into_iter().enumerate() {
            match (before[index], after[index]) {
                (false, true) => self.log.on(kind, self.game_key(kind)),
                (true, false) => self.log.off(kind, reason(kind)),
                _ => {}
            }
        }
    }

    fn on_state(&self) -> [bool; 2] {
        [self.controller.is_on(ToggleKind::Sneak), self.controller.is_on(ToggleKind::Walk)]
    }

    fn game_key(&self, kind: ToggleKind) -> u32 {
        match kind {
            ToggleKind::Sneak => self.config.game_sneak_key,
            ToggleKind::Walk => self.config.game_forward_key,
        }
    }

    fn release_reason(reason: AmmlReleaseReason) -> &'static str {
        match reason {
            AmmlReleaseReason::FOCUS_LOST => "focus_lost",
            AmmlReleaseReason::WORLD_LEFT => "world_left",
            AmmlReleaseReason::DISABLED => "disabled",
            _ => "released",
        }
    }
}
