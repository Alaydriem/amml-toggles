// what one tick reads from the game
pub trait GameView {
    // the player controls the game: no menu, chat or other screen is open
    fn in_gameplay(&self) -> bool;

    // whether the local player is alive; None without a player or without the entries to ask
    fn player_alive(&self) -> Option<bool>;
}
