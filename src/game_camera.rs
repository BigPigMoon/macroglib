use macroquad::prelude::*;

use crate::entity::Entity;

pub struct GameCamera {
    camera: Camera2D,
}

impl GameCamera {
    pub fn new(scale: f32) -> Self {
        let zoom = vec2(scale, scale * (screen_width() / screen_height()));
        Self {
            camera: Camera2D {
                zoom,
                ..Default::default()
            },
        }
    }

    pub fn get_position(&self) -> Vec2 {
        vec2(
            self.camera.offset.x / self.camera.zoom.x,
            self.camera.offset.y / self.camera.zoom.y,
        )
    }

    pub fn set_camera(&self) {
        set_camera(&self.camera);
    }
}

impl Entity for GameCamera {
    fn move_to(&mut self, pos: Vec2) {
        self.camera.offset = vec2(pos.x * self.camera.zoom.x, pos.y * self.camera.zoom.y);
    }

    fn translate(&mut self, pos: Vec2) {
        self.camera.offset += vec2(pos.x * self.camera.zoom.x, pos.y * self.camera.zoom.y);
    }
}
