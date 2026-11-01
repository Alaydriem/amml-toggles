# Task 2 report: fake-key book and watched keys (amm-core)

Saved by the controller from the implementer's hand-back (the subagent could not write files).

Result: `mise run check` exit 0. `mise run test`: 1017 passed, 0 failed. Raw logs are in this folder.

## Public API as built
- `FakeKeyOp { vk: u32, down: bool }`
- `FakeKeys`: `new`, `MAX_OPS = 256`, `hold`, `release`, `release_mod`, `release_all`, `is_held`, `take_ops`, plus `pending` and `drop_ups_since`
- `KeyWatch`: `new`, `MAX_KEYS = 32`, `set`, `forget`, `watchers`
- `ModEvent::FakeKeysReleased`, `ManagerError::InvalidArgument`
- `ManagerService`: `watch_keys`, `send_key`, `take_fake_keys`, `world_left`

## Deviations
- `amm_host_world(false)` reached no ManagerService method. The task added `ManagerService::world_left()`, called from a new `else` branch in `amm-ffi/src/services/host_runtime.rs`.
- The panel opens only through `set_panel_open`, and the release sits on its closed-to-open change.
- ABI stopgap: added `AmmEventKind::FakeKeysReleased`, its conversion arms and an `InvalidArgument` status arm in amm-abi so the build compiles. Task 3 documents them, raises the ABI minor and regenerates the header (the header is stale now).
- The down is queued on the first mod's hold, and the up on the last mod's release. A hold is refused once 256 ops are queued.
- Operator rule: a key-down is NotPermitted while any other mod holds an active Input lease. `publish` compares the Input holders on every lease change, and a newly gained holder releases the other mods' fake keys and sends them FakeKeysReleased.
- The test descriptor gained a `keys.sneak` chord field (the checker needs one for the keybind).

## Tests
- fake_keys: 5 tests
- key_watch: 4 tests
- manager_service: 12 tests from the brief, 9 extra, and 3 for the operator rule

Task Completed
