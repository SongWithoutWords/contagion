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
    pub fade_pers: f32,

    /*TUTORIAL CHECK*/
    pub tutorial: bool,
    pub tut_time: usize,
    pub tut_time_curr: usize,
    pub tut_passed: bool,
    pub tut_01: bool, // select police
    pub tut_02: bool, // Right click to issue attack or move
    pub tut_03: bool, // pause
    /*TUTORIAL END*/

    /*DEBUG TOOL*/
    pub humans_win: bool,
    pub zombies_win: bool,

    /* GAME DIFFICULTY SELECTION */
    pub difficulty: bool,
    pub easy_game: bool,
    pub medium_game: bool,
    pub hard_game: bool,

    pub easy: bool,
    pub medium: bool,
    pub hard: bool,
}

impl GameState {
    pub fn new() -> GameState{
        GameState {
            trans_wait: 0,
            fade_wait: 0,
            fade_max: 60,
            fade_alpha: 1.0,
            fade_pers: 1.0,
            game_paused: false,
            transition_game: false,
            transition_menu: false,
            terminate: false,
            humans_win: false,
            zombies_win: false,
            summary_text: true,
            tutorial: false,
            tut_time: 0,
            tut_time_curr: 0,
            tut_passed: false,
            tut_01: false,
            tut_02: false,
            tut_03: false,
            difficulty: false,
            easy_game: false,
            medium_game: false,
            hard_game: false,
            easy: false,
            medium: false,
            hard: false,
        }
    }

    pub fn new_difficulty(easy_state: bool, medium_state: bool, hard_state: bool) -> GameState{
        GameState {
            trans_wait: 0,
            fade_wait: 0,
            fade_max: 60,
            fade_alpha: 1.0,
            fade_pers: 1.0,
            game_paused: false,
            transition_game: false,
            transition_menu: false,
            terminate: false,
            humans_win: false,
            zombies_win: false,
            summary_text: true,
            tutorial: false,
            tut_time: 0,
            tut_time_curr: 0,
            tut_passed: false,
            tut_01: false,
            tut_02: false,
            tut_03: false,
            difficulty: false,
            easy_game: false,
            medium_game: false,
            hard_game: false,
            easy: easy_state,
            medium: medium_state,
            hard: hard_state,
        }
    }

    pub fn new_tutorial() -> GameState {
        GameState {
            trans_wait: 0,
            fade_wait: 0,
            fade_max: 60,
            fade_alpha: 1.0,
            fade_pers: 1.0,
            game_paused: false,
            transition_game: false,
            transition_menu: false,
            terminate: false,
            humans_win: false,
            zombies_win: false,
            summary_text: true,
            tutorial: true,
            tut_time: 0,
            tut_time_curr: 0,
            tut_passed: false,
            tut_01: true,
            tut_02: true,
            tut_03: true,
            difficulty: false,
            easy_game: false,
            medium_game: false,
            hard_game: false,
            easy: false,
            medium: false,
            hard: false,
        }
    }
}