use crate::core::vector::*;
use crate::simulation::state::State;
use crate::core::matrix::Mat4;
use crate::simulation::control::*;
use crate::core::geo::intersect::rectangle_point::*;
use crate::simulation::game_state::GameState;
use sdl2::keyboard::Keycode;
use glium_sdl2::SDL2Facade;
use sdl2::event::Event;
use sdl2::video::FullscreenType::{Off, True};

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
        text: String,
        highlight: bool,
    },
    Button {
        text: String,
        highlight: bool,
    },
    ZombieUI,
    CopUI,
    CivilianUI
}

#[derive(Clone,PartialEq, Debug)]
pub enum ActiveWindow {
    Game,
    Menu,
    Instruction,
    MainMenu
}
pub static mut CURRENT: ActiveWindow = ActiveWindow::Game;

#[derive(Clone)]
pub struct Component {
    pub components: Vec<Gui>,
    pub active_window: ActiveWindow
}

impl Component {
    // initializer for game's GUI
    pub fn init_game_gui() -> Component {
        let selected_ui = Gui::new(GuiType::Selected, 0.1, 0.1, Vector2{x: -0.9, y: -0.9});
        let zombie_ui = Gui::new(GuiType::ZombieUI, 0.1, 0.1, Vector2{x: 0.91, y: 0.92});
        let cop_ui = Gui::new(GuiType::CopUI, 0.1, 0.1, Vector2{x: 0.61, y: 0.92});
        let civilian_ui = Gui::new(GuiType::CivilianUI, 0.1, 0.1, Vector2{x: 0.76, y: 0.92});
        let drag_ui = Gui::new(GuiType::SelectionDrag, 0.0, 0.0, Vector2{x: 0.0, y: 0.0});
        let box_ui = Gui::new(GuiType::Window, 1.8, 1.8, Vector2{x: 0.0, y: 0.0});
        let button1 = GuiType::Button{text: "Exit".to_string(), highlight: false};
        let button2 = GuiType::Button{text: "Retry".to_string(), highlight: false};
        let button3 = GuiType::Button{text: "Main Menu".to_string(), highlight: false};
        let button4 = GuiType::Button{text: "Instruction".to_string(), highlight: false};
        let button_ui_1 = Gui::new(button1, 0.17, 0.09, Vector2{x: 0.0, y: -0.225});
        let button_ui_2 = Gui::new(button2, 0.2, 0.09, Vector2{x: 0.0, y: -0.075});
        let button_ui_3 = Gui::new(button3, 0.37, 0.09, Vector2{x: 0.0, y: 0.075});
        let button_ui_4 = Gui::new(button4, 0.4, 0.09, Vector2{x: 0.0, y: 0.225});
        let menu_ui = Gui::new(GuiType::Menu{ _window_gui: Box::new(box_ui),
                                                       _buttons_gui: vec![Box::new(button_ui_4),
                                                                          Box::new(button_ui_3),
                                                                          Box::new(button_ui_2),
                                                                          Box::new(button_ui_1)],
                                                       text: "Setting".to_string(), highlight: false},
                               0.1, 0.125,
                               Vector2{x: -0.9, y: 0.9});

        Component {
            components: vec![selected_ui, drag_ui, menu_ui, cop_ui, civilian_ui, zombie_ui],
            active_window: ActiveWindow::Game
        }
    }

    // initializer for main menu's GUI
    pub fn init_main_menu_gui() -> Component {
        // main menu buttons
        let button_start = GuiType::Button{text: "Start".to_string(), highlight: false};
        let button_exit = GuiType::Button {text: "Exit".to_string(), highlight: false};
        let button_start_ui = Gui::new(button_start, 0.20, 0.12, Vector2{x:0.0, y: -0.2});
        let button_exit_ui = Gui::new(button_exit, 0.15, 0.12, Vector2{x: 0.0, y: -0.5});

        // box containment for main menu settings
        let box_ui = Gui::new(GuiType::Window, 1.8, 1.8, Vector2{x: 0.0, y: 0.0});

        // main menu settings button
        let button_menu_instruction = GuiType::Button{text: "Instruction".to_string(), highlight: false};
        let button_menu_settings = GuiType::Button{text: "Fullscreen".to_string(), highlight: false};
        let button_menu_back = GuiType::Button{text: "Back".to_string(), highlight: false};
        let button_menu_instruction_ui = Gui::new(button_menu_instruction, 0.4, 0.09, Vector2{x: 0.0, y: 0.3});
        let button_menu_settings_ui = Gui::new(button_menu_settings, 0.5, 0.09, Vector2{x: 0.0, y: 0.1});
        let button_menu_back_ui = Gui::new(button_menu_back, 0.15, 0.09, Vector2{x: 0.0, y: -0.1});
        let setting_ui = Gui::new(GuiType::Menu{ _window_gui: Box::new(box_ui),
                                                          _buttons_gui: vec![Box::new(button_menu_instruction_ui),
                                                                          Box::new(button_menu_settings_ui),
                                                                          Box::new(button_menu_back_ui)],
                                                           text: "Setting".to_string(), highlight: false},
                                    0.30, 0.12,
                                    Vector2{x: 0.0, y: -0.35});

        // component initialization
        Component {
            components: vec![button_start_ui, button_exit_ui, setting_ui],
            active_window: ActiveWindow::MainMenu
        }
    }

    // initializer for game loss scene's GUI
    pub fn init_loss_gui() -> Component {
        let button_retry = GuiType::Button{text: "Retry".to_string(), highlight: false};
        let button_main_menu = GuiType::Button{text: "Main Menu".to_string(), highlight: false};
        let button_exit = GuiType::Button {text: "Exit".to_string(), highlight: false};
        let button_retry_ui = Gui::new(button_retry, 0.2, 0.09, Vector2{x:-0.5, y: -0.5});
        let button_main_menu_ui = Gui::new(button_main_menu, 0.4, 0.09, Vector2{x:0.0, y: -0.5});
        let button_exit_ui = Gui::new(button_exit, 0.15, 0.09, Vector2{x: 0.5, y: -0.5});
        Component {
            components: vec![button_retry_ui,button_main_menu_ui,button_exit_ui],
            active_window: ActiveWindow::Game
        }
    }

    // initializer for game victory scene's GUI
    pub fn init_victory_gui() -> Component {
        let button_retry = GuiType::Button{text: "Retry".to_string(), highlight: false};
        let button_main_menu = GuiType::Button{text: "Main Menu".to_string(), highlight: false};
        let button_exit = GuiType::Button {text: "Exit".to_string(), highlight: false};
        let button_retry_ui = Gui::new(button_retry, 0.2, 0.09, Vector2{x:-0.5, y: -0.5});
        let button_main_menu_ui = Gui::new(button_main_menu, 0.4, 0.09, Vector2{x:0.0, y: -0.5});
        let button_exit_ui = Gui::new(button_exit, 0.15, 0.09, Vector2{x: 0.5, y: -0.5});
        Component {
            components: vec![button_retry_ui,button_main_menu_ui,button_exit_ui],
            active_window: ActiveWindow::Game
        }
    }

    // main game's GUI event handler
    pub fn handle_event(&mut self, event: Event, window: &mut SDL2Facade, camera_frame: Mat4, state: &mut State, game_state: &mut GameState, control: &mut Control) {
        // handle events for any menu laid on top of game
        let mut handled_event = false;
        for i in 0..self.components.len() {
            let component = &mut self.components[i];
            match &mut component.id {
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
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x: button.top_left.x, y: button.top_left.y };
                                    let bot_right = Vector2 { x: button.bot_right.x, y: button.bot_right.y };
                                    let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                    if check_within_bound {
                                        let mut display_text = "".to_string();
                                        match button.id {
                                            GuiType::Button { text, .. } => {
                                                display_text = text;
                                            }
                                            _ => ()
                                        }

                                        if display_text == "Instruction" {
                                            self.active_window = ActiveWindow::Instruction;
                                            handled_event = true;
                                            game_state.game_paused = true;
                                        } else if display_text == "Main Menu" {
                                            game_state.transition_menu = true;
                                        } else if display_text == "Retry" {
                                            game_state.transition_game = true;
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
                        Event::MouseMotion { timestamp: _, window_id: _, which: _, x, y, .. } => {
                            if self.active_window == ActiveWindow::Menu {
                                let buttons = _buttons_gui;
                                let size = buttons.clone().len();
                                for j in 0..size {
                                    let button = &mut buttons[j];
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x: button.top_left.x, y: button.top_left.y };
                                    let bot_right = Vector2 { x: button.bot_right.x, y: button.bot_right.y };
                                    let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                    match &mut button.id {
                                        GuiType::Button { text, ref mut highlight } => {
                                            if check_within_bound {
                                                let display_text = text;
                                                if display_text == "Instruction" || display_text == "Main Menu"|| display_text == "Retry"|| display_text == "Exit" {
                                                    *highlight = true;
                                                }
                                            } else {
                                                *highlight = false;
                                            }
                                        }
                                        _ => ()
                                    }
                                }
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
                },
                _ => ()
            }
        }

        if !handled_event && self.active_window == ActiveWindow::Game {
            control.handle_event(event, &window, camera_frame, state, game_state);
        }
    }

    // main menu's GUI event handler
    pub fn handle_main_menu_event(&mut self, event: &Event, window: &mut SDL2Facade, game_state: &mut GameState) {
        // handle events for any menu laid on top of game
        for i in 0..self.components.len() {
            let component = &mut self.components[i];
            match &mut component.id {
                GuiType::Button {text, ref mut highlight} => {
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
                            Event::MouseMotion { timestamp: _, window_id: _, which: _, x, y, .. } => {
                                let mouse_pos = &mut Vector2 { x: *x as f64, y: *y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                                let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                                let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                if check_within_bound {
                                    let display_text = text;
                                    if display_text == "Start" || display_text == "Exit" {
                                        *highlight = true;
                                    }
                                } else {
                                    *highlight = false;
                                }
                            },
                            __ => ()
                        }
                    }
                },
                // Settings
                GuiType::Menu {_window_gui, _buttons_gui, text, ref mut highlight} => {
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
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x: button.top_left.x, y: button.top_left.y };
                                    let bot_right = Vector2 { x: button.bot_right.x, y: button.bot_right.y };
                                    let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                    if check_within_bound {
                                        let mut display_text = "".to_string();
                                        match button.id {
                                            GuiType::Button { text, .. } => {
                                                display_text = text;
                                            }
                                            _ => ()
                                        }

                                        if display_text == "Instruction" {
                                            self.active_window = ActiveWindow::Instruction;
                                        } else if display_text == "Fullscreen" {
                                            if window.window_mut().fullscreen_state() == Off {
                                                window.window_mut().set_fullscreen(True);
                                            } else if window.window_mut().fullscreen_state() == True {
                                                window.window_mut().set_fullscreen(Off);
                                            }
                                        }
                                        else if display_text == "Back" {
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
                            }
                        },
                        Event::MouseMotion { timestamp: _, window_id: _, which: _, x, y, .. } => {
                            if self.active_window == ActiveWindow::Menu {
                                let buttons = _buttons_gui;
                                let size = buttons.clone().len();
                                for j in 0..size {

                                    let button = &mut buttons[j];
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x: button.top_left.x, y: button.top_left.y };
                                    let bot_right = Vector2 { x: button.bot_right.x, y: button.bot_right.y };
                                    let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                    match &mut button.id {
                                        GuiType::Button { text, ref mut highlight } => {
                                            if check_within_bound {
                                                let display_text = text;
                                                if display_text == "Instruction" || display_text == "Fullscreen" || display_text == "Back" {
                                                    *highlight = true;
                                                }
                                            } else {
                                                *highlight = false;
                                            }
                                        }
                                        _ => ()
                                    }
                                }
                            } else {
                                let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                translate_mouse_to_camera(mouse_pos, window.window().size());

                                let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                                let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                                let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                                if check_within_bound {
                                    let display_text = text;
                                    if display_text == "Setting" {
                                        *highlight = true;
                                    }
                                } else {
                                    *highlight = false;
                                }
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

    // loss scene's GUI event handler
    pub fn handle_loss_event(&mut self, event: &Event, window: &SDL2Facade, game_state: &mut GameState) {
        // handle events for any menu laid on top of game
        for i in 0..self.components.len() {
            let component = &mut self.components[i];
            match &mut component.id {
                GuiType::Button {text, ref mut highlight} => {
                    match event {
                        Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {
                            let mouse_pos = &mut Vector2 { x: *x as f64, y: *y as f64 };
                            translate_mouse_to_camera(mouse_pos, window.window().size());

                            let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                            let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                            let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                            if check_within_bound {
                                let display_text = text;
                                if display_text == "Retry" {
                                    game_state.transition_game = true;
                                } else if display_text == "Main Menu" {
                                    game_state.transition_menu = true;
                                } else if display_text == "Exit" {
                                    game_state.terminate = true;
                                }
                            }
                        },
                        Event::MouseMotion { timestamp: _, window_id: _, which: _, x, y, .. } => {
                            let mouse_pos = &mut Vector2 { x: *x as f64, y: *y as f64 };
                            translate_mouse_to_camera(mouse_pos, window.window().size());

                            let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                            let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                            let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                            if check_within_bound {
                                let display_text = text;
                                if display_text == "Retry" || display_text == "Main Menu" || display_text == "Exit" {
                                    *highlight = true;
                                }
                            } else {
                                *highlight = false;
                            }
                        },
                        __ => ()
                    }
                },
                _ => ()
            }
        }
    }

    // victory scene's GUI event handler
    pub fn handle_victory_event(&mut self, event: &Event, window: &mut SDL2Facade, game_state: &mut GameState) {
        // handle events for any menu laid on top of game
        for i in 0..self.components.len() {
            let component = &mut self.components[i];
            match &mut component.id {
                GuiType::Button {text, ref mut highlight} => {
                    match event {
                        Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {
                            let mouse_pos = &mut Vector2 { x: *x as f64, y: *y as f64 };
                            translate_mouse_to_camera(mouse_pos, window.window().size());

                            let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                            let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                            let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                            if check_within_bound {
                                let display_text = text;
                                if display_text == "Retry" {
                                    game_state.transition_game = true;
                                } else if display_text == "Main Menu" {
                                    game_state.transition_menu = true;
                                } else if display_text == "Exit" {
                                    game_state.terminate = true;
                                }
                            }
                        },
                        Event::MouseMotion { timestamp: _, window_id: _, which: _, x, y, .. } => {
                            let mouse_pos = &mut Vector2 { x: *x as f64, y: *y as f64 };
                            translate_mouse_to_camera(mouse_pos, window.window().size());

                            let top_left = Vector2 { x: component.top_left.x, y: component.top_left.y };
                            let bot_right = Vector2 { x: component.bot_right.x, y: component.bot_right.y };
                            let check_within_bound = check_bounding_box(top_left, bot_right, *mouse_pos);
                            if check_within_bound {
                                let display_text = text;
                                if display_text == "Retry" || display_text == "Main Menu" || display_text == "Exit" {
                                    *highlight = true;
                                }
                            } else {
                                *highlight = false;
                            }
                        },
                        __ => ()
                    }
                },
                _ => ()
            }
        }
    }
}

// GUI structure
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

    // set dimension of GUI
    pub fn set_dimension(&mut self, tl: Vector2, tr: Vector2, bl: Vector2, br: Vector2) {
        self.top_left = tl;
        self.top_right = tr;
        self.bot_left = bl;
        self.bot_right = br;
    }
}