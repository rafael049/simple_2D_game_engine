extern crate glfw;
extern crate gl_loader;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate freetype;



pub mod input;
pub mod resources;
pub mod shader;
pub mod mesh;
pub mod texture;
pub mod camera;
pub mod object;
pub mod player;
pub mod scene;
pub mod light;
pub mod text;
pub mod text_mesh;
pub mod entity;
pub mod ui_object;
pub mod ui_entity;
pub mod render_engine;
pub mod ui;
pub mod engine;

use crate::entity::Entity;

struct Teste {
    pub obj: object::Object,
}

impl entity::Entity for Teste {
    fn render(&self, shader: &shader::ShaderProgram){
        self.obj.render(shader);
    }

    fn update(&mut self){
        self.obj.update();
    }

    fn process_input(&mut self, input: &input::Input){
        if input.get_key(glfw::Key::W){
            self.obj.pos.y += 2.0;
        }
        if input.get_key(glfw::Key::S){
            self.obj.pos.y -= 2.0;
        }
        if input.get_key(glfw::Key::D){
            self.obj.pos.x += 2.0;
        }
        if input.get_key(glfw::Key::A){
            self.obj.pos.x -= 2.0;
        }
    }

    fn set_pos(&mut self, pos:glm::Vec2){
        self.obj.set_pos(pos);
    }

    fn get_pos(&self) -> glm::Vec2{
        self.obj.get_pos()
    }
}


fn main() {

    let mut engine = engine::Engine::new();

    // Resources
    let resources = resources::Resources::new();


    let light0 = light::Light::new(glm::vec3(100.0, 100.0, 200.0) , glm::vec3(1.0, 1.0, 1.0));
    let light1 = light::Light::new(glm::vec3(100.0, 100.0, 75.0) , glm::vec3(1.0, 1.0, 1.0));
    let light2 = light::Light::new(glm::vec3(-200.0, 300.0, 100.0), glm::vec3(1.0, 0.0, 0.0));
    let light3 = light::Light::new(glm::vec3(-100.0, 400.0, 100.0), glm::vec3(0.0, 1.0, 0.0));

    let background = object::Object::new("background", "background1");
    let obj1 = object::Object::new("sphere", "brick");

    let mut abelha = Teste{obj: object::Object::new("bee", "bee")};
    abelha.set_pos(glm::vec2(100.0, 50.0));

    engine.scene.push_entity(background);
    engine.scene.push_entity(abelha);
    engine.scene.push_entity(obj1);
    engine.scene.push_light(light0);
    engine.scene.push_light(light1);
    engine.scene.push_light(light2);
    engine.scene.push_light(light3);

    let text_fps = text::Text::new("Movimente a camera com: H, J, K e L", glm::vec2(0.0, 150.0), 0.5, glm::vec3(1.0, 1.0, 1.0));
    engine.ui.push_text(text_fps);

    engine.run(&resources);
}

