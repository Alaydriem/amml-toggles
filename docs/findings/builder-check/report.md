# Builder check: toggles design

Questions I still need answered before I can plan. Ranked by importance.

## 1. Does the mod see real key events at all, or does the manager swallow them first?

The spec says the mod hooks `onKeyDown` / `onKeyUp` itself, drops the real key-up, and sees W/S presses for cancel.
I checked `../modmanager-amml`. The manager already detours both functions (`amm-amml/src/game_hooks/keys.rs`).
Its router sets `consume_from_game: true` on a press that matches a live keybind chord, and on the matching release (`input_router.rs`, `release()` and `find()`).
That consumed key never reaches `original`.

- Hook order: when two mods hook one function, which detour runs first, the manager's or ours? Is a second hook on the same target allowed?
  - If the manager runs first and consumes, our hook never sees the F3+W press, so "drop the real key-up" is moot (the game never saw the down).
  - If we run first, we see raw keys, but the toggle chord may arrive twice (raw and as a keybind event).
  - These two cases lead to different designs.
- Is the toggle chord delivered to the mod as a manager keybind event (Pressed phase), or detected from raw hook events? The spec uses both words.

## 2. How do "cancel keys W and S" work, given the manager consumes bound keys?

The settings table lists the cancel keys as a setting. In the manager, a key with a keybind is consumed from the game.

- If W and S are declared as manager keybinds, W and S stop reaching the game. The player cannot walk or step back while the setting is bound. That is a clear regression.
- If they are not manager keybinds, the mod must read them from the raw hook. That depends on question 1.
- Is there a descriptor field kind for a list of keys, or one `chord` field per key (two fields: cancel key 1, cancel key 2)?
- Does the cancel press still go to the game (so the player moves back or forward as usual)? I assume yes. Please confirm.

## 3. Where does the Sneak and Forward key code come from?

"Read the player's binding from the game options" is left to the builder. In `../amml`, `OptionReader` only reads an `Option*` that I already hold (an Int option, by save tag).

- The 26.60 data has no resolved entry that finds an `Option` by tag or id. It has `ClientInstance.getOptions` and a `getHideHud` call only.
- I do not know whether Bedrock key bindings are plain Int options at all. They may live in the input mapping / keyboard layout.
- If no safe source exists, what is the fallback? The spec says "toggle stays off and the mod logs why". That would make the whole mod useless on a default install.
- May I fall back to the default Windows keys (Shift 16 or its VK variant, W 87) when the read fails? Or may I add new AMML recipes? The non-goals forbid "movement-input" recipes. They do not say whether an option-lookup recipe is allowed.
- Which numeric space does the game's `onKeyDown(int keyCode)` use: Windows VK codes or its own codes? The manager has a `normalise` step, so they may differ.
- Left Shift vs Right Shift: which one do I send, and does the game treat both as Sneak?

## 4. What `InputOrigin` value must the fake key-down carry?

The comment in the recipe says `onKeyDown` does extra work only when `origin` is Hardware. The 26.60 data gives no values for the enum.

- Which value makes the game treat the press as real keyboard input for movement? A wrong value may do nothing, or may feed the text-edit path.
- Is it safe to send it while the thread is the game's input thread, or must it come from `ClientInstance.update` (the tick)? Zoom does tick work there. A fake press from another thread could race with real input.

## 5. Does a fake key-down with no key-up from the OS actually keep the action on?

The design depends on the game keeping held state from `onKeyDown` alone.

- Is a periodic re-send needed? Windows key repeat, or the game's own "focus lost clears keys" logic (the manager notes a focus loss releases every key), may clear the fake key.
- The spec lists "focus loss" under builder work. Should the mod re-press after focus returns, or turn the toggle off? The common rules say nothing comes back by itself, but they only mention menus.
- This is the main risk of the approach. Is a failed in-game proof (fake key does not hold movement) a stop point, with a fallback plan, or must I design a fallback up front?

## 6. Key state while a real key is also held

- Player toggles sneak on, then physically taps and releases Shift. The spec drops the real key-up while the toggle is on. Should the real Shift-down be dropped too? After the drop, the game sees one fake down and no up, which is fine. But a second real tap, and then turning the toggle off, may send a down-up pair out of order. Please state the intended result for a real tap while the toggle is on (stay on, or turn off like the cancel keys).
- Auto-walk: if the player holds W (real) when the toggle turns on, then releases it, is that a cancel? The spec says a W press (not release) cancels, but the real key-up is dropped. Confirm that the cancel acts on the down only.

## 7. "Menu opened" and "in gameplay" detection

- The spec offers `getMouseGrabbed` and `isShowingMenu` as equal choices. Zoom uses `getMouseGrabbed`. I need a decision for edge cases such as the pause screen, the manager's own panel (which may release the mouse grab), and the mod manager overlay. Should opening the manager panel count as "a menu opened" and reset the toggles? The common rules say "menu or screen", and the manager panel is a screen.
- Should the toggle chord be ignored (not just "do nothing") when a text field in the panel has focus? The manager normally handles this, but I want it confirmed.

## 8. Release on world exit and unload

The spec has a list of events ("world exit, focus loss, mod unload") but gives no ordering rule.

- During world exit and unload the original `onKeyUp` may point at an object that is gone. Is it acceptable to skip the key-up when the HID controller pointer is not valid, and rely on the game clearing its state on world exit?
- Which AMML callback is the unload hook, and does the manager call it before the hooks are removed? Zoom uses `ClientExitLevel.hook`; is that the intended exit signal here?

## Lower priority

- Keybind capability / claim: the Zoom descriptor needs a `claim` (`Camera`). Which capability does a movement-input mod claim, and is it `Shared` or `Exclusive`? I assume a new or existing name; the manager may refuse an unknown one.
- "F3" alone: is F3 consumed by the manager when it is part of a chord prefix, or does the game also get F3 (the debug screen toggle)? Pressing F3+Shift or F3+W could open the debug overlay. This depends on the other agent's chord work, whose behavior I cannot read.
- Dependency on the other agent's chord work: what is the interface I code against (field kind name, config string such as `"F3+Shift"`)? Without it I cannot build or test the descriptor.

Task Completed
