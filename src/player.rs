extern crate gl;
extern crate nalgebra_glm as glm;

use glfw::Key;

use crate::object;
use crate::shader;
use crate::input;

use crate::entity::Entity;


pub struct Player {
    pub obj: object::Object,
    pub speed: f32,
}

impl Entity for Player {

    fn render(&self, shader: &shader::ShaderProgram) {
        self.obj.render(&shader);
    }

    fn update(&mut self) {
        self.obj.update();
    }

    fn set_pos(&mut self, pos: glm::Vec2){
        self.obj.pos = pos;
    }
    fn set_pos_x(&mut self, x: f32){
        self.obj.pos.x = x;
    }
    fn set_pos_y(&mut self, y: f32){
        self.obj.pos.y = y;
    }

    fn get_pos(&self) -> glm::Vec2{
        self.obj.pos
    }
}


impl Player {
    pub fn new(obj: object::Object) -> Player {
        let speed = 10.05;
        Player{obj, speed}
    }

    pub fn update_input(&mut self, input: &input::Input) {
        self.handle_input(&input);
    }


    fn handle_input(&mut self, input: &input::Input){
        if input.get_key(Key::W) {
            self.obj.pos.y += self.speed;
        }
        if input.get_key(Key::S) {
            self.obj.pos.y -= self.speed;
        }
        if input.get_key(Key::D) {
            self.obj.pos.x += self.speed;
        }
        if input.get_key(Key::A) {
            self.obj.pos.x -= self.speed;
        }
    }
}
