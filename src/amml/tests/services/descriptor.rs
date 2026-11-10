use alaydriem_toggles::services::{SettingsReader, TogglesDescriptor};
use amm_core::services::DescriptorChecker;
use amm_core::services::settings::{ChordText, SchemaChecker};
use amm_core::structs::descriptor::{Capability, ClaimMode, Descriptor, KeyScope};
use amm_core::structs::settings::HclDocument;
use toggles_core::config::TogglesConfig;

fn descriptor() -> Descriptor {
    DescriptorChecker::new(128, 256).parse(TogglesDescriptor::TEXT).unwrap()
}

#[test]
fn descriptor_passes_manager_checker() {
    let descriptor = descriptor();

    assert_eq!(descriptor.namespace, "alaydriem");
    assert_eq!(descriptor.name, "toggles");
    assert_eq!(descriptor.id(), "@alaydriem/toggles");
    let claims: Vec<_> = descriptor.claims.iter().map(|claim| (claim.capability, claim.mode)).collect();
    assert_eq!(claims, [(Capability::Input, ClaimMode::Shared)]);
    let keybinds: Vec<_> = descriptor
        .keybinds
        .iter()
        .map(|keybind| (keybind.action.as_str(), keybind.capability, keybind.scope, keybind.default_chord))
        .collect();
    let sneak = ChordText::parse("F3+Shift").unwrap();
    let walk = ChordText::parse("F3+W").unwrap();
    assert_eq!(
        keybinds,
        [("sneak", Capability::Input, KeyScope::Global, sneak), ("walk", Capability::Input, KeyScope::Global, walk)]
    );
}

#[test]
fn defaults_match_spec() {
    let checked = SchemaChecker::check(&descriptor().settings, HclDocument::empty());

    let config = SettingsReader::read(&checked.document.text());

    assert_eq!(config, TogglesConfig::default());
}
