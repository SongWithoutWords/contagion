use crate::core::vector::*;
use crate::simulation::state::State;
use crate::core::matrix::Mat4;
use crate::simulation::control::*;
use crate::core::geo::intersect::rectangle_point::*;
use crate::simulation::game_state::GameState;
use sdl2::keyboard::Keycode;

use glium_sdl2::SDL2Facade;
use sdl2::event::Event;

#[derive(Clone, PartialEq, Debug)]
pub enum GuiType {
    Selected, // Bottom Middle
    SelectionDrag, // Bottom Right
    Score,    // Top Left
    Timer,    // Top Middle
    Window,
    Menu {
        _window_gui: Box<Gui>,
        _buttons_gui: Vec<Box<Gui>>,
        text: String
    },
    Button {
        text: String
    },
    ZombieUI,
    CopUI,
    CivilianUI
}

// Implementing a callback with mut parameters is too challenging
//pub struct Button {
//    pub text: String,
//    pub on_click: fn(component: &mut Component, game_state: &mut GameState),
//}

//impl Button {
//    pub fn new(text: String, on_click: fn(component: &mut Component, game_state: &mut GameState)) -> Button {
//        Button {
//            text,
//            on_click
//        }
//    }
//}

#[derive(Clone,PartialEq, Debug)]
pub enum ActiveWindow {
    Game,
    Menu,
    Instruction,
    MainMenu
}
pub static mut CURRENT: ActiveWindow = ActiveWindow::Game;

pub struct Component {
    pub components: Vec<Gui>,
    pub active_window: ActiveWindow
}

impl Component {
    pub fn init_game_gui() -> Component {
        let selected_ui = Gui::new(GuiType::Selected, 0.1, 0.1, Vector2{x: -0.9, y: -0.9});
        let zombie_ui = Gui::new(GuiType::ZombieUI, 0.1, 0.1, Vector2{x: 0.91, y: 0.92});
        let cop_ui = Gui::new(GuiType::CopUI, 0.1, 0.1, Vector2{x: 0.61, y: 0.92});
        let civilian_ui = Gui::new(GuiType::CivilianUI, 0.1, 0.1, Vector2{x: 0.76, y: 0.92});
        let drag_ui = Gui::new(GuiType::SelectionDrag, 0.0, 0.0, Vector2{x: 0.0, y: 0.0});
        let box_ui = Gui::new(GuiType::Window, 1.8, 1.8, Vector2{x: 0.0, y: 0.0});
        let button1 = GuiType::Button{text: "Exit".to_string()};
        let button2 = GuiType::Button{text: "Main Menu".to_string()};
        let button3 = GuiType::Button{text: "Instruction".to_string()};
        let button_ui_1 = Gui::new(button1, 0.2, 0.05, Vector2{x: 0.0, y: -0.15});
        let button_ui_2 = Gui::new(button2, 0.4, 0.05, Vector2{x: 0.0, y: 0.0});
        let button_ui_3 = Gui::new(button3, 0.4, 0.05, Vector2{x: 0.0, y: 0.15});
        let menu_ui = Gui::new(GuiType::Menu{ _window_gui: Box::new(box_ui),
                                                       _buttons_gui: vec![Box::new(button_ui_3),
                                                                          Box::new(button_ui_2),
                                                                          Box::new(button_ui_1)],
                                                       text: "Setting".to_string()},
                               0.1, 0.125,
                               Vector2{x: -0.9, y: 0.9});

        Component {
            components: vec![selected_ui, drag_ui, menu_ui, cop_ui, civilian_ui, zombie_ui],
            active_window: ActiveWindow::Game
        }
    }

    pub fn init_main_menu_gui() -> Component {
        // main menu buttons
        let button_start = GuiType::Button{text: "Start".to_string()};
        let button_exit = GuiType::Button {text: "Exit".to_string()};
        let button_start_ui = Gui::new(button_start, 0.3, 0.05, Vector2{x:0.0, y: -0.2});
        let button_exit_ui = Gui::new(button_exit, 0.2, 0.05, Vector2{x: 0.0, y: -0.5});

        // box containment for main menu settings
        let box_ui = Gui::new(GuiType::Window, 1.8, 1.8, Vector2{x: 0.0, y: 0.0});

        // main menu settings button
        let button_menu_instruction = GuiType::Button{text: "Instruction".to_string()};
        let button_menu_settings = GuiType::Button{text: "Advanced (WIP)".to_string()};
        let button_menu_back = GuiType::Button{text: "Back".to_string()};
        let button_menu_instruction_ui = Gui::new(button_menu_instruction, 0.4, 0.05, Vector2{x: 0.0, y: 0.3});
        let button_menu_settings_ui = Gui::new(button_menu_settings, 0.4, 0.05, Vector2{x: 0.0, y: 0.1});
        let button_menu_back_ui = Gui::new(button_menu_back, 0.2, 0.05, Vector2{x: 0.0, y: -0.1});
        let setting_ui = Gui::new(GuiType::Menu{ _window_gui: Box::new(box_ui),
                                                          _buttons_gui: vec![Box::new(button_menu_instruction_ui),
                                                                          Box::new(button_menu_settings_ui),
                                                                          Box::new(button_menu_back_ui)],
                                                           text: "Setting".to_string()},
                                    0.4, 0.05,
                                    Vector2{x: 0.0, y: -0.35});

        // component initialization
        Component {
            components: vec![button_start_ui, button_exit_ui, setting_ui],
            active_window: ActiveWindow::MainMenu
        }
    }

    pub fn handle_event(&mut self, event: Event, window: &SDL2Facade, camera_frame: Mat4, state: &mut State, game_state: &mut GameState, control: &mut Control) {
        // handle events for any menu laid on top of game
        let mut handled_event = false;
        for i in 0..self.components.len() {
            let component = &mut self.components[i];
            match component.id {
                GuiType::Menu {ref mut _window_gui, ref mut _buttons_gui, ..} => {
                    match event {
                        Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {
                            if self.active_window == ActiveWindow::Game {
                                let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                                let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                                if check_bounding_box(top_left, bot_right, *mouse_pos) {
                                    game_state.game_paused = true;
                                    self.active_window = ActiveWindow::Menu;
                                    handled_event = true;
                                }
                            } else if self.active_window == ActiveWindow::Menu {
                                let buttons = _buttons_gui.clone();
                                let size = buttons.len();
                                for j in 0..size {
                                    let button = buttons[j].clone();
//                                        println!("{:?}", button.get_dimension());
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x: button.top_left.x, y: button.top_left.y };
                                    let bot_right = Vector2 { x: button.bot_right.x, y: button.bot_right.y };
//                                        println!("ID: {:?}", button.id);
//                                        println!("top_left: {:?}", top_left);
//                                        println!("bot_right: {:?}", bot_right);
//                                        println!("mouse position: {:?}", mouse_pos);
                                    let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
//                                        println!("check within bound: {:?}", check_within_bound);
                                    if check_within_bound {
                                        let mut display_text = "".to_string();
                                        match button.id {
                                            GuiType::Button { text } => {
                                                display_text = text;
                                            }
                                            _ => ()
                                        }
//                                            println!("{:?}", display_text);

                                        if display_text == "Instruction" {
                                            self.active_window = ActiveWindow::Instruction;
                                            handled_event = true;
                                            game_state.game_paused = true;
                                        } else if display_text == "Main Menu" {
                                            game_state.transition_menu = true;
                                        } else if display_text == "Exit" {
                                            game_state.terminate = true;
                                        }
                                    }
                                }
                                // Check if mouse position is outside the window of menu
                                let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: _window_gui.top_left.x, y: _window_gui.top_left.y };
                                let bot_right = Vector2 { x: _window_gui.bot_right.x, y: _window_gui.bot_right.y };

                                if !check_bounding_box(top_left, bot_right, *mouse_pos) {
                                    //println!("Mouse button up {}, {}, {}", top_left, bot_right, mouse_pos);
                                    self.active_window = ActiveWindow::Game;
                                    game_state.game_paused = false;
                                    handled_event = true;
                                }
                            } else if self.active_window == ActiveWindow::Instruction {
                                // Check if mouse position is outside the window of menu
                                let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: _window_gui.top_left.x, y: _window_gui.top_left.y };
                                let bot_right = Vector2 { x: _window_gui.bot_right.x, y: _window_gui.bot_right.y };

                                if !check_bounding_box(top_left, bot_right, *mouse_pos) {
                                    self.active_window = ActiveWindow::Game;
                                    game_state.game_paused = false;
                                    handled_event = true;
                                }
                            } else {
                                handled_event = false;
                            }
                        },
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            if self.active_window == ActiveWindow::Menu {
                                self.active_window = ActiveWindow::Game;
                                game_state.game_paused = false;
                                handled_event = true;
                            }
                            else if self.active_window == ActiveWindow::Instruction {
                                self.active_window = ActiveWindow::Menu;
                                handled_event = true;
                            }
                            else if self.active_window == ActiveWindow::Game {
                                self.active_window = ActiveWindow::Menu;
                                game_state.game_paused = true;
                                handled_event = true;
                            }
                        },
                        _ => {
                            handled_event = false;
                        }
                    }
                }
                _ => ()
            }
        }

        if !handled_event && self.active_window == ActiveWindow::Game {
            control.handle_event(event, &window, camera_frame, state, game_state);
        }
    }

    pub fn handle_main_menu_event(&mut self, event: &Event, window: &SDL2Facade, game_state: &mut GameState) {
        // handle events for any menu laid on top of game
        for i in 0..self.components.len() {
            let component = &self.components[i];
            match &component.id {
                GuiType::Button {text} => {
                    if self.active_window == ActiveWindow::MainMenu {
                        match event {
                            Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {
                                let mouse_pos = &mut Vector2 { x: *x as f64, y: *y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                                let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                                let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                if check_within_bound {
                                    let display_text = text;
                                    if display_text == "Start" {
                                        game_state.transition_game = true;
                                    } else if display_text == "Exit" {
                                        game_state.terminate = true;
                                    }
                                }
                            },
                            __ => ()
                        }
                    }
                },
                // Settings
                GuiType::Menu {_window_gui, _buttons_gui, ..} => {
                    match event.clone() {
                        Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {
                            if self.active_window == ActiveWindow::MainMenu {
                                let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                                let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                                if check_bounding_box(top_left, bot_right, *mouse_pos) {
                                    self.active_window = ActiveWindow::Menu;
                                }
                            } else if self.active_window == ActiveWindow::Menu {
                                let buttons = _buttons_gui.clone();
                                let size = buttons.len();
                                for j in 0..size {
                                    let button = buttons[j].clone();
//                                        println!("{:?}", button.get_dimension());
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x: button.top_left.x, y: button.top_left.y };
                                    let bot_right = Vector2 { x: button.bot_right.x, y: button.bot_right.y };
                                    let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                    if check_within_bound {
                                        let mut display_text = "".to_string();
                                        match button.id {
                                            GuiType::Button { text } => {
                                                display_text = text;
                                            }
                                            _ => ()
                                        }

                                        if display_text == "Instruction" {
                                            self.active_window = ActiveWindow::Instruction;
                                        } else if display_text == "Back" {
                                            if self.active_window == ActiveWindow::Menu {
                                                self.active_window = ActiveWindow::MainMenu;
                                            }

                                        }
                                    }
                                }
                                // Check if mouse position is outside the window of menu
                                let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: _window_gui.top_left.x, y: _window_gui.top_left.y };
                                let bot_right = Vector2 { x: _window_gui.bot_right.x, y: _window_gui.bot_right.y };

                                if !check_bounding_box(top_left, bot_right, *mouse_pos) {
                                    //println!("Mouse button up {}, {}, {}", top_left, bot_right, mouse_pos);
                                    self.active_window = ActiveWindow::MainMenu;
                                }
                            } else if self.active_window == ActiveWindow::Instruction {
                                // Check if mouse position is outside the window of menu
                                let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: _window_gui.top_left.x, y: _window_gui.top_left.y };
                                let bot_right = Vector2 { x: _window_gui.bot_right.x, y: _window_gui.bot_right.y };

                                if !check_bounding_box(top_left, bot_right, *mouse_pos) {
                                    self.active_window = ActiveWindow::MainMenu;
                                }
                            } else {
                            }
                        },
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            if self.active_window == ActiveWindow::Menu {
                                self.active_window = ActiveWindow::MainMenu;
                            }
                            else if self.active_window == ActiveWindow::Instruction {
                                self.active_window = ActiveWindow::Menu;
                            }
                            else if self.active_window == ActiveWindow::MainMenu {
                                self.active_window = ActiveWindow::Menu;
                            }
                        },
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone,PartialEq, Debug)]
pub struct Gui {
    pub id: GuiType,
    pub top_left: Vector2,
    pub top_right: Vector2,
    pub bot_left: Vector2,
    pub bot_right: Vector2,
}

// using viewport coordinate [-1,1]
impl Gui {
    // instantiates GUI
    pub fn new(_id: GuiType, w: f64, h: f64, pos: Vector2) -> Gui {
        let _x = w/2.0;
        let _y = h/2.0;
        Gui {
            id: _id,
            top_left: Vector2{x: -_x + pos.x, y: _y + pos.y},
            top_right: Vector2{x: _x + pos.x, y: _y + pos.y},
            bot_left: Vector2{x: -_x + pos.x, y: -_y + pos.y},
            bot_right: Vector2{x: _x + pos.x, y: -_y + pos.y},
        }
    }

    // move position of the GUI
    pub fn move_pos(&mut self, vec: Vector2) {
        self.top_left.x += vec.x;
        self.top_right.x += vec.x;
        self.bot_left.x += vec.x;
        self.bot_right.x += vec.x;
        self.top_left.y += vec.y;
        self.top_right.y += vec.y;
        self.bot_left.y += vec.y;
        self.bot_right.y += vec.y;
    }

    // get dimension of the user interface
    // ordered top_left, top_right, bot_left, bot_right
    pub fn get_dimension(&self) -> (Vector2, Vector2, Vector2, Vector2) {
        (self.top_left, self.top_right, self.bot_left, self.bot_right)
    }

    pub fn set_dimension(&mut self, tl: Vector2, tr: Vector2, bl: Vector2, br: Vector2) {
        self.top_left = tl;
        self.top_right = tr;
        self.bot_left = bl;
        self.bot_right = br;
    }
//
}