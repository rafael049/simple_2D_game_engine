extern crate gl;
extern crate stb_image;

use stb_image::stb_image::bindgen::*;

pub struct Texture {
    id: u32,
}


impl Texture {
    pub fn new( data: *mut stbi_uc, width: usize, height: usize) -> Texture {
        let mut id = 0;

        unsafe{
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0, 
                gl::RGBA as u32,
                gl::UNSIGNED_BYTE, 
                data as *const std::ffi::c_void
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture { id }

    }

    pub fn bind(&self, index: i32){
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self){
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
