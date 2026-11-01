use toggles_core::config::TogglesConfig;

#[test]
fn default_matches_spec() {
    let config = TogglesConfig::default();
    assert!(config.sneak_enabled);
    assert!(config.walk_enabled);
    assert_eq!(config.cancel_keys, vec![0x57, 0x53]);
    assert_eq!(config.game_sneak_key, 0x10);
    assert_eq!(config.game_forward_key, 0x57);
    assert!(config.reset_on_menu);
    assert!(config.reset_on_death);
}
