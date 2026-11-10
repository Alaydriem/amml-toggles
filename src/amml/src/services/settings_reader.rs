use amm_core::services::settings::ChordText;
use serde::Deserialize;
use toggles_core::config::TogglesConfig;

// the part of config.hcl the toggles read; a missing value keeps its default. The `keys.sneak` and `keys.walk`
// chords belong to the manager and are not read here
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RawSettings {
    toggles: RawToggles,
    keys: RawKeys,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawToggles {
    sneak_enabled: Option<bool>,
    walk_enabled: Option<bool>,
    reset_on_menu: Option<bool>,
    reset_on_death: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawKeys {
    game_sneak: Option<String>,
    game_forward: Option<String>,
    cancel_walk_1: Option<String>,
    cancel_walk_2: Option<String>,
}

// turns the checked config.hcl into a TogglesConfig: each key setting is a chord text and its key is the game code
pub struct SettingsReader;

impl SettingsReader {
    // text that cannot be read gives the defaults, with one warning
    pub fn read(text: &str) -> TogglesConfig {
        match hcl::from_str::<RawSettings>(text) {
            Ok(raw) => Self::convert(&raw),
            Err(error) => {
                curia::warn!("config could not be read; using the defaults", { error: error.to_string() });
                TogglesConfig::default()
            }
        }
    }

    // the manager routes the left and right Shift, Ctrl and Alt keys as the generic code
    fn normalise(vk: u32) -> u32 {
        match vk {
            0xA0 | 0xA1 => 0x10,
            0xA2 | 0xA3 => 0x11,
            0xA4 | 0xA5 => 0x12,
            other => other,
        }
    }

    pub fn convert(raw: &RawSettings) -> TogglesConfig {
        let defaults = TogglesConfig::default();
        let key = |name: &str, text: &Option<String>, default: u32| match text {
            None => default,
            Some(text) => ChordText::parse(text).map(|chord| Self::normalise(chord.key)).unwrap_or_else(|| {
                curia::warn!("key setting is not a key; keeping the default", { setting: name, value: text.clone() });
                default
            }),
        };
        let first = key("keys.cancel_walk_1", &raw.keys.cancel_walk_1, defaults.cancel_keys[0]);
        let second = key("keys.cancel_walk_2", &raw.keys.cancel_walk_2, defaults.cancel_keys[1]);
        let mut cancel_keys = vec![first];
        if second != first {
            cancel_keys.push(second);
        }
        TogglesConfig {
            sneak_enabled: raw.toggles.sneak_enabled.unwrap_or(defaults.sneak_enabled),
            walk_enabled: raw.toggles.walk_enabled.unwrap_or(defaults.walk_enabled),
            cancel_keys,
            game_sneak_key: key("keys.game_sneak", &raw.keys.game_sneak, defaults.game_sneak_key),
            game_forward_key: key("keys.game_forward", &raw.keys.game_forward, defaults.game_forward_key),
            reset_on_menu: raw.toggles.reset_on_menu.unwrap_or(defaults.reset_on_menu),
            reset_on_death: raw.toggles.reset_on_death.unwrap_or(defaults.reset_on_death),
        }
    }
}
