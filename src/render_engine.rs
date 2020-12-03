extern crate glfw;
extern crate gl;
extern crate nalgebra_glm as glm;

use crate::resources::CharacterSet;
use glfw::Context;

use crate::scene::Scene;
use crate::shader::ShaderProgram;
use crate::resources::Resources;
use crate::editor::Editor;
use crate::ui::UI;
use crate::camera::Camera;

pub struct RenderEngine {
    screen_width: i32,
    screen_height: i32,
    back_color: glm::Vec3,
}


impl RenderEngine {
    pub fn new(screen_width: i32, screen_height: i32 ) -> RenderEngine {

        RenderEngine {
            screen_width,
            screen_height,
            back_color: glm::vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn init(&self) {
        unsafe {
            gl::Viewport(0, 0, self.screen_width as i32, self.screen_height as i32);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::ClearColor(self.back_color.x, self.back_color.y, self.back_color.z, 1.0);
        }
    }

    pub fn render_scene(&self, scene: &Scene, shader: &ShaderProgram) {
        shader.set_view_matrix(&scene.camera.get_view_projection());
        scene.render(shader);
    }

    pub fn render_ui(&self, ui:&UI, ui_shader:&ShaderProgram, text_shader:&ShaderProgram, character_set: &CharacterSet){
        ui.render(ui_shader, text_shader, character_set);
    }

    pub fn render_all(&self, window: &mut glfw::Window, scene: &Scene, ui: &UI, resources: &Resources) {
        let main_shader = resources.get_main_shader();
        let ui_shader = resources.get_ui_shader();
        let text_shader = resources.get_text_shader();

        self.clear();

        self.render_scene(scene, &main_shader.borrow());
        self.render_ui(ui, ui_shader, text_shader, resources.get_character_set());

        window.swap_buffers();
    }

    pub fn set_clear_color(&mut self, color: glm::Vec3) {
        self.back_color = color;
        unsafe {
            gl::ClearColor(self.back_color.x, self.back_color.y, self.back_color.z, 1.0);
        }
    }

    fn clear(&self) {
        unsafe{
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

}
