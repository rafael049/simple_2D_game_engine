extern crate stb_image;
extern crate freetype;

use std::fs::{self, File};
use std::io::Read;
use std::ffi::CString;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use stb_image::image::{self, Image, LoadResult};

use crate::shader::{ShaderProgram, ShaderSource};
use crate::texture::Texture;

pub const SHADERS_PATH: &'static str = "assets/shaders/";
pub const IMAGES_PATH: &'static str = "assets/images/";
pub const FONTS_PATH: &'static str = "assets/fonts/";
pub const MAIN_SHADER_NAME: &'static str = "shader";
pub const FONT_SHADER_NAME: &'static str = "text_shader";
pub const UI_SHADER_NAME: &'static str = "ui_shader";
pub const FONT_NAME: &'static str = "arial.ttf";


pub struct Character {
    pub texture_id: u32,
    pub size: glm::IVec2,
    pub bearing: glm::IVec2,
    pub advance: i32,
}

pub type CharacterSet = HashMap<char, Character>;

pub struct Resources {
    shader_program: Rc<RefCell<ShaderProgram>>,
    text_shader:    ShaderProgram,
    ui_shader:      ShaderProgram,
    textures:       HashMap<String, Rc<RefCell<Texture>> >,
    characters:     HashMap<char, Character>,
}


impl Resources {
    pub fn new() -> Resources {

        //create main shader
        let src_vs = load_shader_source(&format!("{}.vs", MAIN_SHADER_NAME)).unwrap();
        let src_fs = load_shader_source(&format!("{}.fs", MAIN_SHADER_NAME)).unwrap();
        let mod_date = if src_fs.modified_date > src_vs.modified_date { src_fs.modified_date } else { src_vs.modified_date };

        let shader_program = Rc::new(RefCell::new(ShaderProgram::new(&src_vs, &src_fs, mod_date).unwrap()));

        //create font shader
        let src_vs = load_shader_source(&format!("{}.vs", FONT_SHADER_NAME)).unwrap();
        let src_fs = load_shader_source(&format!("{}.fs", FONT_SHADER_NAME)).unwrap();
        let mod_date = if src_fs.modified_date > src_vs.modified_date { src_fs.modified_date } else { src_vs.modified_date };

        let text_shader= ShaderProgram::new(&src_vs, &src_fs, mod_date).unwrap();

        //create ui shader
        let src_vs = load_shader_source(&format!("{}.vs", UI_SHADER_NAME)).unwrap();
        let src_fs = load_shader_source(&format!("{}.fs", UI_SHADER_NAME)).unwrap();
        let mod_date = if src_fs.modified_date > src_vs.modified_date { src_fs.modified_date } else { src_vs.modified_date };

        let ui_shader= ShaderProgram::new(&src_vs, &src_fs, mod_date).unwrap();

        // create textures
        let textures       = HashMap::new();

        // create font set
        let characters = load_characters();

        Resources {
            shader_program,
            text_shader,
            ui_shader,
            textures,
            characters,
        }
    }

    pub fn reload_shader(&self){

        let filepath_vs = format!("{}{}.vs", SHADERS_PATH, MAIN_SHADER_NAME);
        let filepath_fs = format!("{}{}.fs", SHADERS_PATH, MAIN_SHADER_NAME);
        let modified_date = self.shader_program.borrow().modified_date;
        let new_modified_date_vs = fs::metadata(&filepath_vs).unwrap().modified().unwrap();
        let new_modified_date_fs = fs::metadata(&filepath_fs).unwrap().modified().unwrap();
        let new_modified_date = if new_modified_date_vs > new_modified_date_fs { new_modified_date_vs } else { new_modified_date_fs };
        if new_modified_date > modified_date {
            // reload shader
            println!("Reloading Main Shader");
            let src_vs = load_shader_source(&format!("{}.vs", MAIN_SHADER_NAME)).unwrap();
            let src_fs = load_shader_source(&format!("{}.fs", MAIN_SHADER_NAME)).unwrap();
            let new_shader = ShaderProgram::new(&src_vs, &src_fs, new_modified_date);
            let mut shader_ref = self.shader_program.borrow_mut();

            match new_shader {
                Err(err) => println!("\n{:?}", err.message),
                Ok(shader) => *shader_ref = shader,
            }
        }
       
    }


    pub fn get_main_shader(&self) -> Rc<RefCell<ShaderProgram>> {
        self.shader_program.clone()
    }

    pub fn get_text_shader(&self) -> &ShaderProgram {
        &self.text_shader
    }

    pub fn get_ui_shader(&self) -> &ShaderProgram {
        &self.ui_shader
    }

    pub fn get_character_set(&self) -> &CharacterSet {
        &self.characters
    }
}


fn load_shader_source(filename: &str) -> Result<ShaderSource, std::io::Error> {
    let mut file = File::open(format!("{}{}", SHADERS_PATH, filename))?;
    let file_size = file
        .metadata()?
        .len() as usize + 1;
    let modified_date = file.metadata()?.modified().unwrap();

    let mut buffer: Vec<u8> = Vec::with_capacity(file_size);
    file.read_to_end(&mut buffer)?;

    let shader_source =
        ShaderSource{ src: CString::new(buffer).expect("Failed to create CSring form buffer"), modified_date };
        

    Ok(shader_source)
}


pub fn load_image(filename: &str) -> Image<u8> {
    unsafe {
        stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    }
    let image = match image::load(format!("{}{}", IMAGES_PATH, filename)){
                    LoadResult::ImageU8(image) => image,
                    _                          => panic!("Failed to load image: {}", filename)
                };

    image
}

pub fn load_characters() -> HashMap<char, Character>{
    let mut characters = HashMap::new();

    // Initialize Freetype
    let ft_lib = freetype::Library::init().unwrap();
    let face = ft_lib.new_face(format!("{}{}", FONTS_PATH, FONT_NAME), 0).unwrap();
    face.set_char_size(40*64, 0, 50, 0).unwrap();

    // create textures
    //
    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1); // Because is using gl::RED
    }
    for c in 0..255 as u8{
        face.load_char(c as usize, freetype::face::LoadFlag::RENDER).unwrap();

        let mut id = 0;
        let width = face.glyph().bitmap().width() as i32;
        let rows = face.glyph().bitmap().rows() as i32;
        let left = face.glyph().bitmap_left() as i32;
        let top  = face.glyph().bitmap_top() as i32;

        unsafe{
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RED as i32,
                width,
                rows,
                0, 
                gl::RED as u32,
                gl::UNSIGNED_BYTE, 
                face.glyph().bitmap().buffer().as_ptr() as *const std::ffi::c_void
            );

        }

        let character = Character{
            texture_id: id,
            size: glm::vec2(width, rows),
            bearing: glm::vec2(left, top),
            advance: face.glyph().advance().x as i32,
        };

        characters.insert(c as char, character);
    }

    characters
}
