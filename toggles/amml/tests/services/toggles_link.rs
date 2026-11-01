use std::path::Path;

use alaydriem_toggles::services::TogglesLink;
use amm_client::test_support::FakeTable;

#[test]
fn link_watches_cancel_and_sneak_keys() {
    let link = TogglesLink::new(FakeTable::table(), Path::new(FakeTable::CONFIG_DIR)).unwrap();

    assert_eq!(FakeTable::with(|state| state.watched()), [0x57, 0x53, 0x10]);
    assert_eq!(link.config().cancel_keys, [0x57, 0x53]);
}

#[test]
fn link_without_send_key_stays_off() {
    let link = FakeTable::link_without_frame_on(FakeTable::api_minor_0());

    assert!(TogglesLink::from_link(link).is_err());
    assert!(FakeTable::with(|state| state.watched()).is_empty());
}
