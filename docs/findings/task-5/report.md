# Task 5 report: manager game side (amm-amml)

Saved by the controller from the implementer's hand-back. Results: check passes, test 1035/1035 passes, build-mod passes. Dist: `../modmanager-amml/dist/alaydriem-mod-manager/`.

- `FakeKeySender`: `new()`, `DEFAULT_ORIGIN = 1`, `note(controller, origin)` and `flush(&dyn GameCalls)`. With no controller, `flush` keeps the ops queued.
- `GameCalls`:
  - `take_fake_keys(&mut Vec<FakeKeyOp>)`: LiveCalls loops `amm_host_take_fake_keys` with a 64-entry buffer.
  - `game_key(controller, vk, down, origin)`: calls `GUARD.original_fn()` of key_down / key_up.
- `flush` runs in `ManagerRuntime::tick` right after the host tick. `note` runs at the start of the key_down detour.
- amm-core moved from dev-dependencies to dependencies of amm-amml (for `FakeKeyOp` in `GameCalls`).
- Tests: `flush_without_controller_keeps_ops`, `flush_sends_in_order_with_last_origin`, `tick_flushes_after_host_tick`, `a_real_key_down_sets_where_fake_keys_go`.
- Deviation: no separate red run (the new type was missing, so the tests could not compile).

Task Completed
