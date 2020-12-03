extern crate gl;

use std::mem::size_of;

use gl::types::{GLuint, GLvoid, GLsizeiptr, GLsizei};

use crate::shader::ShaderProgram;
use crate::texture::Texture;


pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
}

impl Mesh {

    pub fn new() -> Mesh {
                                             // vertices       tex coords
        let vertices: Vec<f32>   = vec![ -1.0, -1.0, 0.0,   0.0, 0.0,
                                         -1.0,  1.0, 0.0,   0.0, 1.0,
                                          1.0, -1.0, 0.0,   1.0, 0.0,
                                          1.0, -1.0, 0.0,   1.0, 0.0,
                                          1.0,  1.0, 0.0,   1.0, 1.0,
                                         -1.0,  1.0, 0.0,   0.0, 1.0
                                       ];

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        //let mut ebo: Gluint;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            //gl::GenBuffers(1, &mut rbo);

            // bind vao
            gl::BindVertexArray(vao);

            // bind vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // fill vbo
            gl::BufferData(gl::ARRAY_BUFFER, 
                           (vertices.len() * size_of::<f32>()) as GLsizeiptr,
                           vertices.as_ptr() as *const GLvoid,
                           gl::STATIC_DRAW);
            
            // setup vao for verices
            gl::VertexAttribPointer(0,                              // offset
                                    3,                              // length
                                    gl::FLOAT,                      // type
                                    gl::FALSE,                      // normalize
                                    5 * size_of::<f32>() as GLsizei,  // size in bytes stride
                                    0 as *const GLvoid);

            gl::EnableVertexAttribArray(0);

            // setup vao for texture coords
            gl::VertexAttribPointer(1,                              // offset
                                    2,                              // length
                                    gl::FLOAT,                      // type
                                    gl::FALSE,                      // normalize
                                    5 * size_of::<f32>() as GLsizei,  // size in bytes stride
                                    (3 * size_of::<f32>()) as *const GLvoid);

            gl::EnableVertexAttribArray(1);

            // unbind objects
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Mesh { vao, vbo }
    }

    pub fn render(&self, shader: &ShaderProgram, texture_difuse: &Texture, texture_normal: Option<&Texture>) {
        shader.set_used();
        shader.set_int("u_difuseTexture", 0);
        texture_difuse.bind(0);
        shader.set_int("u_normalTexture", 1);

        match texture_normal {
            Some(normal) => normal.bind(1),
            None         => {},
        };

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

        texture_difuse.unbind();
    }
}
