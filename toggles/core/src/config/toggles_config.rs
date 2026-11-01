#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TogglesConfig {
    pub sneak_enabled: bool,
    pub walk_enabled: bool,
    pub cancel_keys: Vec<u32>,
    pub game_sneak_key: u32,
    pub game_forward_key: u32,
    pub reset_on_menu: bool,
    pub reset_on_death: bool,
}

impl Default for TogglesConfig {
    fn default() -> Self {
        Self {
            sneak_enabled: true,
            walk_enabled: true,
            cancel_keys: vec![0x57, 0x53],
            game_sneak_key: 0x10,
            game_forward_key: 0x57,
            reset_on_menu: true,
            reset_on_death: true,
        }
    }
}
