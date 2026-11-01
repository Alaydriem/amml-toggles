use std::path::Path;

use alaydriem_toggles::services::ToggleRuntime;
use alaydriem_toggles::types::LinkState;
use amm_client::test_support::FakeTable;
use amm_core::structs::event::{KeybindPhase, ModEvent};

use crate::support::fake_game::FakeGame;

const SNEAK: ModEvent = ModEvent::Keybind { action: 0, phase: KeybindPhase::Pressed };
const WALK: ModEvent = ModEvent::Keybind { action: 1, phase: KeybindPhase::Pressed };
const SHIFT: u32 = 0x10;
const W: u32 = 0x57;

fn attached() -> ToggleRuntime {
    let runtime = ToggleRuntime::new(Path::new(FakeTable::CONFIG_DIR));
    assert_eq!(runtime.attach_with(Ok(FakeTable::table())), LinkState::Attached);
    runtime
}

fn sent() -> Vec<(u32, bool)> {
    FakeTable::with(|state| state.sent_keys())
}

#[test]
fn chord_event_sends_key_down() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(SNEAK);
    FakeTable::push_event(WALK);

    runtime.tick_with(&game);

    assert_eq!(sent(), [(SHIFT, true), (W, true)]);
}

#[test]
fn menu_open_sends_key_up() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(SNEAK);
    runtime.tick_with(&game);

    game.set_gameplay(false);
    runtime.tick_with(&game);
    runtime.tick_with(&game);

    assert_eq!(sent(), [(SHIFT, true), (SHIFT, false)]);
}

#[test]
fn death_sends_key_up() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(WALK);
    runtime.tick_with(&game);

    game.set_alive(Some(false));
    runtime.tick_with(&game);

    assert_eq!(sent(), [(W, true), (W, false)]);
}

#[test]
fn fake_keys_released_sends_nothing() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(SNEAK);
    runtime.tick_with(&game);

    FakeTable::push_event(ModEvent::FakeKeysReleased);
    runtime.tick_with(&game);
    game.set_gameplay(false);
    runtime.tick_with(&game);

    assert_eq!(sent(), [(SHIFT, true)]);
}

#[test]
fn settings_change_rewatches() {
    let runtime = attached();
    let game = FakeGame::playing();
    assert_eq!(FakeTable::with(|state| state.watched()), [0x57, 0x53, 0x10]);

    FakeTable::set_config("keys {\n  cancel_walk_1 = \"A\"\n  cancel_walk_2 = \"D\"\n  game_sneak = \"C\"\n}\n");
    FakeTable::push_event(ModEvent::SettingsChanged { paths: 1 });
    runtime.tick_with(&game);

    assert_eq!(FakeTable::with(|state| state.watched()), [0x41, 0x44, 0x43]);
}

#[test]
fn cancel_key_press_releases_walk() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(WALK);
    runtime.tick_with(&game);

    FakeTable::push_event(ModEvent::Key { vk: 0x53, down: true, repeat: false });
    runtime.tick_with(&game);

    assert_eq!(sent(), [(W, true), (W, false)]);
}

#[test]
fn focus_loss_and_world_exit_reset_on_the_next_tick() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(SNEAK);
    FakeTable::push_event(WALK);
    runtime.tick_with(&game);

    runtime.queue_reset(toggles_core::structs::ResetCause::FocusLost);
    runtime.queue_reset(toggles_core::structs::ResetCause::WorldExit);
    runtime.tick_with(&game);

    assert_eq!(sent().len(), 4);
    assert!(sent()[2..].iter().all(|(_, down)| !down));
}

#[test]
fn a_refused_key_down_does_not_leave_the_toggle_on() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::with(|state| state.send_key_status = amm_abi::types::AmmStatus::NotPermitted);
    FakeTable::push_event(SNEAK);
    runtime.tick_with(&game);

    FakeTable::with(|state| state.send_key_status = amm_abi::types::AmmStatus::Ok);
    FakeTable::push_event(SNEAK);
    runtime.tick_with(&game);

    assert_eq!(sent(), [(SHIFT, true), (SHIFT, true)]);
}

#[test]
fn enabled_event_rewatches() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::with(|state| state.watched_keys.clear());

    FakeTable::push_event(ModEvent::Enabled);
    runtime.tick_with(&game);

    assert_eq!(FakeTable::with(|state| state.watched()), [0x57, 0x53, 0x10]);
}

#[test]
fn a_refused_walk_down_keeps_sneak_on() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(SNEAK);
    runtime.tick_with(&game);
    FakeTable::with(|state| state.send_key_status = amm_abi::types::AmmStatus::NotPermitted);
    FakeTable::push_event(WALK);
    runtime.tick_with(&game);

    FakeTable::with(|state| state.send_key_status = amm_abi::types::AmmStatus::Ok);
    FakeTable::push_event(SNEAK);
    runtime.tick_with(&game);

    assert_eq!(sent(), [(SHIFT, true), (W, true), (SHIFT, false)]);
}

#[test]
fn detach_releases_held_keys_and_unregisters() {
    let runtime = attached();
    let game = FakeGame::playing();
    FakeTable::push_event(SNEAK);
    runtime.tick_with(&game);

    runtime.detach();

    assert_eq!(sent(), [(SHIFT, true), (SHIFT, false)]);
    assert_eq!(FakeTable::with(|state| state.unregistered.len()), 1);
    assert_eq!(runtime.link_state(), LinkState::Disabled);
}

#[test]
fn first_tick_without_the_manager_turns_the_toggles_off() {
    let runtime = ToggleRuntime::new(Path::new(FakeTable::CONFIG_DIR));

    assert_eq!(runtime.attach_with(Err(amm_core::errors::LinkError::ManagerMissing)), LinkState::Off);
}
