use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{PoisonError, RwLock};

use amml_game::{ClientInstance, GameEntries};

use crate::traits::GameView;

// the game's primary ClientInstance and the entries to read it with.
// The client is the first `this` seen by ClientInstance::update: the primary client is created at start-up
// and lives until the game closes, while split-screen clients come and go
pub struct GameClient {
    client: AtomicUsize,
    entries: RwLock<Option<GameEntries>>,
}

impl GameClient {
    pub const fn new() -> Self {
        Self { client: AtomicUsize::new(0), entries: RwLock::new(None) }
    }

    pub fn set_entries(&self, entries: GameEntries) {
        *self.entries.write().unwrap_or_else(PoisonError::into_inner) = Some(entries);
    }

    // keeps the first non-null client; later calls change nothing
    pub fn capture(&self, client: usize) {
        if client != 0 {
            let _ = self.client.compare_exchange(0, client, Ordering::AcqRel, Ordering::Acquire);
        }
    }

    pub fn client(&self) -> usize {
        self.client.load(Ordering::Acquire)
    }

    // None before the first update or without the entries
    fn with_client<R>(&self, body: impl FnOnce(&ClientInstance<'_>) -> R) -> Option<R> {
        let client = self.client();
        if client == 0 {
            return None;
        }
        let entries = self.entries.read().unwrap_or_else(PoisonError::into_inner);
        let entries = entries.as_ref()?;
        // SAFETY: `client` is the ClientInstance* the game passed to ClientInstance::update, and the primary
        // client lives until the game closes; `entries` were resolved on this game build. The calls run on the game
        // thread, inside ClientInstance::update.
        #[allow(unsafe_code)]
        let instance = unsafe { ClientInstance::from_raw(client as *mut c_void, entries) }?;
        Some(body(&instance))
    }
}

impl GameView for GameClient {
    // the client has grabbed the mouse, so no menu, chat or other screen is open; false before the first update
    // or without the entry
    fn in_gameplay(&self) -> bool {
        self.with_client(|client| client.mouse_grabbed()).unwrap_or(false)
    }

    fn player_alive(&self) -> Option<bool> {
        self.with_client(|client| client.local_player().and_then(|player| player.is_alive())).flatten()
    }
}

impl Default for GameClient {
    fn default() -> Self {
        Self::new()
    }
}
