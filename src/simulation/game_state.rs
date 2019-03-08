#[derive(Clone)]
pub struct GameState {
    pub game_paused: bool,
    pub terminate: bool,
    pub transition_game: bool,
    pub transition_menu: bool,
    // debug purposes for scene transition
    pub humans_win: bool,
    pub zombies_win: bool
}

impl GameState {
    pub fn new() -> GameState{
        GameState {
            transition_game: false,
            transition_menu: false,
            game_paused: false,
            terminate: false,
            humans_win: false,
            zombies_win: false,
        }
    }
}