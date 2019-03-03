use crate::presentation::{ui::gui, camera};
use crate::simulation::state::State;
use crate::presentation::ui::gui::Component;
use crate::presentation::camera::Camera;
use crate::simulation::control::Control;

use crate::simulation::initial_state::initial_state;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SceneType {
    StartMenu,
    InGame,
    None,
}

pub static mut CURRENT_SCENE: SceneType = SceneType::InGame;
pub static mut PREV_SCENE: SceneType = SceneType::None;

pub trait Scene {
    fn handle_input(&self);
    fn update(&self) -> Option<Box<Scene>>;
    fn render(&self);
}

///*Start Menu*/
//pub struct StartMenu {
//    gui: Component,
//}
//
//impl Scene for StartMenu {
//    fn handle_input(&self){}
//    fn update(&self) -> Box<Scene> {
//        None
//    }
//    fn render(&self){}
//}

/*In Game*/
pub struct InGame {
    pub state: Option<State>,
    pub gui: Option<Component>,
    pub control: Option<Control>,
    pub camera: Option<Camera>,
}

impl InGame {
    pub fn new() -> InGame {
        InGame {
            state: None,
            gui: None,
            control: None,
            camera: None,
        }
    }
}

impl Scene for InGame {
    fn handle_input(&self){}
    fn update(&self) -> Option<Box<Scene>> {
        unsafe {
            if PREV_SCENE != CURRENT_SCENE {
                PREV_SCENE = SceneType::InGame;
                Some(Box::new(InGame {
                    state: Some(initial_state(100, rand::random::<u32>())),
                    gui: Some(Component::init_demo()),
                    control: Some(Control::new()),
                    camera: Some(Camera::new()),
                }))
            }
            else {
                None
            }
        }
    }
    fn render(&self){}
}

pub fn update_scene(x: &Scene) -> Option<Box<Scene>>{
    x.update()
}