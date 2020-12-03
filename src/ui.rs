use crate::camera;
use crate::resources::CharacterSet;
use crate::ui_entity::UIEntity;
use crate::shader::ShaderProgram;
use crate::text::Text;





pub struct UI {
    objects: Vec<Box<dyn UIEntity>>,
    texts: Vec<Text>,

    projection: glm::Mat4,
}

impl UI {
    pub fn new() -> UI {
        let left   = -camera::NATURAL_WIDTH as f32;
        let right  = camera::NATURAL_WIDTH as f32;
        let bottom = -camera::NATURAL_HEIGHT as f32;
        let top    = camera::NATURAL_HEIGHT as f32;
        let near   = -1.0;
        let far    = 2.0;

        let ortho = glm::ortho(left, right, bottom, top, near, far);
        UI {
            objects: Vec::new(),
	    texts: Vec::new(),
	    projection: ortho,
        }
    }
    pub fn render(&self, ui_shader: &ShaderProgram, text_shader: &ShaderProgram, character_set: &CharacterSet){
        for object in &self.objects {
            object.render(ui_shader);
        }
	for text in &self.texts {
	    text.render(text_shader, &self.projection, &character_set);
	}

	    
    }

    pub fn push_object<E: 'static + UIEntity>(&mut self, obj: E){
        self.objects.push(Box::new(obj));
    }
    pub fn push_text(&mut self, text: Text){
        self.texts.push(text);
    }
}
