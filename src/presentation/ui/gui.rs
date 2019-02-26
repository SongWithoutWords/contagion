use crate::core::vector::*;
use crate::simulation::state::State;
use crate::core::matrix::Mat4;
use crate::simulation::control::*;
use crate::core::geo::intersect::rectangle_point::*;
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
        active: bool
    },
    Button {
        text: String
    }
}

#[derive(Clone,PartialEq, Debug)]
pub enum ActiveWindow {
    Game,
    Menu,
    Instruction,
}
pub static mut CURRENT: ActiveWindow = ActiveWindow::Game;

pub struct Component {
    pub components: Vec<Gui>
}

impl Component {
    pub fn init_demo() -> Component {
        let selected_ui = Gui::new(GuiType::Selected, 0.1, 0.1, Vector2{x: -0.9, y: -0.9});
        let drag_ui = Gui::new(GuiType::SelectionDrag, 0.0, 0.0, Vector2{x: 0.0, y: 0.0});
        let box_ui = Gui::new(GuiType::Window, 1.8, 1.8, Vector2{x: 0.0, y: 0.0});
        let button1 = Gui::new(GuiType::Button{text: "Exit".to_string()}, 0.2, 0.05, Vector2{x: 0.0, y: -0.1});
        let button2 = Gui::new(GuiType::Button{text: "Instruction".to_string()}, 0.4, 0.05, Vector2{x: 0.0, y: 0.1});
        let menu_ui = Gui::new(GuiType::Menu{ _window_gui: Box::new(box_ui), _buttons_gui: vec![Box::new(button2), Box::new(button1)], active: false}, 0.1, 0.125, Vector2{x: -0.9, y: 0.9});
//        let menu_ui = Gui::new(GuiType::Menu{ _window_gui: Box::new(box_ui), _buttons_gui: vec![], active: false}, 0.1, 0.125, Vector2{x: -0.9, y: 0.9});
        unsafe {
            CURRENT = ActiveWindow::Game;
        }

        Component {
            components: vec![selected_ui, drag_ui, menu_ui]
        }
    }

    pub fn handle_event(&mut self, event: Event, control: &mut Control, window: &SDL2Facade, camera_frame: Mat4, state: &mut State, game_paused: &mut bool, terminate: &mut bool) {
        // handle events for any menu laid on top of game
        for i in 0..self.components.len() {
            let component = &mut self.components[i];
            match component.id {
                GuiType::Menu {ref mut _window_gui, ref mut _buttons_gui, ref mut active} => {
                    match event {
                        Event::MouseButtonUp { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {
                            unsafe {
                                if CURRENT == ActiveWindow::Game {
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x : component.top_left.x, y: component.top_left.y};
                                    let bot_right = Vector2 { x : component.bot_right.x, y: component.bot_right.y};
                                    if check_bounding_box(top_left, bot_right, *mouse_pos) {
                                        *active = !*active;
                                        *game_paused = !*game_paused;
                                        CURRENT = ActiveWindow::Menu;
                                    }
                                }
                                else if CURRENT == ActiveWindow::Menu {
                                    let buttons = _buttons_gui.clone();
                                    let size = buttons.len();
                                    for j in 0..size{
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
                                        if  check_within_bound {
                                            let mut display_text = "".to_string();
                                            match button.id {
                                                GuiType::Button {text} => {
                                                    display_text = text;
                                                }
                                                _ => ()
                                            }
//                                            println!("{:?}", display_text);

                                            if display_text == "Instruction" {
//                                                println!("{:?}", button.id.clone());
                                                CURRENT = ActiveWindow::Instruction;
                                                *game_paused = true;
                                            } else if display_text == "Exit" {
//                                                println!("{:?}", button.id.clone());
                                                *terminate = true;
                                            }
                                        }
                                    }
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x : _window_gui.top_left.x, y: _window_gui.top_left.y};
                                    let bot_right = Vector2 { x : _window_gui.bot_right.x, y: _window_gui.bot_right.y};

                                    if !check_bounding_box(top_left, bot_right, *mouse_pos) {
                                        //println!("Mouse button up {}, {}, {}", top_left, bot_right, mouse_pos);
                                        *active = false;
                                        CURRENT = ActiveWindow::Game;
                                        *game_paused = false;
                                    }
                                }
                                else if CURRENT == ActiveWindow::Instruction {
                                    let mouse_pos = &mut Vector2 { x: x as f64, y: y as f64 };
                                    translate_mouse_to_camera(mouse_pos, window.window().size());

                                    let top_left = Vector2 { x : _window_gui.top_left.x, y: _window_gui.top_left.y};
                                    let bot_right = Vector2 { x : _window_gui.bot_right.x, y: _window_gui.bot_right.y};

                                    if !check_bounding_box(top_left, bot_right, *mouse_pos) {
                                        //println!("Mouse button up {}, {}, {}", top_left, bot_right, mouse_pos);
                                        *active = false;
                                        CURRENT = ActiveWindow::Game;
                                        *game_paused = false;
                                    }
                                }
                            }
                        },
                        _ => ()
                    }
                }
                _ => ()
            }
        }

        // if there are no active menu, handle events for control
        if !check_active_menu(&mut self.components) {
            control.handle_event(event, window, camera_frame, state);
            return;
        }

        // TODO: handle events for any active menu
//        match event {
//            Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {}
//            Event::MouseMotion {
//                timestamp: _,
//                window_id: _,
//                which: _,
//                mousestate: _,
//                x,
//                y,
//                xrel: _,
//                yrel: _, } => {}
//            Event::MouseButtonUp { timestamp: _, window_id: _, which: _, mouse_btn, x, y } => {}
//            _ => ()
//        }
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

fn check_active_menu(components: &mut Vec<Gui>) -> bool {
    for i in 0..components.len() {
        match components[i].id {
            GuiType::Menu { _window_gui: _, _buttons_gui: _, ref mut active} => {
                if *active { return true; }
            }
            _ => ()
        }
    }
    return false;
}
