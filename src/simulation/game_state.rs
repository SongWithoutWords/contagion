#[derive(Clone)]
pub struct GameState {
    pub trans_wait: usize,
    pub game_paused: bool,
    pub terminate: bool,
    pub transition_game: bool,
    pub transition_menu: bool,
    pub summary_text: bool,

    /*TEXT FADE*/
    pub fade_wait: usize,
    pub fade_max: usize,
    pub fade_alpha: f32,

    /*TUTORIAL CHECK*/
    pub tutorial: bool,
    pub tut_time: usize,
    pub tut_01: bool, // spacebar to unpause
    pub tut_02: bool, // select police
    pub tut_03: bool, // Right click to issue attack or move
    /*TUTORIAL END*/

    /*DEBUG TOOL*/
    pub humans_win: bool,
    pub zombies_win: bool
}

impl GameState {
    pub fn new() -> GameState{
        GameState {
            trans_wait: 0,
            fade_wait: 0,
            fade_max: 60,
            fade_alpha: 1.0,
            game_paused: false,
            transition_game: false,
            transition_menu: false,
            terminate: false,
            humans_win: false,
            zombies_win: false,
            summary_text: true,
            tutorial: false,
            tut_time: 0,
            tut_01: false,
            tut_02: false,
            tut_03: false,
        }
    }

    pub fn new_tutorial() -> GameState {
        GameState {
            trans_wait: 0,
            fade_wait: 0,
            fade_max: 60,
            fade_alpha: 1.0,
            game_paused: false,
            transition_game: false,
            transition_menu: false,
            terminate: false,
            humans_win: false,
            zombies_win: false,
            summary_text: true,
            tutorial: true,
            tut_time: 0,
            tut_01: true,
            tut_02: true,
            tut_03: true,
        }
    }
}