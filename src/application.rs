use macroquad::prelude::*;

use crate::scene::Scene;

pub struct App {
    current_scene: Option<Box<dyn Scene>>,
}

impl App {
    pub fn new() -> Self {
        App {
            current_scene: None,
        }
    }

    pub fn set_scene<S: Scene + 'static>(&mut self, scene: S) {
        self.current_scene = Some(Box::new(scene));
    }

    pub fn update(&mut self) {
        if let Some(ref mut scene) = self.current_scene {
            scene.update(get_frame_time());
        }
    }

    pub fn render(&self) {
        if let Some(ref scene) = self.current_scene {
            scene.render();
        }
    }
}
