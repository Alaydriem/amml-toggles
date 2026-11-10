use std::path::Path;

use amm_client::link::{ApiTable, EventDrain, ManagerLink};
use amm_core::errors::LinkError;
use amm_core::structs::event::ModEvent;
use toggles_core::config::TogglesConfig;
use toggles_core::structs::KeyCommand;

use crate::services::{RawSettings, SettingsReader, TogglesDescriptor};

// the registration with the mod manager and the settings read from it; `events` holds at most
// MAX_EVENTS_PER_TICK and is cleared at the start of every drain
pub struct TogglesLink {
    link: ManagerLink,
    config: TogglesConfig,
    events: Vec<ModEvent>,
    refusing: bool,
}

impl TogglesLink {
    pub const MAX_DRAINS_PER_TICK: usize = 4;
    pub const MAX_EVENTS_PER_TICK: usize = Self::MAX_DRAINS_PER_TICK * EventDrain::CAPACITY;

    pub fn new(table: ApiTable, config_dir: &Path) -> Result<Self, LinkError> {
        let link = ManagerLink::without_frame(table, TogglesDescriptor::TEXT, config_dir)?;
        Self::from_link(link)
    }

    // reads the settings and asks for the cancel and sneak keys; a manager without watch_keys is an error, so
    // the toggles stay off
    pub fn from_link(link: ManagerLink) -> Result<Self, LinkError> {
        let mut this = Self {
            config: TogglesConfig::default(),
            link,
            events: Vec::with_capacity(Self::MAX_EVENTS_PER_TICK),
            refusing: false,
        };
        this.config = this.read_settings();
        this.link.watch_keys(&Self::watched(&this.config))?;
        Ok(this)
    }

    pub fn config(&self) -> &TogglesConfig {
        &self.config
    }

    // the keys the mod wants reports for: the cancel keys, then the game sneak key, each once
    pub fn watched(config: &TogglesConfig) -> Vec<u32> {
        let mut keys = Vec::with_capacity(config.cancel_keys.len() + 1);
        for key in config.cancel_keys.iter().copied().chain([config.game_sneak_key]) {
            if !keys.contains(&key) {
                keys.push(key);
            }
        }
        keys
    }

    // reads the settings again and watches the keys they name
    pub fn reload(&mut self) -> &TogglesConfig {
        self.config = self.read_settings();
        self.rewatch();
        &self.config
    }

    // asks the manager again for the keys the current settings name; a disabled mod loses its watch set
    pub fn rewatch(&self) {
        if let Err(error) = self.link.watch_keys(&Self::watched(&self.config)) {
            curia::warn!("watched keys could not be set", { error: error.to_string() });
        }
    }

    // reads the queued manager events, at most MAX_EVENTS_PER_TICK; the rest wait for the next tick
    pub fn drain(&mut self) -> Vec<ModEvent> {
        self.events.clear();
        for _ in 0..Self::MAX_DRAINS_PER_TICK {
            let before = self.events.len();
            self.events.extend(self.link.events());
            if self.events.len() - before < EventDrain::CAPACITY {
                break;
            }
        }
        self.events.clone()
    }

    // sends one key; false when the manager refused. The first refusal of a run is logged at info, the rest at
    // debug, so a refusal never fills the log
    pub fn send(&mut self, command: KeyCommand) -> bool {
        match self.link.send_key(command.vk, command.down) {
            Ok(()) => {
                self.refusing = false;
                true
            }
            Err(error) => {
                if self.refusing {
                    curia::debug!("key refused", { vk: command.vk, down: command.down, error: error.to_string() });
                } else {
                    curia::info!("key refused", { vk: command.vk, down: command.down, error: error.to_string() });
                }
                self.refusing = true;
                false
            }
        }
    }

    fn read_settings(&self) -> TogglesConfig {
        match self.link.config::<RawSettings>() {
            Ok(raw) => SettingsReader::convert(&raw),
            Err(error) => {
                curia::warn!("config could not be read; using the defaults", { error: error.to_string() });
                TogglesConfig::default()
            }
        }
    }
}
