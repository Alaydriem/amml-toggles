use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};

use amm_abi::types::AmmStatus;
use amm_client::link::ApiTable;
use amm_core::errors::LinkError;
use amm_core::structs::event::{KeybindPhase, ModEvent};
use amml_sdk::{HookHandle, ModRuntime};
use toggles_core::config::TogglesConfig;
use toggles_core::services::ToggleController;
use toggles_core::structs::{ResetCause, ToggleInput, ToggleKind};

use crate::services::{GameClient, TogglesLink};
use crate::traits::GameView;
use crate::types::LinkState;

// the manager link, the toggle state and the settings of one attach; they live and go together
struct Active {
    link: TogglesLink,
    controller: ToggleController,
    config: TogglesConfig,
}

impl Active {
    fn new(link: TogglesLink) -> Self {
        let config = link.config().clone();
        Self { link, controller: ToggleController::new(), config }
    }

    // gives one input to the controller and sends the keys it asks for. A key-down the manager refuses leaves
    // nothing held, so the controller forgets that toggle
    fn apply(&mut self, input: ToggleInput, in_gameplay: bool) {
        let commands = self.controller.apply(input, in_gameplay, &self.config);
        for command in commands {
            if !self.link.send(command) && command.down {
                self.forget_refused(command.vk);
            }
        }
    }

    // the toggle whose game key is `vk` holds nothing, so only that toggle is cleared
    fn forget_refused(&mut self, vk: u32) {
        for (kind, key) in
            [(ToggleKind::Sneak, self.config.game_sneak_key), (ToggleKind::Walk, self.config.game_forward_key)]
        {
            if key == vk {
                self.controller.forget(kind);
            }
        }
    }

    fn is_on(&self) -> bool {
        self.controller.is_on(ToggleKind::Sneak) || self.controller.is_on(ToggleKind::Walk)
    }
}

// what the hooks share: the game client, the manager link state, the queued resets and the installed hooks.
// The mod manager is reached on the first ClientInstance::update after enable, when every mod of the host
// has loaded; `hooks` holds at most HOOK_CAPACITY handles
pub struct ToggleRuntime {
    client: GameClient,
    config_dir: PathBuf,
    link_state: AtomicU8,
    active: Mutex<Option<Active>>,
    pending: AtomicU8,
    hooks: Mutex<Vec<HookHandle>>,
}

static RUNTIME: OnceLock<ToggleRuntime> = OnceLock::new();

impl ToggleRuntime {
    pub const HOOK_CAPACITY: usize = 8;

    // the keybinds of the descriptor, in order
    const SNEAK_ACTION: u16 = 0;
    const WALK_ACTION: u16 = 1;

    const FOCUS_LOST: u8 = 1;
    const WORLD_EXIT: u8 = 2;

    pub fn new(config_dir: &Path) -> Self {
        Self {
            client: GameClient::new(),
            config_dir: config_dir.to_path_buf(),
            link_state: AtomicU8::new(LinkState::Waiting.to_u8()),
            active: Mutex::new(None),
            pending: AtomicU8::new(0),
            hooks: Mutex::new(Vec::with_capacity(Self::HOOK_CAPACITY)),
        }
    }

    // the runtime of this DLL; the first call wins
    pub fn install(config_dir: &Path) -> &'static ToggleRuntime {
        RUNTIME.get_or_init(|| Self::new(config_dir))
    }

    pub fn get() -> Option<&'static ToggleRuntime> {
        RUNTIME.get()
    }

    pub fn client(&self) -> &GameClient {
        &self.client
    }

    pub fn link_state(&self) -> LinkState {
        LinkState::from_u8(self.link_state.load(Ordering::SeqCst))
    }

    // enable: the next tick reaches the manager again
    pub fn enable_link(&self) {
        self.link_state.store(LinkState::Waiting.to_u8(), Ordering::SeqCst);
    }

    // once per ClientInstance::update on the game thread, after the original: keeps the client, reaches the
    // manager on the first call after enable, then handles the manager's events and the game state
    pub fn tick(&self, client: usize) {
        self.client.capture(client);
        self.tick_with(&self.client);
    }

    pub fn tick_with<G: GameView>(&self, game: &G) {
        if self.link_state() == LinkState::Waiting {
            self.attach_with(ApiTable::resolve());
        }
        if self.link_state() == LinkState::Attached {
            self.run(game);
        }
    }

    // registers with the manager the table belongs to; on failure the toggles stay off and one line is logged.
    // Only a Waiting runtime takes the result: if disable ran meanwhile, a new link is dropped again
    pub fn attach_with(&self, table: Result<ApiTable, LinkError>) -> LinkState {
        let state = match table.and_then(|table| TogglesLink::new(table, &self.config_dir)) {
            Ok(link) => {
                *self.active.lock().unwrap_or_else(PoisonError::into_inner) = Some(Active::new(link));
                curia::info!("@alaydriem/toggles attached to the mod manager");
                LinkState::Attached
            }
            Err(error) => {
                Self::log_link_error(&error);
                LinkState::Off
            }
        };
        let waiting = LinkState::Waiting.to_u8();
        match self.link_state.compare_exchange(waiting, state.to_u8(), Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => state,
            Err(current) => {
                if state == LinkState::Attached {
                    drop(self.active.lock().unwrap_or_else(PoisonError::into_inner).take());
                }
                LinkState::from_u8(current)
            }
        }
    }

    // disable: the keys go up and the mod unregisters; no tick reaches the manager until the next enable
    pub fn detach(&self) {
        self.link_state.store(LinkState::Disabled.to_u8(), Ordering::SeqCst);
        let taken = self.active.lock().unwrap_or_else(PoisonError::into_inner).take();
        if let Some(mut active) = taken {
            active.apply(ToggleInput::Reset(ResetCause::Disabled), false);
        }
    }

    // the focus-loss and level-exit detours: the reset is made on the next tick, on the game thread
    pub fn queue_reset(&self, cause: ResetCause) {
        let bit = match cause {
            ResetCause::FocusLost => Self::FOCUS_LOST,
            ResetCause::WorldExit => Self::WORLD_EXIT,
            ResetCause::Menu | ResetCause::Death | ResetCause::Disabled => return,
        };
        self.pending.fetch_or(bit, Ordering::SeqCst);
    }

    // keeps an installed hook; false, keeping nothing, when HOOK_CAPACITY hooks are already kept
    pub fn keep_hook(&self, hook: HookHandle) -> bool {
        let mut hooks = self.hooks.lock().unwrap_or_else(PoisonError::into_inner);
        if hooks.len() >= Self::HOOK_CAPACITY {
            return false;
        }
        hooks.push(hook);
        true
    }

    // the installed hooks, newest first, leaving none behind
    pub fn take_hooks(&self) -> Vec<HookHandle> {
        let mut hooks = self.hooks.lock().unwrap_or_else(PoisonError::into_inner);
        let mut taken = std::mem::take(&mut *hooks);
        taken.reverse();
        taken
    }

    fn run<G: GameView>(&self, game: &G) {
        let Ok(mut slot) = self.active.lock() else { return };
        let Some(active) = slot.as_mut() else { return };
        let in_gameplay = game.in_gameplay();
        for event in active.link.drain() {
            match event {
                ModEvent::Keybind { action, phase: KeybindPhase::Pressed } => {
                    let kind = match action {
                        Self::SNEAK_ACTION => ToggleKind::Sneak,
                        Self::WALK_ACTION => ToggleKind::Walk,
                        _ => continue,
                    };
                    active.apply(ToggleInput::Chord(kind), in_gameplay);
                }
                ModEvent::Key { vk, down: true, repeat } => {
                    active.apply(ToggleInput::KeyDown { vk, repeat }, in_gameplay)
                }
                ModEvent::FakeKeysReleased => active.apply(ToggleInput::ManagerReleased, in_gameplay),
                ModEvent::SettingsChanged { .. } => {
                    active.config = active.link.reload().clone();
                    active.apply(ToggleInput::ConfigChanged, in_gameplay);
                }
                ModEvent::Enabled => active.link.rewatch(),
                ModEvent::Disabled => active.apply(ToggleInput::Reset(ResetCause::Disabled), in_gameplay),
                _ => {}
            }
        }
        let pending = self.pending.swap(0, Ordering::SeqCst);
        if pending & Self::FOCUS_LOST != 0 {
            active.apply(ToggleInput::Reset(ResetCause::FocusLost), in_gameplay);
        }
        if pending & Self::WORLD_EXIT != 0 {
            active.apply(ToggleInput::Reset(ResetCause::WorldExit), in_gameplay);
        }
        if !in_gameplay {
            active.apply(ToggleInput::Reset(ResetCause::Menu), in_gameplay);
        }
        if active.is_on() && game.player_alive() == Some(false) {
            active.apply(ToggleInput::Reset(ResetCause::Death), in_gameplay);
        }
    }

    fn log_link_error(error: &LinkError) {
        let Some(runtime) = ModRuntime::get() else { return };
        let unsupported = LinkError::from(AmmStatus::Unsupported);
        match error {
            LinkError::ManagerMissing | LinkError::AbiMismatch { .. } => {
                runtime.logger().error("@alaydriem/toggles needs @amml/modmanager 0.2.x; toggles are off");
            }
            other if *other == unsupported => {
                runtime.logger().warn("@amml/modmanager is too old to send keys; update it. Toggles are off");
            }
            other => {
                runtime
                    .logger()
                    .error(&format!("@amml/modmanager refused @alaydriem/toggles ({other}); toggles are off"));
            }
        }
    }
}
