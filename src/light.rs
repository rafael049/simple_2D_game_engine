extern crate nalgebra_glm as glm;

use crate::shader::ShaderProgram;

use crate::entity::Entity;

pub struct Light {
    pub pos: glm::Vec3,
    pub intensity: f32,
    pub color: glm::Vec3,
}

impl Entity for Light {
    #[allow(unused_variables)]
    fn render(&self, shader:&ShaderProgram){}

    fn update(&mut self) {}

    fn set_pos(&mut self, pos: glm::Vec2){
        self.pos.x = pos.x;
        self.pos.y = pos.y;
    }
    fn set_pos_x(&mut self, x: f32){
        self.pos.x = x;
    }
    fn set_pos_y(&mut self, y: f32){
        self.pos.y = y;
    }
    fn get_pos(&self) -> glm::Vec2 {
        glm::vec3_to_vec2(&self.pos)
    }

    fn downcast_to_light(&self) -> Light {
        Light {
            pos: self.pos,
            intensity: self.intensity,
            color: self.color,
        }
    }
}


impl Light{
    pub fn new(pos: glm::Vec3, color: glm::Vec3) -> Light {
        let intensity = 1.0;
        Light {
            pos,
            intensity,
            color,
        }
    }

    fn set_pos_z(&mut self, z: f32){
        self.pos.z = z;
    }

}



