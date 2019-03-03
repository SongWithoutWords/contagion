use crate::presentation::{ui::gui, camera};
use crate::simulation::state::State;
use crate::presentation::ui::gui::Component;
use crate::presentation::camera::Camera;
use crate::simulation::initial_state::initial_state;
use crate::simulation::control::Control;

#[derive(Clone,PartialEq, Debug)]
pub enum Scene {
    StartMenu,
    Game,
    Menu,
    Instruction,
    None,
}

pub struct SceneManager {
    pub prev: Scene,
    pub next: Scene,
    pub ui: Option<Component>,
    pub state: Option<State>,
    pub camera: Option<Camera>,
    pub control: Option<Control>
}

impl SceneManager {
    // initialize scene_manager with no scenes
    pub fn init() -> SceneManager {
        SceneManager{
            prev: Scene::None,
            next: Scene::None,
            ui: None,
            state: None,
            camera: None,
            control: None,
        }
    }

    pub fn update(&mut self) {
        match self.next {
            Scene::StartMenu => {},
            Scene::Game => {
                self.state = Some(initial_state(100,rand::random::<u32>()));
                self.ui = Some(Component::init_demo());
                self.camera = Some(Camera::new());
                self.control = Some(Control::new());
                self.next = Scene::None;
            },
            Scene::Instruction => {},
            Scene::Menu => {},
            Scene::None => {},
        }
    }

    pub fn next(&mut self, next_scene: Scene) {
        self.next = next_scene;
    }
}
