# Toggles: toggle sneak and auto-walk

Date: 2026-10-30
Status: draft, waiting for review

## Intent

Minecraft Bedrock has no toggle sneak and no auto-walk. The player must hold Shift to sneak and hold W to walk. This mod adds both as toggles: press a key combination once to turn the action on, press it again to turn it off.

The mod is an AMML mod (`../amml`), a client of Alaydriem's Mod Manager (`../modmanager-amml`), built the same way as `../zoom`.

## Features

### 1. Toggle sneak

- Press the sneak toggle (default **F3+Shift**). The player starts to sneak and stays sneaking with no key held.
- Press the sneak toggle again. The player stops sneaking.
- A real press of the game Sneak key (default Shift) while toggle sneak is on turns it off, the same way W/S cancel auto-walk. The Shift press that is part of the F3+Shift toggle chord does not count as a cancel.

### 2. Auto-walk

- Press the auto-walk toggle (default **F3+W**). The player walks forward with no key held. Mouse look and strafe (A, D) work as normal and do not stop it.
- Press the auto-walk toggle again. The player stops.
- Press a cancel key (default **W** or **S**) without F3 held. Auto-walk stops. (Builder call, from the operator's defaults: a W or S press counts as a cancel only when F3 is not held, so the F3+W press that starts auto-walk does not also stop it.)

### Common rules

- The toggle keys work only during gameplay. When a menu, chat, inventory or other screen is open, they do nothing.
- When a menu or screen opens, both toggles turn off. The mod manager's own panel counts as a menu.
- When the game window loses focus (for example alt-tab), both toggles turn off.
- When the player dies, both toggles turn off.
- After a reset the player must press the toggle again. Nothing comes back by itself when the menu closes.
- No on-screen indicator. The player's movement shows the state.

## Non-goals

- No sprint or run toggle. Only the two actions above.
- No support for game builds other than 26.60 (BuildId `6ad79ee4-17d51891a1119914`).
- No controller or touch input. Keyboard only.
- No new AMML recipes for the movement-input state (see "How it works").
- No new AMML recipe to read the game's key bindings. The keys the mod sends are settings instead (see "Settings").
- The F3 chord support in the mod manager is out of scope. Another agent is doing it. This mod declares chord defaults and relies on that work.

## Anti-goals

- A toggle that stays on while a menu is open, or that turns on by itself after a menu closes.
- The F3+W press that starts auto-walk also stops it.
- A key that stays "stuck down" in the game after the toggle turns off, after a reset, or after the mod unloads or the world exits.
- Moving the player in a menu, in chat, or while dead.

## How it works (operator decision)

**Fake key presses, sent through the mod manager.** The operator chose fake key presses over new movement-input recipes, and chose to do all key work through `../modmanager-amml`. The toggles mod has **no keyboard hook of its own**.

The mod manager (`amm-amml/src/game_hooks/keys.rs`) already detours `HIDControllerGameCoreDesktop.onKeyDown` / `onKeyUp` (resolved for 26.60 in `../amml/data/6ad79ee4-17d51891a1119914.hcl`). Its router (`amm-core/src/services/input/input_router.rs`) consumes a key press that matches a live keybind chord, and its release.

- **Toggle keys.** The two toggle chords are manager keybinds. The mod reacts to the manager's `ModEvent::Keybind` event.
- **Cancel keys** (W, S, and the game Sneak key). These are **not** manager keybinds, because a keybind would stop the key from reaching the game. The mod sees them through the new "watch keys" API (below), and they always reach the game.
- **Fake key.** When a toggle turns on, the mod asks the manager, through the new "send key" API, to send one key-down for the "game Sneak key" or "game Forward key" setting. While the toggle is on, the real key-up of that key must not release the action. Releasing Shift after F3+Shift, or W after F3+W, does not stop it. When the toggle turns off (by its chord, a cancel key, a menu, focus loss or death), the mod asks the manager to send one key-up.
- Known side effect, accepted by the operator: the fake press does everything that key does in the game. For example, held Sneak makes the player descend while flying in creative and sink in water.

### Mod manager additions (in `../modmanager-amml`)

The manager today gives raw keys only to the mod with a focused text field, or to holders of the `Input` capability. An `Input` holder takes every key away from the game except its `input_passthrough` list, so it does not fit. The manager has no API to send a key. Two additions are part of this work:

1. **Watch keys.** A mod can get raw key events (`ModEvent::Key`) for a set of keys it names, without taking them from the game and without the `Input` capability. The keys still go through the router as normal.
2. **Send key.** A mod asks the manager to send a key-down or key-up to the game. The manager:
   - marks the event as fake, so it never counts toward a keybind chord (a fake Shift must not count toward F3+Shift) and is not sent back to watchers as a real key;
   - while a fake key-down is held, keeps the real key-up of the same key from releasing it in the game;
   - releases every fake key it holds on focus loss, when its panel opens, when the mod is disabled or unloaded, and on world exit, so no key stays stuck.

3. **Defer to input control (operator rule).** Fake keys touch movement keys, so they yield to any mod that takes input control. While any other mod holds an active `Input` lease (Shared or Exclusive; DejaVu holds Input Shared), `send_key` key-downs are refused. When an `Input` lease becomes active for a mod, the manager releases every other mod's fake keys and sends those mods `FakeKeysReleased`, which turns the toggles off. The toggles mod declares `Input` (Shared) in its descriptor and never takes the lease itself.

Both go through the manager's existing layers: the ABI (`amm-abi`), the C header (`include/alaydriem_mod_manager.h`), `amm-client`, `amm-core` and `amm-amml`. They follow that repo's rules and tests. Another agent is also changing the manager (F3 chord support), so the plan must take care to avoid overlapping edits.

### First proof, then stop if it fails

The first task of the plan is an in-game proof on 26.60: one fake key-down sent from the manager's key path, with the real key-up held back, holds Sneak and holds Forward, and one fake key-up releases it. The proof also settles the `InputOrigin` value and the thread the fake key is sent from.

If the proof fails, the work **stops and the builder reports back to the operator**. The builder does not start option 2 (new movement-input recipes in `../amml`) on its own.

## Existing parts to reuse

| Need | Where it is |
| --- | --- |
| Is a menu open? | `ClientInstance.getMouseGrabbed` (see `../zoom/zoom/amml/src/services/game_client.rs`, `in_gameplay`). `ClientInstance.isShowingMenu` is also resolved |
| Is the player dead? | `ClientInstance.getLocalPlayer` and `Actor.isAlive` (resolved on 26.60; `Actor::is_alive` in `amml-game`) |
| Per-tick work | `ClientInstance.update`, as Zoom uses it |
| Keys | The mod manager's key detours and router, extended as above. Not hooked by the toggles mod |
| Keybinds and settings | Mod manager descriptor (`keybind` blocks, `kind = "chord"` fields, `section` / `field`), as in `../zoom/zoom/amml/descriptor.hcl` |

## Settings

All behavior is set in the mod manager panel and saved to `config.hcl`. The names are the builder's call. The values below are the operator's.

| Setting | Default |
| --- | --- |
| Sneak toggle key | F3+Shift |
| Auto-walk toggle key | F3+W |
| Keys that cancel auto-walk | W, S |
| Game Sneak key (the key the mod presses; match it to the Minecraft binding) | Shift |
| Game Forward key (the key the mod presses; match it to the Minecraft binding) | W |
| Reset toggles when a menu opens | on |
| Reset toggles on death | on |
| Enable toggle sneak | on |
| Enable auto-walk | on |

## Project shape

Like `../zoom`:

- `toggles/core`: all logic (toggle state, cancel rules, reset rules, settings model and clamps). `#![forbid(unsafe_code)]`. No game dependencies. Unit tests here.
- `toggles/amml`: the AMML mod. Game reads (menu, death), the per-tick hook, the manager link (keybind events, watched keys, send key, settings), descriptor. No key hook.
- `../modmanager-amml`: the two additions above, done first.
- Workspace `Cargo.toml` with the same path dependencies as Zoom (`amml-sdk`, `amm-client`, `amm-core`).
- mise tasks for `preflight`, `check`, `test`, `build-mod`, `install-mod`.
- Mod id `alaydriem-toggles` (builder call, following `alaydriem-zoom`).

## Left to the builder

- How a key setting maps to the `onKeyDown` key code (Windows VK code or the game's own code; see the manager's `normalise` step). Whether Left and Right Shift both count.
- The exact shape of the two manager APIs (names, ABI version change, event fields), within the rules above.
- How the manager sends the fake key into the game (through the original `onKeyDown` / `onKeyUp` behind its detour) and from which thread.
- The world-exit signal (Zoom uses `ClientExitLevel`).
- The descriptor capability name to claim. Check which names the mod manager accepts.
- Names, file layout, and the descriptor text.

## Depends on

- The F3 chord work in the mod manager, done by another agent. The plan uses its field kind and config string format (for example `"F3+Shift"`) once it lands.
## Verification

- Unit tests in `toggles/core` for every toggle, cancel and reset rule.
- Tests in `../modmanager-amml` for watch keys and send key: watched keys still reach the game, fake keys never match a chord, the real key-up is held back while a fake key-down is held, and every fake key is released on focus loss, panel open, disable, unload and world exit. The manager's own test suites must still pass.
- `mise run check` and `mise run test` at each task.
- An in-game run on 26.60 at the end: each toggle on and off, W/S cancel, Shift cancel, F3+W and F3+Shift do not self-cancel, reset on menu, on the mod manager panel, on alt-tab and on death, no stuck keys after each reset.
