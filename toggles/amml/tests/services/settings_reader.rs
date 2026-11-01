use alaydriem_toggles::services::SettingsReader;
use toggles_core::config::TogglesConfig;

#[test]
fn shift_parses_to_game_code() {
    let config = SettingsReader::read("keys {\n  game_sneak = \"Shift\"\n}\n");

    assert_eq!(config.game_sneak_key, 0x10);
}

#[test]
fn lshift_setting_normalises_to_shift() {
    let config = SettingsReader::read(
        "keys {
  game_sneak = \"VK_0xA0\"
  cancel_walk_1 = \"VK_0xA3\"
  cancel_walk_2 = \"VK_0xA4\"
}
",
    );

    assert_eq!(config.game_sneak_key, 0x10);
    assert_eq!(config.cancel_keys, [0x11, 0x12]);
}

#[test]
fn w_parses_to_0x57() {
    let config =
        SettingsReader::read("keys {\n  game_forward = \"W\"\n  cancel_walk_1 = \"W\"\n  cancel_walk_2 = \"A\"\n}\n");

    assert_eq!(config.game_forward_key, 0x57);
    assert_eq!(config.cancel_keys, [0x57, 0x41]);
}

#[test]
fn bad_key_keeps_default() {
    let config = SettingsReader::read("keys {\n  game_sneak = \"NotAKey\"\n  game_forward = \"Z\"\n}\n");

    assert_eq!(config.game_sneak_key, TogglesConfig::default().game_sneak_key);
    assert_eq!(config.game_forward_key, 0x5A);
}

#[test]
fn flags_are_read() {
    let config = SettingsReader::read("toggles {\n  walk_enabled = false\n  reset_on_menu = false\n}\n");

    assert!(config.sneak_enabled && config.reset_on_death);
    assert!(!config.walk_enabled && !config.reset_on_menu);
}

#[test]
fn unreadable_text_gives_the_defaults() {
    assert_eq!(SettingsReader::read("toggles {"), TogglesConfig::default());
}
