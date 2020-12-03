extern crate nalgebra_glm as glm;
extern crate glfw;
use crate::input;

use glfw::Key;

pub const WIDTH:          i32 = (640 as f32 * 2.0) as i32;
pub const HEIGHT:         i32 = (360 as f32 * 2.0) as i32;

pub const NATURAL_WIDTH: f32 = 320 as f32;
pub const NATURAL_HEIGHT: f32 = 180  as f32;

pub struct Camera {
    pos:  glm::Vec2,
    zoom: f32,
    width: i32,
    height: i32,
    ortho: glm::Mat4,

    pub move_sensivity: f32,
    pub zoom_sensivity: f32,
}


impl Camera{
    pub fn new(pos: glm::Vec2, zoom: f32, width: i32, height: i32) -> Camera {
        let left   = -NATURAL_WIDTH as f32;
        let right  = NATURAL_WIDTH as f32;
        let bottom = -NATURAL_HEIGHT as f32;
        let top    = NATURAL_HEIGHT as f32;
        let near   = 0.0;
        let far    = 1.0;

        let ortho = glm::ortho(left, right, bottom, top, near, far);

        let move_sensivity = 5.0;
        let zoom_sensivity = 0.01;

        Camera{pos, zoom, width, height, ortho, move_sensivity, zoom_sensivity}
    }

    pub fn get_view_projection(&self) -> glm::Mat4 {
        let scaled = glm::scale(&self.ortho, &glm::vec3(self.zoom as f32 ,self.zoom as f32 ,1.0));
        let view  = glm::translate(&scaled, &glm::vec3(-self.pos.x, -self.pos.y, 0.0));

        view
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_pos(&self) -> glm::Vec2 {
        self.pos
    }

    pub fn update(&mut self, input: &input::Input){
        if input.get_key(Key::H){
            self.pos.x -= self.move_sensivity;
        }
        if input.get_key(Key::J){
            self.pos.y -= self.move_sensivity;
        }
        if input.get_key(Key::K){
            self.pos.y += self.move_sensivity;
        }
        if input.get_key(Key::L){
            self.pos.x += self.move_sensivity;
        }

        if input.get_key(Key::Z){
            self.zoom += self.zoom_sensivity;
        }
        if input.get_key(Key::X){
            self.zoom -= self.zoom_sensivity;
        }
    }
}
