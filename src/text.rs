extern crate freetype;
extern crate nalgebra_glm as glm;

use std::mem;
use gl::types::{GLvoid, GLsizeiptr, GLsizei};

use crate::resources::CharacterSet;
use crate::shader::ShaderProgram;
use crate::text_mesh::TextMesh;



pub struct Text {
    pub pos: glm::Vec2,
    pub scale: f32,
    pub string: String,
    pub color: glm::Vec3,

    pub mesh: TextMesh,
}



impl Text {
    pub fn new(string: &str, pos: glm::Vec2, scale: f32, color: glm::Vec3) -> Text {
        let string = String::from(string);
        
	let mesh = TextMesh::new(&string);

        Text {
            pos,
            scale,
            string,
            color,
	    mesh,
        }
    }

    pub fn render(&self, shader: &ShaderProgram, projection: &glm::Mat4, characters: &CharacterSet) {
        shader.set_used();

	/*let projection = glm::mat4(1.0, 0.0, 0.0, 0.0,
	                           0.0, 1.0, 0.0, 0.0,
	                           0.0, 0.0, 1.0, 0.0,
	                           0.0, 0.0, 0.0, 1.0);
        */
        shader.set_uniform_mat4("projection", &projection);
        shader.set_uniform_vec3("textColor", &self.color);

	self.mesh.render(&self.pos, self.scale, &self.string, &characters);
	println!("rendering text!");



    }

    pub fn set_text(&mut self, text: &str) {
        self.string = String::from(text);
    }
}
