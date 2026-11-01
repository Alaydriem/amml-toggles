# Task 3 report: manager ABI, FFI and header

Saved by the controller from the implementer's hand-back. Results: `mise run check` passes, `mise run test` 1027/1027 passes, `header` and `header-check` pass. Logs are in this folder.

## As built
- `AmmApi::watch_keys(handle, vks: *const u32, count: usize) -> AmmStatus` and `AmmApi::send_key(handle, vk: u32, down: u8) -> AmmStatus` at the end of the table. `AMM_ABI_MINOR = 1`.
- `AmmEventKind::FakeKeysReleased` is last (value 18), with the doc "Since 2.1".
- `amm_ffi::types::AmmHostFakeKey { vk: u32, down: u8 }` (8 bytes).
- `amm_ffi::host::amm_host_take_fake_keys(out: *mut AmmHostFakeKey, capacity: usize) -> usize` is `unsafe extern "C"`, follows the other out-pointer host functions, and keeps the rest of the queue for the next call.
- The new `Boundary::values<T>` helper reads the key slice.
- The header is regenerated.

## Deviations
- The 8-byte layout test is in `amm-ffi/tests/types/host_fake_key.rs`, because amm-abi cannot see amm-ffi types.
- `amm-client` fake table: the new slots are `None` for now (Task 4 fills them).
- `amm-amml/tests/api_export.rs`: the expected ABI version is now (2, 1).
- The `HostRuntime` carry-over buffer has no cap of its own; it is bounded by the core queue's cap of 256 ops.

Task Completed
