use std::cell::Cell;

use alaydriem_toggles::traits::GameView;

// a stand-in for the game: the test sets whether a screen is open and whether the player lives
pub struct FakeGame {
    gameplay: Cell<bool>,
    alive: Cell<Option<bool>>,
}

impl FakeGame {
    pub fn playing() -> Self {
        Self { gameplay: Cell::new(true), alive: Cell::new(Some(true)) }
    }

    pub fn set_gameplay(&self, gameplay: bool) {
        self.gameplay.set(gameplay);
    }

    pub fn set_alive(&self, alive: Option<bool>) {
        self.alive.set(alive);
    }
}

impl GameView for FakeGame {
    fn in_gameplay(&self) -> bool {
        self.gameplay.get()
    }

    fn player_alive(&self) -> Option<bool> {
        self.alive.get()
    }
}
