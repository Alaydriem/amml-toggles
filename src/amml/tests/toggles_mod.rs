use alaydriem_toggles::TogglesMod;
use amm_client::link::LinkUses;
use amml_api::{AmmlMod, Area};

use crate::support::toggles_rig::TogglesRig;

const W: u16 = 0x57;
const S: u16 = 0x53;
const LSHIFT: u16 = 0xA0;

#[test]
fn walk_chord_presses_the_forward_key() {
    let rig = TogglesRig::new();

    rig.chord(TogglesRig::WALK);

    assert!(rig.logged("info toggles linked"));
    assert!(rig.logged("info toggle on kind=walk key=0x57"));
    assert_eq!(rig.injected(), ["key 0x57 down"]);
}

#[test]
fn second_chord_lets_the_key_go() {
    let rig = TogglesRig::new();

    rig.chord(TogglesRig::WALK);
    rig.chord(TogglesRig::WALK);

    assert!(rig.logged("info toggle off kind=walk reason=chord"));
    assert_eq!(rig.injected(), ["key 0x57 down", "key 0x57 up"]);
}

#[test]
fn injected_echo_keeps_the_walk_on() {
    let rig = TogglesRig::new();

    rig.chord(TogglesRig::WALK);
    rig.tick();
    rig.game.frame();

    assert!(rig.lines("toggle off").is_empty());
}

#[test]
fn cancel_key_ends_the_walk() {
    let rig = TogglesRig::new();
    rig.chord(TogglesRig::WALK);

    rig.tap(S);

    assert!(rig.logged("info toggle off kind=walk reason=cancel_key"));
    assert_eq!(rig.injected(), ["key 0x57 down", "key 0x57 up"]);
}

#[test]
fn real_side_shift_ends_the_sneak() {
    let rig = TogglesRig::new();
    rig.chord(TogglesRig::SNEAK);

    rig.tap(LSHIFT);

    assert!(rig.logged("info toggle on kind=sneak key=0x10"));
    assert!(rig.logged("info toggle off kind=sneak reason=real_key"));
}

#[test]
fn consumed_chord_key_is_no_cancel_key() {
    let rig = TogglesRig::consuming(W);
    rig.chord(TogglesRig::WALK);

    rig.game.real_key(W, true);
    rig.tick();

    assert!(rig.lines("toggle off").is_empty());
}

#[test]
fn menu_ends_the_toggles() {
    let mut rig = TogglesRig::new();
    rig.chord(TogglesRig::WALK);

    rig.set_client(|client| client.mouse_captured = Some(false));
    rig.tick();

    assert!(rig.logged("info toggle off kind=walk reason=menu"));
}

#[test]
fn menu_keeps_the_toggles_when_reset_on_menu_is_off() {
    let mut rig = TogglesRig::with_config("toggles {\n  reset_on_menu = false\n}\n");
    rig.chord(TogglesRig::WALK);

    rig.set_client(|client| client.mouse_captured = Some(false));
    rig.tick();

    assert!(rig.lines("toggle off").is_empty());
}

#[test]
fn death_ends_the_toggles() {
    let mut rig = TogglesRig::with_config("toggles {\n  reset_on_menu = false\n}\n");
    rig.chord(TogglesRig::WALK);

    rig.set_client(|client| client.player_alive = Some(false));
    rig.tick();

    assert!(rig.logged("info death check ready alive=true"));
    assert!(rig.logged("info toggle off kind=walk reason=death"));
}

#[test]
fn death_check_without_the_area_changes_nothing() {
    let mut rig = TogglesRig::new();
    rig.game.withdraw_area(Area::ACTOR_READ);
    rig.chord(TogglesRig::WALK);

    rig.set_client(|client| client.player_alive = Some(false));
    rig.tick();

    assert!(rig.lines("toggle off").is_empty());
}

#[test]
fn focus_loss_ends_the_toggles() {
    let rig = TogglesRig::new();
    rig.chord(TogglesRig::WALK);

    rig.focus_lost();

    assert!(rig.logged("info toggle off kind=walk reason=focus_lost"));
}

#[test]
fn world_leave_ends_the_toggles() {
    let rig = TogglesRig::new();
    rig.chord(TogglesRig::SNEAK);

    rig.world_left();

    assert!(rig.logged("info toggle off kind=sneak reason=world_left"));
}

#[test]
fn real_up_of_one_held_key_ends_only_that_toggle() {
    let rig = TogglesRig::with_config("keys {\n  cancel_walk_1 = \"A\"\n  cancel_walk_2 = \"D\"\n}\n");
    rig.chord(TogglesRig::SNEAK);
    rig.chord(TogglesRig::WALK);

    rig.tap(W);

    assert!(rig.logged("info toggle off kind=walk reason=released"));
    assert_eq!(rig.lines("toggle off").len(), 1);
}

#[test]
fn disable_lets_the_keys_go() {
    let rig = TogglesRig::new();
    rig.chord(TogglesRig::WALK);

    rig.game.disable::<TogglesMod>();
    rig.game.frame();

    assert!(rig.logged("info toggle off kind=walk reason=disabled"));
    assert_eq!(rig.injected(), ["key 0x57 down", "key 0x57 up"]);
}

#[test]
fn refused_key_leaves_the_toggle_off() {
    let rig = TogglesRig::new();
    rig.game.withdraw_area(Area::KEY_INJECT);

    rig.chord(TogglesRig::WALK);
    rig.chord(TogglesRig::WALK);

    assert_eq!(rig.lines("key refused kind=walk").len(), 2);
    assert!(rig.lines("toggle on").is_empty());
    assert!(rig.injected().is_empty());
}

#[test]
fn uses_hold_every_row_of_the_manager_link() {
    assert!(LinkUses::check(&TogglesMod::USES).is_ok());
}

#[test]
fn manifest_uses_block_matches_uses() {
    let manifest = include_str!("../amml.hcl").replace("\r\n", "\n");

    assert_eq!(TogglesMod::USES.write_into(&manifest), manifest);
}

#[test]
fn manager_disable_ends_the_toggles() {
    let rig = TogglesRig::new();
    rig.chord(TogglesRig::SNEAK);

    rig.manager_event(amm_core::structs::event::ModEvent::Disabled);

    assert!(rig.logged("info toggle off kind=sneak reason=disabled"));
    assert_eq!(rig.injected(), ["key 0x10 down", "key 0x10 up"]);
}
