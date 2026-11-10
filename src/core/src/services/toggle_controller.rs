use crate::config::TogglesConfig;
use crate::structs::{KeyCommand, ResetCause, ToggleInput, ToggleKind};

// Each field holds the virtual key this controller pressed, so a release always matches the press.
#[derive(Debug, Default)]
pub struct ToggleController {
    sneak: Option<u32>,
    walk: Option<u32>,
}

impl ToggleController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, input: ToggleInput, in_gameplay: bool, config: &TogglesConfig) -> Vec<KeyCommand> {
        match input {
            ToggleInput::Chord(kind) => self.chord(kind, in_gameplay, config),
            ToggleInput::KeyDown { vk, repeat } => {
                if repeat {
                    Vec::new()
                } else {
                    self.key_down(vk, config)
                }
            }
            ToggleInput::Reset(cause) => {
                let acts = match cause {
                    ResetCause::Menu => config.reset_on_menu,
                    ResetCause::Death => config.reset_on_death,
                    ResetCause::FocusLost | ResetCause::WorldExit | ResetCause::Disabled => true,
                };
                if acts { self.release_all() } else { Vec::new() }
            }
            ToggleInput::ManagerReleased => {
                self.sneak = None;
                self.walk = None;
                Vec::new()
            }
            ToggleInput::ConfigChanged => self.config_changed(config),
        }
    }

    // drops one toggle without sending a release, for a key-down the manager refused
    pub fn forget(&mut self, kind: ToggleKind) {
        *self.slot_mut(kind) = None;
    }

    pub fn is_on(&self, kind: ToggleKind) -> bool {
        self.slot(kind).is_some()
    }

    fn slot(&self, kind: ToggleKind) -> Option<u32> {
        match kind {
            ToggleKind::Sneak => self.sneak,
            ToggleKind::Walk => self.walk,
        }
    }

    fn slot_mut(&mut self, kind: ToggleKind) -> &mut Option<u32> {
        match kind {
            ToggleKind::Sneak => &mut self.sneak,
            ToggleKind::Walk => &mut self.walk,
        }
    }

    fn enabled(kind: ToggleKind, config: &TogglesConfig) -> bool {
        match kind {
            ToggleKind::Sneak => config.sneak_enabled,
            ToggleKind::Walk => config.walk_enabled,
        }
    }

    fn game_key(kind: ToggleKind, config: &TogglesConfig) -> u32 {
        match kind {
            ToggleKind::Sneak => config.game_sneak_key,
            ToggleKind::Walk => config.game_forward_key,
        }
    }

    fn release(&mut self, kind: ToggleKind) -> Option<KeyCommand> {
        self.slot_mut(kind).take().map(|vk| KeyCommand { vk, down: false })
    }

    fn release_all(&mut self) -> Vec<KeyCommand> {
        [ToggleKind::Sneak, ToggleKind::Walk].into_iter().filter_map(|kind| self.release(kind)).collect()
    }

    fn chord(&mut self, kind: ToggleKind, in_gameplay: bool, config: &TogglesConfig) -> Vec<KeyCommand> {
        if !in_gameplay || !Self::enabled(kind, config) {
            return Vec::new();
        }
        if let Some(command) = self.release(kind) {
            return vec![command];
        }
        let vk = Self::game_key(kind, config);
        *self.slot_mut(kind) = Some(vk);
        vec![KeyCommand { vk, down: true }]
    }

    fn key_down(&mut self, vk: u32, config: &TogglesConfig) -> Vec<KeyCommand> {
        let mut commands = Vec::new();
        if self.walk.is_some()
            && config.cancel_keys.contains(&vk)
            && let Some(command) = self.release(ToggleKind::Walk)
        {
            commands.push(command);
        }
        if self.sneak.is_some()
            && vk == config.game_sneak_key
            && let Some(command) = self.release(ToggleKind::Sneak)
        {
            commands.push(command);
        }
        commands
    }

    fn config_changed(&mut self, config: &TogglesConfig) -> Vec<KeyCommand> {
        [ToggleKind::Sneak, ToggleKind::Walk]
            .into_iter()
            .filter_map(|kind| {
                let stale = self
                    .slot(kind)
                    .is_some_and(|pressed| !Self::enabled(kind, config) || pressed != Self::game_key(kind, config));
                if stale { self.release(kind) } else { None }
            })
            .collect()
    }
}
