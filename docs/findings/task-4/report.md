# Task 4 report: manager client API (amm-client)

Saved by the controller from the implementer's hand-back. Results: check passes, test 1031/1031 passes. Logs are in this folder.

- `ManagerLink::watch_keys(&self, vks: &[u32]) -> Result<(), LinkError>`
- `ManagerLink::send_key(&self, vk: u32, down: bool) -> Result<(), LinkError>`
- An absent function (minor 0 or a null slot) returns `LinkError::from(AmmStatus::Unsupported)`.
- Fake table:
  - `FakeTable::with(|s| s.watched())` and `FakeTable::with(|s| s.sent_keys())`.
  - `FakeTable::api_minor_0()` with `FakeTable::link_without_frame_on(...)`.
  - `s.send_key_status` and `s.watch_keys_status` set the status the fakes return.
- Tests: `watch_keys_passes_slice`, `send_key_passes_vk_and_down`, `send_key_on_minor_0_table_is_unsupported`, `send_key_maps_not_permitted`.
- Deviations: none.

Task Completed
