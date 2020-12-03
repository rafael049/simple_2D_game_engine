
pub mod crate::input;
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
pub mod editor;
pub mod entity;
pub mod ui_object;
pub mod ui_entity;
pub mod render_engine;
pub mod ui;


pub struct Engine {
    render_engine: RenderEngine,
    input: Input,
}


impl Engine {
    pub fn new() -> Engine{
        let screen_width  = camera::WIDTH as u32;
        let screen_height = camera::HEIGHT as u32;

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

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
        let mut input = input::Input::new(events);

        // Resources
        let mut resources = resources::Resources::new();


        let mut camera = camera::Camera::new(glm::vec2(0.0, 0.0), 1.0, screen_width as i32, screen_height as i32);

        let text_fps = text::Text::new("Movimente a camera comm: HJKL", glm::vec2(0.0, 150.0), 0.5, glm::vec3(1.0, 1.0, 1.0));


        let obj = object::Object::new("player_obj", "boy", "lighting", &mut resources);
        let background = object::Object::new("background", "background1", "lighting", &mut resources);
        let mut obj3 = object::Object::new("sphere", "brick", "lighting", &mut resources);
        let mut light = object::Object::new("light", "light", "shader", &mut resources);
        let mut light2 = light::Light::new(glm::vec3(100.0, 100.0,50.0) , glm::vec3(1.0, 1.0, 1.0));
        let mut light3 = light::Light::new(glm::vec3(-200.0, 300.0,100.0), glm::vec3(1.0, 1.0, 1.0));
        let mut light4 = light::Light::new(glm::vec3(-100.0, 400.0,100.0), glm::vec3(1.0, 1.0, 1.0));

        let mut player = player::Player::new(obj);
        obj3.pos = glm::vec2(-500.0, 200.0);
        light.pos = glm::vec2(10 as f32, 10  as f32);

        let mut scene = scene::Scene::new(camera, player);
        scene.push_light(light2);
        scene.push_light(light3);
        scene.push_light(light4);
        scene.push_entity(background);
        scene.push_entity(obj3);

        let mut ui = ui::UI::new();

        ui.push_text(text_fps);

        render_engine.set_clear_color(glm::vec3(0.2, 0.2, 0.2));

        }

        pub fn run() {

        }
}
