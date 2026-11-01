use std::path::Path;

use alaydriem_toggles::TogglesMod;
use alaydriem_toggles::services::ToggleRuntime;
use amm_client::test_support::FakeTable;
use amml_sdk::{AmmlMod, SdkError};

use crate::support::fake_host::FakeHost;

const REQUIRED: [(&str, u64); 2] = [(TogglesMod::UPDATE, 0x1000), (TogglesMod::MOUSE_GRABBED, 147)];

fn runtime() -> ToggleRuntime {
    ToggleRuntime::new(Path::new(FakeTable::CONFIG_DIR))
}

#[test]
fn the_mod_id_is_the_namespaced_id_and_amml_hcl_names_the_same_parts() {
    let document: hcl::Value = hcl::from_str(include_str!("../amml.hcl")).unwrap();
    let hcl::Value::Object(map) = document else { panic!("the document is not an object") };

    assert_eq!(TogglesMod::id(), "@alaydriem/toggles");
    assert_eq!(TogglesMod::id(), TogglesMod::MODULE);
    assert_eq!(map.get("namespace").and_then(hcl::Value::as_str), Some(TogglesMod::NAMESPACE));
    assert_eq!(map.get("name").and_then(hcl::Value::as_str), Some(TogglesMod::NAME));
}

#[test]
fn every_entry_installs_every_hook() {
    let mut entries = REQUIRED.to_vec();
    entries.extend([
        (TogglesMod::FOCUS_LOST, 0x4000),
        (TogglesMod::LEVEL_EXIT, 0x6000),
        (TogglesMod::GET_LOCAL_PLAYER, 150),
        (TogglesMod::IS_ALIVE, 20),
    ]);
    let host = FakeHost::new(&entries);

    TogglesMod::enable_with(&host, &runtime()).unwrap();

    assert_eq!(host.hooked(), [TogglesMod::FOCUS_LOST, TogglesMod::LEVEL_EXIT, TogglesMod::UPDATE]);
    assert!(host.warnings().is_empty());
}

#[test]
fn missing_optional_entries_turn_off_only_their_features() {
    let host = FakeHost::new(&REQUIRED);

    TogglesMod::enable_with(&host, &runtime()).unwrap();

    assert_eq!(host.hooked(), [TogglesMod::UPDATE]);
    let warnings = host.warnings();
    assert_eq!(warnings.len(), 3);
    assert!(warnings.iter().any(|line| line.contains("the reset on focus loss")));
    assert!(warnings.iter().any(|line| line.contains("the reset on leaving the world")));
    assert!(warnings.iter().any(|line| line.contains("the reset on death")));
}

#[test]
fn a_missing_required_entry_installs_nothing() {
    let host = FakeHost::new(&[(TogglesMod::UPDATE, 0x1000)]);

    let result = TogglesMod::enable_with(&host, &runtime());

    assert!(result.is_err());
    assert!(host.hooked().is_empty());

    let host = FakeHost::new(&[(TogglesMod::MOUSE_GRABBED, 147)]);
    let result = TogglesMod::enable_with(&host, &runtime());
    assert!(matches!(result, Err(SdkError::MissingEntry(id)) if id == TogglesMod::UPDATE));
    assert!(host.hooked().is_empty());
}
