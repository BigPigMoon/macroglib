pub trait Scene {
    fn update(&mut self, dt: f32);
    fn render(&self);
}
