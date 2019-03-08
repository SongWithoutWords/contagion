pub struct GameState {
    pub game_paused: bool,
    pub terminate: bool,
    pub transition_game: bool,
    pub transition_menu: bool,
}

impl GameState {
    pub fn new() -> GameState{
        GameState {
            transition_game: false,
            transition_menu: false,
            game_paused: false,
            terminate: false
        }
    }
}