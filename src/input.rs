extern crate glfw;

use glfw::{Action, Key};

use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub struct Input {
    events:            Receiver<(f64, glfw::WindowEvent)>,
    keys_map:          HashMap<glfw::Key, bool>,
    last_keys_map:     HashMap<glfw::Key, bool>,
    //mouse_buttons_map: HashMap<sdl2::mouse::MouseButton, bool>,
    mouse_motion:      (f32, f32),
    quit:              bool,
}


impl Input {

    pub fn new(events: Receiver<(f64, glfw::WindowEvent)>) -> Input {
        Input { events:               events,
                keys_map:          HashMap::new(),
                last_keys_map:     HashMap::new(),
                //mouse_buttons_map: HashMap::new(),
                mouse_motion:      (0.0, 0.0),
                quit:              false,
              }
    }

    pub fn update(&mut self, glfw: &mut glfw::Glfw) {
        self.mouse_motion = (0.0, 0.0);
        self.last_keys_map.clear();

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events){
            match event {
                glfw::WindowEvent::Key(key, _, Action::Press, _) => { self.keys_map.insert(key, true);
                                                                          self.last_keys_map.insert(key, true);
                                                                        },
                 glfw::WindowEvent::Key(key, _, Action::Release, _)  => {self.keys_map.insert(key, false);},

                //sdl2::event::Event::MouseButtonDown { mouse_btn, ..} => { self.mouse_buttons_map.insert(mouse_btn, true); },
                //sdl2::event::Event::MouseButtonUp { mouse_btn, ..}   => { self.mouse_buttons_map.remove(&mouse_btn); },

                //sdl2::event::Event::MouseMotion { xrel, yrel, ..}    => { self.mouse_motion = (xrel as f32, yrel as f32); },

                //sdl2::event::Event::Quit {..}                           => self.quit = true,

                _                                                    => {},
            }
        }
    }
    pub fn get_key(&self, key: Key) -> bool {
        match self.keys_map.get(&key) {
            Some(true) => true,
            _       => false,
        }
    }

    pub fn get_on_key_down(&self, key: Key) -> bool {
        match self.last_keys_map.get(&key) {
            Some(true) => {println!("Key down");true},
            _          => false,
        }
    }

    //pub fn get_mouse_btn(&mut self, btn: sdl2::mouse::MouseButton) -> bool {
    //    //match self.mouse_buttons_map.get(&btn) {
    //    //    Some(_) => true,
    //    //    _       => false,
    //    //}
    //}

    pub fn get_quit(&mut self) -> bool {
        self.quit
    }

    pub fn get_mouse_motion_x(&self) -> f32 {
        self.mouse_motion.0
    }

    pub fn get_mouse_motion_y(&self) -> f32 {
        self.mouse_motion.1
    }

}
