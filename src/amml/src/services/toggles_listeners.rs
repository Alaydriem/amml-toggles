use std::sync::{Arc, Mutex, PoisonError};

use amml_api::amml_abi::{ClientTick, InjectedInputReleased, InputLeaseLost, KeyInput, WindowFocus, WorldLeft};
use amml_api::{ApiError, Ctx, Event, HookOrder, InjectedInputReleasedExt, KeyInputExt, Listener, WindowFocusExt};

use crate::services::TogglesRuntime;

// the loader listeners of the toggles; each hands its event to the runtime. Dropping them removes them
pub struct TogglesListeners {
    listeners: Vec<Listener>,
    runtime: Arc<Mutex<TogglesRuntime>>,
}

impl TogglesListeners {
    // every listen is required: a refused one fails the enable
    pub fn start(ctx: &Ctx, runtime: Arc<Mutex<TogglesRuntime>>) -> Result<Self, ApiError> {
        let events = ctx.events();
        let mut listeners = Vec::with_capacity(6);
        let ticking = Arc::clone(&runtime);
        listeners.push(events.listen::<ClientTick>(HookOrder::NORMAL, move |_, at| {
            let client = at.client();
            let in_gameplay = client.as_ref().and_then(|client| client.mouse_captured()).unwrap_or(false);
            let alive = client.as_ref().and_then(|client| client.local_player()).and_then(|player| player.is_alive());
            Self::with(&ticking, |runtime| runtime.tick(in_gameplay, alive));
        })?);
        // after the writers, so a chord key the manager consumed (cancelled) is not seen as a cancel key
        let keys = Arc::clone(&runtime);
        listeners.push(events.listen::<KeyInput>(HookOrder::READ_AFTER_WRITERS, move |event, _| {
            if event.down == 0 || event.repeat != 0 || event.is_injected() || event.is_cancelled() {
                return;
            }
            let key = event.key_code();
            Self::with(&keys, |runtime| runtime.key_down(key));
        })?);
        let released = Arc::clone(&runtime);
        listeners.push(events.listen::<InjectedInputReleased>(HookOrder::NORMAL, move |event, _| {
            let (lease, reason, key) = (event.lease, event.reason, event.released_key());
            Self::with(&released, |runtime| runtime.released(lease, reason, key));
        })?);
        let lost = Arc::clone(&runtime);
        listeners.push(events.listen::<InputLeaseLost>(HookOrder::NORMAL, move |event, _| {
            let lease = event.lease;
            Self::with(&lost, |runtime| runtime.lease_lost(lease));
        })?);
        let focus = Arc::clone(&runtime);
        listeners.push(events.listen::<WindowFocus>(HookOrder::NORMAL, move |event, _| {
            if !event.gained() {
                Self::with(&focus, TogglesRuntime::focus_lost);
            }
        })?);
        let leaving = Arc::clone(&runtime);
        listeners.push(events.listen::<WorldLeft>(HookOrder::NORMAL, move |_, _| {
            Self::with(&leaving, TogglesRuntime::world_left);
        })?);
        Ok(Self { listeners, runtime })
    }

    // the listeners go first, so no listener runs while the runtime lets go of the keys and the link
    pub fn stop(self) {
        let Self { listeners, runtime } = self;
        drop(listeners);
        Self::with(&runtime, TogglesRuntime::detach);
    }

    // the runtime is plain state, so a lock poisoned by a panic is recovered
    fn with(runtime: &Mutex<TogglesRuntime>, body: impl FnOnce(&mut TogglesRuntime)) {
        let mut runtime = runtime.lock().unwrap_or_else(PoisonError::into_inner);
        body(&mut runtime);
    }
}
