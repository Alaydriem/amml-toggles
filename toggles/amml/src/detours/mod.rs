// The game functions the toggles hook. Each body hands the raw arguments to ToggleRuntime and calls the original;
// bool arguments and results cross as bytes, so no bool invariant is assumed of the game's registers

use amml_sdk::amml_detour;
use toggles_core::structs::ResetCause;

use crate::services::ToggleRuntime;

// bool ClientInstance::update(bool isInitFinished) (26.60: update(bool, bool)): once per frame on the game thread
// 26.60 added a bool third argument (r8). It is carried as a raw u64 so the register reaches the original
// bit for bit; 26.51 ignores r8, so one declaration works on both builds
amml_detour!(pub mod client_update(this: usize, init_finished: u8, extra: u64) -> u8 |original| {
    let result = original(this, init_finished, extra);
    if let Some(runtime) = ToggleRuntime::get() {
        runtime.tick(this);
    }
    result
});

// void MinecraftGame::onAppFocusLost(): Alt+Tab and similar. `this` is the AppPlatformListener subobject
// (MinecraftGame + 0x90), passed on unchanged. The reset waits for the next tick
amml_detour!(pub mod focus_lost(this: usize) -> () |original| {
    original(this);
    if let Some(runtime) = ToggleRuntime::get() {
        runtime.queue_reset(ResetCause::FocusLost);
    }
});

// void ClientInstance::onLevelExit(), the function LeviLamina raises ClientExitLevelEvent from. `this` is the
// GameCallbacks subobject (ClientInstance + 0x88), passed on unchanged. The reset waits for the next tick
amml_detour!(pub mod level_exit(this: usize) -> () |original| {
    if let Some(runtime) = ToggleRuntime::get() {
        runtime.queue_reset(ResetCause::WorldExit);
    }
    original(this);
});
