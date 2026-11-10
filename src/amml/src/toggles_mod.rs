use std::ffi::c_void;

use amml_game::{EntryId, EntryNeed, EntryResolver, GameEntries, GameError};
use amml_sdk::{AmmlMod, Api, ModDirs, SdkError};

use crate::detours::{client_update, focus_lost, level_exit};
use crate::services::{AmmlHost, HookInstaller, ToggleRuntime};
use crate::traits::GameHost;
use crate::types::HookSpec;

// the loader a game host stands in for, as the resolver the game entries are read with
struct HostResolver<'a, H: GameHost>(&'a H);

impl<H: GameHost> EntryResolver for HostResolver<'_, H> {
    fn resolve(&self, id: &str) -> Result<u64, SdkError> {
        self.0.resolve(id)
    }
}

// the AMML entry point: load makes the runtime, enable installs the hooks, disable removes them and leaves the
// mod manager. The manager link is made on the first ClientInstance::update after enable
pub struct TogglesMod;

impl TogglesMod {
    // the id as the module string the loader and the hook host take; a test checks it against `AmmlMod::id`
    pub const MODULE: &'static str = "@alaydriem/toggles";
    pub const UPDATE: &'static str = "ClientInstance.update";
    pub const MOUSE_GRABBED: &'static str = "ClientInstance.getMouseGrabbed";
    pub const GET_LOCAL_PLAYER: &'static str = "ClientInstance.getLocalPlayer";
    pub const IS_ALIVE: &'static str = "Actor.isAlive";
    pub const FOCUS_LOST: &'static str = "MinecraftGame.onAppFocusLost";
    pub const LEVEL_EXIT: &'static str = "ClientExitLevel.hook";

    const DEATH_FEATURE: &'static str = "the reset on death";

    // every hook, in install order; the client update is the toggles themselves, the rest are optional
    pub fn hooks() -> [HookSpec; 3] {
        [
            HookSpec {
                entry: Self::FOCUS_LOST,
                detour: focus_lost::detour as *const c_void,
                original: focus_lost::GUARD.original(),
                required: false,
                feature: "the reset on focus loss",
            },
            HookSpec {
                entry: Self::LEVEL_EXIT,
                detour: level_exit::detour as *const c_void,
                original: level_exit::GUARD.original(),
                required: false,
                feature: "the reset on leaving the world",
            },
            HookSpec {
                entry: Self::UPDATE,
                detour: client_update::detour as *const c_void,
                original: client_update::GUARD.original(),
                required: true,
                feature: "the toggles",
            },
        ]
    }

    // the game reads: the mouse-grabbed slot is required, the death check is optional
    pub fn wanted() -> [(EntryId, EntryNeed); 3] {
        [
            (EntryId::CLIENT_INSTANCE_GET_MOUSE_GRABBED, EntryNeed::Required),
            (EntryId::CLIENT_INSTANCE_GET_LOCAL_PLAYER, EntryNeed::Optional { feature: Self::DEATH_FEATURE }),
            (EntryId::ACTOR_IS_ALIVE, EntryNeed::Optional { feature: Self::DEATH_FEATURE }),
        ]
    }

    // the body of enable: the game entries and the required hook entries must resolve, then the hooks go in and
    // the next tick reaches the manager
    pub fn enable_with<H: GameHost>(host: &H, runtime: &ToggleRuntime) -> Result<(), SdkError> {
        let entries = GameEntries::resolve(&HostResolver(host), &Self::wanted()).map_err(|error| match error {
            GameError::MissingEntry { id } => SdkError::MissingEntry(id),
            other => SdkError::Mod(other.to_string()),
        })?;
        HookInstaller::install(host, runtime, &Self::hooks())?;
        for (feature, cause) in entries.off_features() {
            host.warn(&format!("{feature} is off ({cause})"));
        }
        runtime.client().set_entries(entries);
        runtime.enable_link();
        Ok(())
    }
}

impl AmmlMod for TogglesMod {
    const NAMESPACE: &'static str = "alaydriem";
    const NAME: &'static str = "toggles";

    fn load(api: &Api) -> Result<(), SdkError> {
        let dirs = ModDirs::new(api, Self::MODULE)?;
        ToggleRuntime::install(&dirs.config_dir);
        Ok(())
    }

    fn enable(api: &Api) -> Result<(), SdkError> {
        let runtime = ToggleRuntime::get().ok_or_else(|| SdkError::Mod("load did not run".to_string()))?;
        Self::enable_with(&AmmlHost::new(api, Self::MODULE), runtime)
    }

    fn disable(api: &Api) {
        let Some(runtime) = ToggleRuntime::get() else { return };
        HookInstaller::remove_all(&AmmlHost::new(api, Self::MODULE), runtime);
        runtime.detach();
    }
}
