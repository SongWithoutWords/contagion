pub struct GameState {
    pub game_paused: bool,
    pub terminate: bool
}

impl GameState {
    pub fn new() -> GameState{
        GameState {
            game_paused: false,
            terminate: false
        }
    }
}