use amm_client::link::{LinkState, ManagerLink};
use amm_core::errors::LinkError;
use amm_core::structs::event::ModEvent;
use amml_api::Ctx;
use toggles_core::config::TogglesConfig;

use crate::services::{RawSettings, SettingsReader, TogglesDescriptor};

// the registration with the mod manager: its keybind and settings events and the settings it holds. `events` is
// cleared at the start of every drain; the link itself keeps a bounded number of unread events. Dropping the link
// removes its listeners, then unregisters
pub struct TogglesLink {
    link: ManagerLink,
    events: Vec<ModEvent>,
}

impl TogglesLink {
    // the link waits for the manager's tables; `state` says when it is connected
    pub fn start(ctx: &Ctx) -> Result<Self, LinkError> {
        let link = ManagerLink::start_without_frame(ctx, TogglesDescriptor::TEXT)?;
        Ok(Self { link, events: Vec::new() })
    }

    pub fn state(&self) -> LinkState {
        self.link.state()
    }

    // the settings the manager holds; text that cannot be read gives the defaults, with one warning
    pub fn settings(&self) -> TogglesConfig {
        match self.link.config::<RawSettings>() {
            Ok(raw) => SettingsReader::convert(&raw),
            Err(error) => {
                curia::warn!("config could not be read; using the defaults", { error: error.to_string() });
                TogglesConfig::default()
            }
        }
    }

    // reads every queued manager event
    pub fn drain(&mut self) -> Vec<ModEvent> {
        self.events.clear();
        self.events.extend(self.link.events());
        self.events.clone()
    }
}
