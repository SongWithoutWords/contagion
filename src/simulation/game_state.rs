pub struct GameState {
    pub game_paused: bool,
    pub terminate: bool,
    pub start: bool,
}

impl GameState {
    pub fn new() -> GameState{
        GameState {
            start: false,
            game_paused: false,
            terminate: false
        }
    }
}