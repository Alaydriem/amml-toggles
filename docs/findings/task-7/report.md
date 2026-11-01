# Task 7 report: toggles AMML mod

Saved by the controller from the implementer's hand-back. Results: check passes, test 39/39 passes (22 toggles-amml + 17 toggles-core), build-mod passes. Dist: `dist/alaydriem-toggles/`.

## Event and state mapping
- Keybind 0 Pressed → `Chord(Sneak)`; keybind 1 Pressed → `Chord(Walk)`.
- `Key` down → `KeyDown`.
- `FakeKeysReleased` → `ManagerReleased`.
- `SettingsChanged` → re-read the settings, `watch_keys` again, then `ConfigChanged`.
- `Disabled` → `Reset(Disabled)`.
- Focus-loss and level-exit detours → `Reset(FocusLost)` / `Reset(WorldExit)` on the next tick.
- `!in_gameplay` → `Reset(Menu)`. A dead local player → `Reset(Death)`.
- A refused key-down → `ManagerReleased`, so the toggle does not stay on.

## Descriptor
- `claim "Input"` Shared.
- Keybinds: sneak `F3+Shift` and walk `F3+W`.
- Fields: `toggles.*` bools, and the chord fields `keys.game_sneak` (Shift), `keys.game_forward` (W), `keys.cancel_walk_1` (W) and `keys.cancel_walk_2` (S). There is no list field kind, so the cancel keys are two chord fields.

## Entries
`ClientInstance.update` (required), `getMouseGrabbed` (required), `getLocalPlayer`, `Actor.isAlive`, `MinecraftGame.onAppFocusLost`, `ClientExitLevel.hook` (optional).

## Deviations
- The session logic is folded into `ToggleRuntime`.
- `GameClient` uses `amml-game` wrappers (a new dependency) instead of a raw vtable read.
- An older manager is tested only at link level.

Task Completed
