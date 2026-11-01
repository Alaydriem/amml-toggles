# Task 1 spike report: fake key presses on 26.60

Instance: `%LOCALAPPDATA%/amml/instances/1.26.60.05-toggles` (PID 50752, now stopped). Mods: `alaydriem-mod-manager` and `toggles-spike`. Raw log: `toggles-spike.log` in this folder. Build notes: `build-report.md`.

## Result: gate passed

| Check | Result | Evidence |
| --- | --- | --- |
| Fake Shift down holds sneak with no key held | Pass | Operator, in game (F8, more than 10 s) |
| Real Shift tap while the fake is held (real up dropped) | Sneak stays | Operator, in game |
| Fake Shift up releases sneak | Pass | Operator, in game |
| Fake W down walks forward | Pass | Log: 3.2 blocks in 200 frames, stopped at a wall (z = 3.7 = 4.0 - 0.3) |
| Fake W up stops | Pass | Log: 0.0 blocks in the next 100 frames; operator F9 check |

## Values for later tasks

- Key codes are Windows VK codes. Shift arrives as `0x10` (no left/right codes seen). W is `0x57`.
- Real key origin: `1`.
- `ClientInstance.update` and the key hooks run on the same thread (tid 36832), so fake keys can be sent from the manager's tick.
- The automatic eye-height test could not measure sneak: on 26.60 the head position equals `StateVectorComponent.mPos`. Sneak was confirmed by eye instead.

Task Completed
