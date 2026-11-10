use std::path::Path;

use alaydriem_toggles::services::TogglesLink;
use amm_client::test_support::FakeTable;
use amm_core::structs::event::ModEvent;

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

#[test]
fn a_drain_reads_at_most_the_per_tick_bound_and_defers_the_rest_in_order() {
    let mut link = TogglesLink::new(FakeTable::table(), Path::new(FakeTable::CONFIG_DIR)).unwrap();
    let bound = TogglesLink::MAX_EVENTS_PER_TICK;
    let extra = 10;
    let event = |index: usize| ModEvent::Key { vk: index as u32, down: true, repeat: false };
    for index in 0..bound + extra {
        FakeTable::push_event(event(index));
    }

    let first = link.drain();
    assert_eq!(first.len(), bound);
    assert_eq!(first.first().copied(), Some(event(0)));
    assert_eq!(first.last().copied(), Some(event(bound - 1)));

    let second = link.drain();
    assert_eq!(second, (bound..bound + extra).map(event).collect::<Vec<_>>());

    assert!(link.drain().is_empty());
}

#[test]
fn a_drain_of_exactly_the_bound_loses_nothing() {
    let mut link = TogglesLink::new(FakeTable::table(), Path::new(FakeTable::CONFIG_DIR)).unwrap();
    for index in 0..TogglesLink::MAX_EVENTS_PER_TICK {
        FakeTable::push_event(ModEvent::Key { vk: index as u32, down: true, repeat: false });
    }

    assert_eq!(link.drain().len(), TogglesLink::MAX_EVENTS_PER_TICK);
    assert!(link.drain().is_empty());
}
