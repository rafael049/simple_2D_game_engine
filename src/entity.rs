extern crate gl;
extern crate nalgebra_glm as glm;

use crate::input::Input;
use crate::shader::ShaderProgram;
use crate::light::Light;


pub trait Entity {

    fn render(&self, shader: &ShaderProgram);

    fn update(&mut self);

    #[allow(unused_variables)]
    fn process_input(&mut self, input: &Input){
        
    }

    fn set_pos(&mut self, pos: glm::Vec2);

    #[allow(unused_variables)]
    fn set_pos_x(&mut self, x: f32){
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn set_pos_y(&mut self, y: f32){
        unimplemented!();
    }

    fn get_pos(&self) -> glm::Vec2;

    #[allow(unused_variables)]
    fn set_outline(&mut self, state: bool) {
    }


    fn downcast_to_light(&self) -> Light {
        unimplemented!();
    }
}
