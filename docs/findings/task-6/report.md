# Task 6 report: toggles workspace and toggles-core

Saved by the controller from the implementer's hand-back. Results: preflight passes, check passes, test 17/17 passes. Logs are in this folder.

- Workspace: members toggles/core and toggles/amml. mise tasks preflight, check, test, build-mod, install-mod (no bench). `mise.local.toml` has no LM_MODS_DIR.
- `toggles_core::config::TogglesConfig`: defaults are cancel `[0x57, 0x53]`, sneak key 0x10, forward key 0x57, all bools true.
- `toggles_core::structs`: `ToggleKind`, `ResetCause`, `ToggleInput`, `KeyCommand`.
- `toggles_core::services::ToggleController`: `new`, `apply(input, in_gameplay, &config) -> Vec<KeyCommand>`, `is_on(kind)`. It remembers the key it pressed for each toggle.
- Tests: `default_matches_spec` and 16 controller tests (the names are in the plan).
- Deviations:
  - toggles-core has no dependencies.
  - toggles-amml is still an empty lib; Task 7 adds its dependencies and `amml.hcl`.
  - If one key-down matches both a cancel key and the sneak key, it cancels both toggles.
  - `cargo fmt --all` was run once directly.

Task Completed
