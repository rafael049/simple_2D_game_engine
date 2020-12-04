use glfw::{Context};

use crate::input;
use crate::resources;
use crate::camera;
use crate::scene;
use crate::text;
use crate::render_engine;
use crate::ui;


pub struct Engine {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub render_engine: render_engine::RenderEngine,
    pub input: input::Input,
    pub scene: scene::Scene,
    pub ui: ui::UI,
}


impl Engine {
    pub fn new() -> Engine{
        let screen_width  = camera::WIDTH as u32;
        let screen_height = camera::HEIGHT as u32;

        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) =
            glfw.create_window(screen_width, screen_height, "OpenGL 2D", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();
        load_gl_symbol();

         // Render engine
        let mut render_engine = render_engine::RenderEngine::new(screen_width as i32, screen_height as i32);
        render_engine.init(); 

        // Input
        let input = input::Input::new(events);



        let camera = camera::Camera::new(glm::vec2(0.0, 0.0), 1.0, screen_width as i32, screen_height as i32);





        let scene = scene::Scene::new(camera);

        let mut ui = ui::UI::new();


        render_engine.set_clear_color(glm::vec3(0.2, 0.2, 0.2));

        return Engine {
            glfw,
            window,
            render_engine,
            input,
            scene,
            ui,
        }

    }

    pub fn run(mut self, resources: &resources::Resources) {
        while !self.window.should_close() {
            self.glfw.poll_events();
            self.input.update(&mut self.glfw);

            resources.reload_shader();

            self.scene.update(&self.input);
            self.render_engine.render_all(&mut self.window, &self.scene, &self.ui,  &resources);

        }

    }
}

fn load_gl_symbol() {
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
}
