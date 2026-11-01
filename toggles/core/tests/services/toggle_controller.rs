use toggles_core::config::TogglesConfig;
use toggles_core::services::ToggleController;
use toggles_core::structs::{KeyCommand, ResetCause, ToggleInput, ToggleKind};

const SHIFT: u32 = 0x10;
const W: u32 = 0x57;
const S: u32 = 0x53;
const D: u32 = 0x44;

fn down(vk: u32) -> KeyCommand {
    KeyCommand { vk, down: true }
}

fn up(vk: u32) -> KeyCommand {
    KeyCommand { vk, down: false }
}

fn key(vk: u32) -> ToggleInput {
    ToggleInput::KeyDown { vk, repeat: false }
}

fn both_on(controller: &mut ToggleController, config: &TogglesConfig) {
    controller.apply(ToggleInput::Chord(ToggleKind::Sneak), true, config);
    controller.apply(ToggleInput::Chord(ToggleKind::Walk), true, config);
}

#[test]
fn forget_clears_only_that_kind() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    both_on(&mut c, &config);

    c.forget(ToggleKind::Walk);

    assert!(c.is_on(ToggleKind::Sneak));
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn sneak_chord_turns_on_and_off() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    let on = c.apply(ToggleInput::Chord(ToggleKind::Sneak), true, &config);
    assert_eq!(on, vec![down(SHIFT)]);
    assert!(c.is_on(ToggleKind::Sneak));
    let off = c.apply(ToggleInput::Chord(ToggleKind::Sneak), true, &config);
    assert_eq!(off, vec![up(SHIFT)]);
    assert!(!c.is_on(ToggleKind::Sneak));
}

#[test]
fn walk_chord_turns_on_and_off() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    let on = c.apply(ToggleInput::Chord(ToggleKind::Walk), true, &config);
    assert_eq!(on, vec![down(W)]);
    assert!(c.is_on(ToggleKind::Walk));
    let off = c.apply(ToggleInput::Chord(ToggleKind::Walk), true, &config);
    assert_eq!(off, vec![up(W)]);
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn chord_ignored_outside_gameplay() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    let out = c.apply(ToggleInput::Chord(ToggleKind::Sneak), false, &config);
    assert!(out.is_empty());
    assert!(!c.is_on(ToggleKind::Sneak));
}

#[test]
fn chord_ignored_when_disabled() {
    let config = TogglesConfig { sneak_enabled: false, walk_enabled: false, ..TogglesConfig::default() };
    let mut c = ToggleController::new();
    assert!(c.apply(ToggleInput::Chord(ToggleKind::Sneak), true, &config).is_empty());
    assert!(c.apply(ToggleInput::Chord(ToggleKind::Walk), true, &config).is_empty());
    assert!(!c.is_on(ToggleKind::Sneak));
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn w_cancels_walk() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    c.apply(ToggleInput::Chord(ToggleKind::Walk), true, &config);
    let out = c.apply(key(W), true, &config);
    assert_eq!(out, vec![up(W)]);
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn s_cancels_walk() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    c.apply(ToggleInput::Chord(ToggleKind::Walk), true, &config);
    let out = c.apply(key(S), true, &config);
    assert_eq!(out, vec![up(W)]);
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn d_does_not_cancel_walk() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    c.apply(ToggleInput::Chord(ToggleKind::Walk), true, &config);
    let out = c.apply(key(D), true, &config);
    assert!(out.is_empty());
    assert!(c.is_on(ToggleKind::Walk));
}

#[test]
fn real_sneak_key_cancels_sneak() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    c.apply(ToggleInput::Chord(ToggleKind::Sneak), true, &config);
    let out = c.apply(key(SHIFT), true, &config);
    assert_eq!(out, vec![up(SHIFT)]);
    assert!(!c.is_on(ToggleKind::Sneak));
}

#[test]
fn repeat_key_down_is_not_a_cancel() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    for vk in [W, S, SHIFT] {
        let out = c.apply(ToggleInput::KeyDown { vk, repeat: true }, true, &config);
        assert!(out.is_empty());
    }
    assert!(c.is_on(ToggleKind::Sneak));
    assert!(c.is_on(ToggleKind::Walk));
}

#[test]
fn menu_reset_respects_setting() {
    let mut config = TogglesConfig::default();
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    config.reset_on_menu = false;
    let out = c.apply(ToggleInput::Reset(ResetCause::Menu), true, &config);
    assert!(out.is_empty());
    assert!(c.is_on(ToggleKind::Sneak));
    config.reset_on_menu = true;
    let out = c.apply(ToggleInput::Reset(ResetCause::Menu), true, &config);
    assert_eq!(out.len(), 2);
    assert!(out.contains(&up(SHIFT)));
    assert!(out.contains(&up(W)));
    assert!(!c.is_on(ToggleKind::Sneak));
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn death_reset_respects_setting() {
    let mut config = TogglesConfig::default();
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    config.reset_on_death = false;
    let out = c.apply(ToggleInput::Reset(ResetCause::Death), true, &config);
    assert!(out.is_empty());
    assert!(c.is_on(ToggleKind::Walk));
    config.reset_on_death = true;
    let out = c.apply(ToggleInput::Reset(ResetCause::Death), true, &config);
    assert_eq!(out.len(), 2);
    assert!(!c.is_on(ToggleKind::Sneak));
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn focus_lost_always_resets() {
    let config = TogglesConfig { reset_on_menu: false, reset_on_death: false, ..TogglesConfig::default() };
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    let out = c.apply(ToggleInput::Reset(ResetCause::FocusLost), true, &config);
    assert_eq!(out.len(), 2);
    assert!(!c.is_on(ToggleKind::Sneak));
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn world_exit_always_resets() {
    let config = TogglesConfig { reset_on_menu: false, reset_on_death: false, ..TogglesConfig::default() };
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    let out = c.apply(ToggleInput::Reset(ResetCause::WorldExit), false, &config);
    assert_eq!(out.len(), 2);
    assert!(!c.is_on(ToggleKind::Sneak));
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn manager_released_clears_without_commands() {
    let config = TogglesConfig::default();
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    let out = c.apply(ToggleInput::ManagerReleased, true, &config);
    assert!(out.is_empty());
    assert!(!c.is_on(ToggleKind::Sneak));
    assert!(!c.is_on(ToggleKind::Walk));
}

#[test]
fn game_key_change_releases_old_key() {
    let mut config = TogglesConfig::default();
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    config.game_sneak_key = 0xA0;
    let out = c.apply(ToggleInput::ConfigChanged, true, &config);
    assert_eq!(out, vec![up(SHIFT)]);
    assert!(!c.is_on(ToggleKind::Sneak));
    assert!(c.is_on(ToggleKind::Walk));
}

#[test]
fn disabling_walk_while_on_releases() {
    let mut config = TogglesConfig::default();
    let mut c = ToggleController::new();
    both_on(&mut c, &config);
    config.walk_enabled = false;
    let out = c.apply(ToggleInput::ConfigChanged, true, &config);
    assert_eq!(out, vec![up(W)]);
    assert!(!c.is_on(ToggleKind::Walk));
    assert!(c.is_on(ToggleKind::Sneak));
}
