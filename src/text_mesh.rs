extern crate freetype;
extern crate nalgebra_glm as glm;

use std::mem;
use gl::types::{GLvoid, GLsizeiptr, GLsizei};

use crate::resources::CharacterSet;



pub struct TextMesh {
    vao: u32,
    vbo: u32,
}



impl TextMesh {
    pub fn new() -> TextMesh {
        let mut vao:u32 = 0;
        let mut vbo:u32 = 0;
        
       unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                           (24 * mem::size_of::<f32>()) as GLsizeiptr,
                           0 as *const GLvoid,
                           gl::DYNAMIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,                              // offset
                                    4,                              // length
                                    gl::FLOAT,                      // type
                                    gl::FALSE,                      // normalize
                                    4 * mem::size_of::<f32>() as GLsizei,  // size in bytes stride
                                    0 as *const GLvoid);
           gl::BindBuffer(gl::ARRAY_BUFFER, 0);
           gl::BindVertexArray(0);
        }

        TextMesh {
            vao,
            vbo,
        }
    }

    pub fn render(&self, pos: &glm::Vec2, scale: f32, string: &str, characters: &CharacterSet) {

        let mut x = pos.x;
        let     y = pos.y;

        unsafe{
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.vao);

            for c in string.chars() {
                let ch = characters.get(&c)
                    .unwrap();

                let xpos = x + (ch.bearing.x as f32) * scale;
                let ypos = y - ((ch.size.y - ch.bearing.y) as f32) * scale;

                let w = ch.size.x as f32 * scale;
                let h = ch.size.y as f32 * scale;

                let vertices: [[f32;4];6] = [ [xpos,     ypos + h, 0.0, 0.0],
                                              [xpos,     ypos,     0.0, 1.0],
                                              [xpos + w, ypos,     1.0, 1.0],

                                              [xpos,     ypos + h, 0.0, 0.0],
                                              [xpos + w, ypos,     1.0, 1.0],
                                              [xpos + w, ypos + h, 1.0, 0.0],
                                            ];
                gl::BindTexture(gl::TEXTURE_2D, ch.texture_id);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl::BufferSubData(gl::ARRAY_BUFFER, 0, mem::size_of_val(&vertices) as isize, vertices.as_ptr() as *const GLvoid);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);

                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                x += (ch.advance >> 6) as f32 * scale;
            }

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);

        }


    }

}
