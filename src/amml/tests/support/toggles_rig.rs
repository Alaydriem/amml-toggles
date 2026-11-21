use alaydriem_toggles::TogglesMod;
use amm_client::test_support::FakeTable;
use amm_core::structs::event::{KeybindPhase, ModEvent};
use amml_api::amml_abi::{ClientTick, KeyInput, WindowFocus, WorldLeft};
use amml_api::testing::{FakeClient, FakeGame};
use amml_api::{Area, Cancel, HookOrder, Listener};

// the toggles mod enabled on a fake game that serves every row of its `USES`, with the manager provided, the mouse
// captured and a living player
pub struct TogglesRig {
    pub game: FakeGame,
    client: FakeClient,
    _consumer: Option<Listener>,
}

impl TogglesRig {
    pub const SNEAK: u16 = 0;
    pub const WALK: u16 = 1;

    pub fn new() -> Self {
        Self::build("", None)
    }

    // the manager holds `config` as the mod's config.hcl
    pub fn with_config(config: &str) -> Self {
        Self::build(config, None)
    }

    // a listener of another mod in the write band cancels every real `key` down, as the manager does with the key of
    // a chord it consumed
    pub fn consuming(key: u16) -> Self {
        Self::build("", Some(key))
    }

    fn build(config: &str, consumed: Option<u16>) -> Self {
        let game = FakeTable::game();
        game.offer::<ClientTick>();
        game.offer::<KeyInput>();
        game.offer::<WindowFocus>();
        game.offer::<WorldLeft>();
        for area in [Area::CLIENT_MOUSE, Area::CLIENT_READ, Area::ACTOR_READ] {
            game.offer_area(area);
        }
        FakeTable::provide(&game.ctx()).unwrap();
        FakeTable::set_config(config);
        let consumer = consumed.map(|key| Self::consumer(&game, key));
        game.load::<TogglesMod>().unwrap();
        game.enable::<TogglesMod>().unwrap();
        let client = FakeClient {
            local_player: true,
            world_active: true,
            mouse_captured: Some(true),
            player_alive: Some(true),
            ..FakeClient::default()
        };
        game.set_client(client);
        let rig = Self { game, client, _consumer: consumer };
        rig.tick();
        rig
    }

    // the client the game reports from the next tick on
    pub fn set_client(&mut self, change: impl FnOnce(&mut FakeClient)) {
        change(&mut self.client);
        self.game.set_client(self.client);
    }

    pub fn tick(&self) {
        self.game.raise_with_client(&mut ClientTick { header: FakeGame::header::<ClientTick>() });
    }

    // the manager's keybind press `action`, read at the next tick; the injected key reaches the game at the frame
    pub fn chord(&self, action: u16) {
        FakeTable::push_event(ModEvent::Keybind { action, phase: KeybindPhase::Pressed });
        self.tick();
        self.game.frame();
    }

    pub fn manager_event(&self, event: ModEvent) {
        FakeTable::push_event(event);
        self.tick();
        self.game.frame();
    }

    // a real key press and release from the keyboard, then a frame
    pub fn tap(&self, key: u16) {
        self.game.real_key(key, true);
        self.game.real_key(key, false);
        self.game.frame();
    }

    fn consumer(game: &FakeGame, key: u16) -> Listener {
        game.ctx_named("@test/manager")
            .events()
            .listen::<KeyInput>(HookOrder::WRITE, move |event, _| {
                if event.key == key && event.down != 0 && event.header.injected == 0 {
                    event.cancel();
                }
            })
            .unwrap()
    }

    pub fn focus_lost(&self) {
        let mut event =
            WindowFocus { header: FakeGame::header::<WindowFocus>(), gained: 0, replayed: 0, _reserved: [0; 2] };
        self.game.raise(&mut event);
        self.game.frame();
    }

    pub fn world_left(&self) {
        self.game.raise(&mut WorldLeft { header: FakeGame::header::<WorldLeft>(), epoch: 0 });
        self.game.frame();
    }

    pub fn injected(&self) -> Vec<String> {
        self.game.injected()
    }

    // the lines of the mod log that start with `prefix`
    pub fn lines(&self, prefix: &str) -> Vec<String> {
        self.game.logs().into_iter().filter(|line| line.contains(prefix)).collect()
    }

    pub fn logged(&self, line: &str) -> bool {
        self.game.logs().iter().any(|logged| logged == line)
    }
}
