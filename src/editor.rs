extern crate glfw;
extern crate gl;
extern crate nalgebra_glm as glm;

use glfw::Key;

use crate::object::Object;
use crate::resources::Resources;
use crate::input::Input;
use crate::entity::Entity;
use crate::scene::EntityCollection;
use crate::shader::ShaderProgram;
use crate::{camera, camera::Camera};
use crate::scene::Scene;
use crate::ui_object::UIObject;
use crate::text::Text;


pub struct Editor {
    pub index: usize,

    cursor: Object,
    text: Text,
}


impl Editor {
    pub fn new(resources: &mut Resources) -> Editor {
        Editor{
            index: 0,
            cursor: Object::new("cursor", "cursor", "default", resources),
	    text: Text::new("Teste", glm::vec2(10.0, 10.0), 2.0, glm::vec3(1.0, 1.0, 1.0)),
        }
    }


    pub fn control_pos(&mut self, input: &Input, shader: &ShaderProgram,  scene: &mut Scene) {
        {
        let speed = 10.0;
        let mut entitys = &mut scene.entity_collection;
        let entitys_len = entitys.len();
        self.index = self.index % entitys_len;

        let mut ent:&mut Box<dyn Entity>;

        if self.index < entitys.lights.len() {
            ent = &mut entitys.lights[self.index];
        }
        else if self.index < entitys.lights.len() + entitys.objects.len() {
            ent = &mut entitys.objects[self.index - entitys.lights.len()];
        }
        else {
            ent = &mut entitys.player;
        }


        if input.get_key(Key::W) {
            ent.set_pos_y(ent.get_pos().y + speed);
        }
        if input.get_key(Key::S) {
            ent.set_pos_y(ent.get_pos().y - speed);
        }
        if input.get_key(Key::D) {
            ent.set_pos_x(ent.get_pos().x + speed);
        }
        if input.get_key(Key::A) {
            ent.set_pos_x(ent.get_pos().x - speed);
        }

        if input.get_on_key_down(Key::Tab) {
            if input.get_key(Key::LeftShift){
                if self.index == 0 {
                    self.index = entitys_len - 1;
                }
                else {
                    self.index -= 1;
                }
            }
            else {
                self.index += 1;
            }

        }

        }
        let camera = scene.get_camera();
        let entitys = &scene.entity_collection;

        let mut ent:&Box<dyn Entity>;

        if self.index < entitys.lights.len() {
            ent = &entitys.lights[self.index];
        }
        else if self.index < entitys.lights.len() + entitys.objects.len() {
            ent = &entitys.objects[self.index - entitys.lights.len()];
        }
        else {
            ent = &entitys.player;
        }

        self.render_current_cursor(shader, camera, ent);
    }

    pub fn render_current_cursor(&mut self, shader: &ShaderProgram, camera: &Camera,  cur_entity: &Box<dyn Entity>){
        let view_pos = (cur_entity.get_pos() - camera.get_pos() ) * camera.get_zoom();
        self.cursor.set_pos(view_pos);
        self.cursor.render(shader);
        self.cursor.update();

    }

    pub fn render_interface(&self, ui_shader: &ShaderProgram, camera: &Camera) {
        let left   = -camera::NATURAL_WIDTH as f32;
        let right  = camera::NATURAL_WIDTH as f32;
        let bottom = -camera::NATURAL_HEIGHT as f32;
        let top    = camera::NATURAL_HEIGHT as f32;
        let near   = 0.0;
        let far    = 1.0;

        let ortho = glm::ortho(left, right, bottom, top, near, far);
        ui_shader.set_view_matrix(&ortho);


    }
}
