use crate::object::*;
use crate::light::*;
use crate::camera::*;
use crate::input::*;
use crate::player::*;
use crate::shader::ShaderProgram;
use crate::text::Text;
use crate::resources::Resources;

use crate::entity::Entity;

//use std::{thread, time};

pub struct EntityCollection {
    pub lights:  Vec<Box<dyn Entity>>,
    pub objects: Vec<Box<dyn Entity>>,
    pub player: Box<dyn Entity>,
}

impl EntityCollection {
    pub fn len(&self) -> usize {
        self.objects.len() + self.lights.len() + 1
    }
}



pub struct Scene {
    pub camera: Camera,
    pub entity_collection: EntityCollection,
}


impl Scene {
    pub fn new(camera: Camera, player: Player) -> Scene {
        let entity_collection =
            EntityCollection {
                lights:  Vec::new(),
                objects: Vec::new(),
                player: Box::new(player),

            };

        Scene {
            camera,
            entity_collection,
        }
    }

    pub fn render(&self, shader: &ShaderProgram) {
        shader.set_view_matrix(&self.camera.get_view_projection());
        shader.set_ambient_light(0.4);
        shader.set_lights(&self.entity_collection.lights);

        for obj in &self.entity_collection.objects {
            obj.render(&shader);
        }

        self.entity_collection.player.render(&shader);

    }


    pub fn update(&mut self, input:&Input) {
        self.camera.update(&input);
        self.entity_collection.player.update();
        for obj in &mut self.entity_collection.objects {
            obj.update();
        }
        //self.texts[0].set_text("macunaima");
    }

    pub fn push_entity<E: 'static + Entity>(&mut self, ent: E){
        self.entity_collection.objects.push(Box::new(ent));
    }
    pub fn push_light(&mut self, light: Light){
        self.entity_collection.lights.push(Box::new(light));
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }
}
