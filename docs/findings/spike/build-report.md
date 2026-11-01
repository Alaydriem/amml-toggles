# Spike build report (Task 1)

## What was built
A throwaway AMML mod `toggles-spike` in `C:\Users\charl\projects\levi\toggles\spike\` (own Cargo workspace, Zoom layout, no amm-client).
Crate `toggles-spike/amml` (package `toggles-spike-amml`, lib `toggles_spike`). Output: `spike/dist/toggles-spike/` with `toggles_spike.dll` and `amml.hcl`. Copy that folder to `<game>/mods/amml/toggles-spike/`. Not installed, game not started.

## Entries used (all present in data/6ad79ee4-17d51891a1119914.hcl, 26.60)
ClientInstance.update, ClientInstance.getLocalPlayer, HIDControllerGameCoreDesktop.onKeyDown, HIDControllerGameCoreDesktop.onKeyUp,
Actor.stateVectorComponent, Actor.actorRotationComponent, StateVectorComponent.mPos, ActorRotationComponent.mRot (these four feed amml-game `Actor::pose`),
Actor.getAttachPos, Actor.mEntityContext, ActorAttribute.getHealth (feature game.actor_extra, optional; gives `head_position`).
Hook signatures: onKeyDown(this, key: i32, origin: i32), onKeyUp(this, key: i32), update(this, u8, u64) -> u8.

## How the sequence works
- Key hooks record the last HIDController `this`, the last real `origin`, and the last real Left Shift code (0x10 or 0xA0). They log every real key code, origin and thread id.
- Fake keys call `key_down::GUARD.original_fn()` / `key_up::GUARD.original_fn()` (next in chain, so our own detour is skipped) with the last `this` and real `origin`. They run only from the update hook (game thread).
- While a fake key code is held, the real key-up of the same code is dropped (logged as DROPPED).
- Update hook: waits until `getLocalPlayer` is non-null for 200 consecutive ticks AND a real key was seen (else logs one "waiting ... press any key" line and keeps waiting). Then:
  1. Baseline 100 ticks, 2. Shift DOWN 200 ticks, 3. Shift UP 100 ticks, 4. W DOWN 200 ticks, 5. W UP 100 ticks, 6. `SPIKE done` + summary.
  Position/head logged every 20 ticks. Shift code = last real Left Shift code seen, else 0x10. W = 0x57.
- Manual: real F8 (0x77) toggles fake Shift, F9 (0x78) toggles fake W; processed on the next update tick. F8/F9 real presses pass through to the game.
- "Feet" height: `pos` = StateVectorComponent.mPos (Bedrock may store this at eye level, not feet). Eye value logged is head.y - pos.y; head.y is also logged and summarized alone, so sneak change is visible either way.

## Log line formats (prefix SPIKE, via the mod logger "toggles-spike")
- `SPIKE enabled: hooks installed ...`, `SPIKE feature <f> is off (<cause>)`
- `SPIKE real key down code=0x.. origin=N this=0x.. tid=N`
- `SPIKE real key up code=0x.. this=0x.. tid=N passed|DROPPED (fake key held)`
- `SPIKE update hook thread id=N (was M)`
- `SPIKE local player present, counting 200 ticks`, `SPIKE waiting: no real key seen yet ...`, `SPIKE sequence start: shift code=.. (seen real=..) w code=0x57`
- `SPIKE phase <Baseline|ShiftDown|ShiftUp|WDown|WUp|Done> begin`
- `SPIKE <Phase> t=N pos=(x,y,z) head=(x,y,z) eye(head.y-pos.y)=E`
- `SPIKE fake <Shift|W|Shift (F8)|W (F9)> DOWN sent code=0x.. origin=N this=0x.. tid=N` and `... UP sent code=0x.. this=0x.. tid=N`
- `SPIKE done`, then `SPIKE summary eye height ...`, `SPIKE summary head.y ...`, `SPIKE summary W held 200 ticks: horizontal distance=D`, `SPIKE summary after W up 100 ticks: horizontal distance=D`

## Build
```
cd C:\Users\charl\projects\levi\toggles\spike
mise trust
mise run build-mod
```
Result: success, no warnings. `cargo build --release -p toggles-spike-amml` finished; printed `built dist/toggles-spike`.

## Notes / risks
- Fake sends need a real key first (for `this`/origin); the controller must press any key after entering the world.
- Origin used for fake keys is the origin of the most recent real key-down, which may be a non-keyboard value if the last key was odd; the log shows it.
- Mod manager also hooks the same functions; AMML chains them. Fake keys skip hooks earlier in the chain than ours (they call our next), so the manager does not see them.

Task Completed
