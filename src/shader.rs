extern crate gl;
extern crate nalgebra_glm as glm;

use std;
use std::ffi::{CString};
use std::ptr;
use std::time;
use gl::types::{GLuint, GLenum, GLint, GLchar};

use crate::entity::Entity;


#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

pub struct ShaderSource {
    pub src: CString,
    pub modified_date: time::SystemTime,
}

pub struct ShaderProgram {
    pub id: GLuint,
    pub modified_date: time::SystemTime,
}


impl ShaderProgram {

    pub fn new(src_vs: &ShaderSource, src_fs: &ShaderSource, modified_date: time::SystemTime) -> Result<ShaderProgram, Error> {

        let vs = new_vs_shader(&src_vs)?;
        let fs = new_fs_shader(&src_fs)?;
         
        let id = compile_program(vs, fs)?;

        Ok( ShaderProgram{ id, modified_date} )
    }

    pub fn set_view_matrix(&self, matrix: &glm::Mat4){
        self.set_uniform_mat4("projection", &matrix);
    }

    pub fn set_material(){}

    pub fn set_lights(&self, lights: &Vec<Box<dyn Entity>>){
        self.set_int("u_numPointLights", lights.len() as i32);
        for i in 0..lights.len() {
            let light = lights[i].downcast_to_light();

            self.set_uniform_float(&format!("u_pointLights[{}].intensity", i), light.intensity);
            self.set_uniform_vec3(&format!("u_pointLights[{}].pos", i), &light.pos);
            self.set_uniform_vec3(&format!("u_pointLights[{}].color", i), &light.color);
        }
    }

    pub fn set_ambient_light(&self, value: f32){
        self.set_uniform_float("u_ambientLight", value);
    }


    pub fn set_used(&self) {
        unsafe{
            gl::UseProgram(self.id);
        }
    }

    pub fn set_int(&self, name: &str, value: i32){
        self.set_used();
        unsafe {
            let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform1i(
                location,
                value
            );
        }
    }

    pub fn set_uniform_float(&self, name: &str, value: f32){
        self.set_used();
        unsafe {
            let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform1f(
                location,
                value
            );
        }
    }

    pub fn set_uniform_vec2(&self, name: &str, value: &glm::Vec2){
        self.set_used();
        unsafe {
            let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform2fv(
                location,
                1,
                glm::value_ptr(value).as_ptr() as *const gl::types::GLfloat
            );
        }

    }
    pub fn set_uniform_vec3(&self, name: &str, value: &glm::Vec3){
        self.set_used();
        unsafe {
            let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform3fv(
                location,
                1,
                glm::value_ptr(value).as_ptr() as *const gl::types::GLfloat
            );
        }

    }

    pub fn set_uniform_array(&self, name: &str, value: &glm::Vec3){
        self.set_used();
        unsafe {
            let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform3fv(
                location,
                1,
                glm::value_ptr(value).as_ptr() as *const gl::types::GLfloat
            );
        }
        todo!();
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &glm::Mat4){
        self.set_used();
        unsafe {
            let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                glm::value_ptr(value).as_ptr() as *const gl::types::GLfloat
            );
        }
    }
}

pub type Shader = GLuint;

pub fn new_vs_shader(src: &ShaderSource) -> Result<Shader, Error> {
    let id = shader_id_from_src(&src.src, gl::VERTEX_SHADER)?;

    Ok( id )
}

pub fn new_fs_shader(src: &ShaderSource) -> Result<Shader, Error> {
    let id = shader_id_from_src(&src.src, gl::FRAGMENT_SHADER)?;

    Ok( id )
}

//pub fn new_shader(resources: &Resources, name: &str) -> Result<Shader, Error> {
//    let kind;
//    if name.ends_with(".vs"){
//        kind = gl::VERTEX_SHADER;
//    }
//    else if name.ends_with(".fs"){
//        kind = gl::FRAGMENT_SHADER;
//    }
//    else {
//        return Err( Error{ message: "Unknow shader type".to_string() } );
//    }
//
//    let source = resources.get_shader_src(name);
//
//    let id = shader_id_from_src(&source, kind)?;
//
//    Ok( id )
//}

fn shader_id_from_src(source: &CString, kind: GLenum) -> Result<GLuint, Error> {
    let id;
    unsafe {
        id = gl::CreateShader(kind);
        gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(id);

        let mut success: GLint = 1;

        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

        if success == 0{
            let mut len: GLint = 0;

            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let error = create_whitespace_cstring(len as usize);

            gl::GetShaderInfoLog(id, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);

            return Err(Error { message: error.to_string_lossy().into_owned() } );
        }
    }

    Ok( id )
}



fn compile_program(shader_vs: Shader, shader_fs: Shader) -> Result<GLuint, Error> {
    let id;
    unsafe {
        // create and link the program
        id = gl::CreateProgram();
        gl::AttachShader(id, shader_fs);
        gl::AttachShader(id, shader_vs);
        gl::LinkProgram(id);

        // delete unused shaders
        gl::DetachShader(id, shader_vs);
        gl::DetachShader(id, shader_fs);
        gl::DeleteShader(shader_vs);
        gl::DeleteShader(shader_fs);
        
        // get error length
        let mut len: GLint = 0;
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

        // check for error and get error message
        let mut success = 0;
        let error = create_whitespace_cstring(len as usize);
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

        if success == 0 {
            gl::GetProgramInfoLog(id, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
            println!("erro{}", len);
            return Err( Error{ message: error.to_string_lossy().into_owned() } );
        }
    }

    Ok( id )
}
fn create_whitespace_cstring(len: usize) -> CString {
    
        // Aloca Buffer
        let buffer: Vec<u8> = vec![b' '; len];
        // Convert buffer to CString
        unsafe { CString::from_vec_unchecked(buffer) }
}
