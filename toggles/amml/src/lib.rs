//! @alaydriem/toggles as an AMML mod: hooks the client update, focus loss and level exit, reads the manager's
//! events and the game state, and drives toggles-core with them. The keys are sent through the mod manager.

#![deny(unsafe_code)]

pub mod detours;
pub mod services;
pub mod toggles_mod;
pub mod traits;
pub mod types;

pub use toggles_mod::TogglesMod;

amml_sdk::amml_mod!(TogglesMod);
