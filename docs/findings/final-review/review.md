# Final review (Opus): toggles feature

Saved by the controller from the reviewer's hand-back. Verdict: do not ship until findings 1-3 are fixed.

1. **Critical.** A chord-off leaves the key stuck. `manager_service.rs` `send_key` / `finish_fake_release` `drop_ups_since` treats any held real key as a handover. When the real down was swallowed by a keybind (F3+W, F3+Shift), the real up is swallowed too, so the game never sees an up. The same applies to `release_fake_all` and `release_fake_mod`. Fix: count it as a handover only if the key is down and the router did not swallow it.
2. **Important.** After disable then enable, the watch set is forgotten and toggles never calls `watch_keys` again, so W, S and Shift no longer cancel. Fix: handle `ModEvent::Enabled` in toggles (call `watch_keys` again).
3. **Important.** When the manager is disabled or unloads, held fake keys are never released (`manager_lifecycle.rs`). Fix: release all and flush before the Tick hooks are removed, inside the `ticked_within` guard.
4. **Minor.** A refused key-down clears both toggles. Fix: forget only the refused kind.
5. **Minor.** Watched `Key` events are not protected from queue overflow. Parked: toggles drains 4 batches per tick, so the risk is low.
6. **Minor.** A game key setting of LShift/RShift (0xA0/0xA1) never matches, because routing normalises them to 0x10. Fix: normalise in `SettingsReader::convert`.

Checked and correct: no self-cancel, the handover when the game saw the real key, focus-loss ordering, FakeKeysReleased protection, ABI back-compat, the Input-holder rule, thread safety.

Task Completed
