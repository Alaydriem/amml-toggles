//! @alaydriem/toggles as an AMML mod: listens to the loader's client tick, key, focus, world leave and injected
//! input events, reads the manager's keybinds and settings, drives toggles-core with them and holds the game keys
//! through one loader input lease.

#![deny(unsafe_code)]

#[forbid(unsafe_code)]
pub mod services;
#[forbid(unsafe_code)]
pub mod toggles_mod;

pub use toggles_mod::TogglesMod;

amml_api::amml_mod!(TogglesMod);
