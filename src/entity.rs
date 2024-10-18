use macroquad::prelude::*;

pub trait Entity {
    fn move_to(&mut self, pos: Vec2);
    fn translate(&mut self, pos: Vec2);
}
