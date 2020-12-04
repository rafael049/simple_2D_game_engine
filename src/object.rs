extern crate gl;
extern crate nalgebra_glm as glm;

use crate::mesh;
use crate::shader;
use crate::resources;
use crate::texture;

use crate::entity::Entity;


pub struct Object {
    pub name: String,
    pub pos: glm::Vec2,
    pub scale: glm::Vec2,
    pub model_mat: glm::Mat4,

    pub mesh: mesh::Mesh,
    pub texture_difuse: texture::Texture,
    pub texture_normal: texture::Texture,

    pub outline: bool,

    last_pos: glm::Vec2,
}

impl Entity for Object {
    fn render(&self, shader: &shader::ShaderProgram){
        shader.set_uniform_mat4("model", &self.model_mat);
        self.mesh.render(&shader, &self.texture_difuse, Some(&self.texture_normal));
    }

    fn update(&mut self){
        if glm::distance2(&self.pos, &self.last_pos) > 0.0000001 {
            let trans_vec = glm::vec4(self.pos.x, self.pos.y, 0.0, 1.0);
            self.model_mat = glm::set_column(&self.model_mat, 3, &trans_vec);
            self.last_pos = self.pos;
        }
    }

    fn set_pos(&mut self, pos: glm::Vec2){
        self.pos = pos;
    }
    fn set_pos_x(&mut self, x: f32){
        self.pos.x = x;
    }
    fn set_pos_y(&mut self, y: f32){
        self.pos.y = y;
    }

    fn get_pos(&self) -> glm::Vec2{
        self.pos
    }

    fn set_outline(&mut self, state: bool) {
        self.outline = state;
    }
}


impl Object {
    pub fn new(name: &str, texture_name: &str) -> Object {

        let difuse_filename = format!("{}_difuse.png", texture_name);
        let normal_filename = format!("{}_normal.png", texture_name);

        let mut image_difuse = resources::load_image(&difuse_filename);
        let mut image_normal = resources::load_image(&normal_filename);

        let name = name.to_string();
        let pos = glm::vec2(0.0, 0.0);
        let scale = glm::vec2(image_difuse.width as f32, image_difuse.height as f32);
        let model_mat = glm::translate(&glm::scaling(&glm::vec2_to_vec3(&scale)), &glm::vec3(pos.x, pos.y, 0.0));
        let mesh = mesh::Mesh::new();
        let texture_difuse  = texture::Texture::new(image_difuse.data.as_mut_ptr(), image_difuse.width, image_difuse.height);
        let texture_normal  = texture::Texture::new(image_normal.data.as_mut_ptr(), image_normal.width, image_normal.height);
        let outline = false;
        let last_pos = pos.clone();

        Object{name, pos, scale, model_mat, mesh, texture_difuse, texture_normal, outline, last_pos}
    }

}
