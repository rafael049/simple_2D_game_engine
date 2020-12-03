extern crate gl;
extern crate nalgebra_glm as glm;

use crate::shader::ShaderProgram;
use crate::light::Light;


pub trait Entity {

    fn render(&self, shader: &ShaderProgram);
    fn update(&mut self);

    fn set_pos(&mut self, pos: glm::Vec2);
    fn set_pos_x(&mut self, x: f32);
    fn set_pos_y(&mut self, y: f32);

    fn get_pos(&self) -> glm::Vec2;

    fn set_outline(&mut self, state: bool) {
    }


    fn downcast_to_light(&self) -> Light {
        unimplemented!();
    }
}
