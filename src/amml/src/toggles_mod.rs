use std::sync::{Arc, Mutex, PoisonError};

use amml_api::amml_abi::{
    ClientTick, InjectedInputReleased, InputLeaseLost, InputLeaseResumed, InputLeaseSuspended, KeyInput, ModsLoaded,
    ProviderGone, ProviderReady, ScreenshotTaken, ScreenshotWritten, WindowFocus, WorldLeft,
};
use amml_api::{AmmlMod, ApiError, Area, Ctx, DisableCtx, Uses};

use crate::services::{TogglesLink, TogglesListeners, TogglesRuntime};

// what the enabled mod owns. `disable` takes it, which drops the listeners, lets the keys go and closes the link
static RUNNING: Mutex<Option<TogglesListeners>> = Mutex::new(None);

// the AMML entry point: enable starts the manager link and the listeners, disable removes them, lets the held keys
// go and leaves the mod manager
pub struct TogglesMod;

impl AmmlMod for TogglesMod {
    const NAMESPACE: &'static str = "alaydriem";
    const NAME: &'static str = "toggles";
    const USES: Uses = amml_api::amml_uses! {
        feature "toggles" {
            required ClientTick;
            required KeyInput;
            required InjectedInputReleased;
            required InputLeaseLost;
            required ProviderReady;
            required ProviderGone;
            required ModsLoaded;
            required InputLeaseSuspended;
            required InputLeaseResumed;
            required area Area::INPUT_LEASE;
            required area Area::KEY_INJECT;
            required area Area::CLIENT_MOUSE;
            // The manager link listens to these when the loader serves them; toggles takes no screenshot
            optional ScreenshotTaken;
            optional ScreenshotWritten;
        }
        feature "reset" {
            required WindowFocus;
            required WorldLeft;
        }
        feature "death" {
            required area Area::CLIENT_READ;
            required area Area::ACTOR_READ;
        }
    };

    fn enable(ctx: &Ctx) -> Result<(), ApiError> {
        let log = ctx.log();
        let link = TogglesLink::start(ctx).map_err(|error| {
            log.error("toggles off", &[("error", &error.to_string())]);
            ApiError::Refused
        })?;
        let runtime = Arc::new(Mutex::new(TogglesRuntime::new(link, ctx.input(), log)));
        let listeners = TogglesListeners::start(ctx, runtime).inspect_err(|error| {
            log.error("toggles off", &[("error", &error.to_string())]);
        })?;
        *RUNNING.lock().unwrap_or_else(PoisonError::into_inner) = Some(listeners);
        Ok(())
    }

    fn disable(_ctx: &Ctx, _how: &DisableCtx) {
        let running = RUNNING.lock().unwrap_or_else(PoisonError::into_inner).take();
        if let Some(listeners) = running {
            listeners.stop();
        }
    }
}
