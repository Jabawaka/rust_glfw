extern crate gl;
use self::gl::types::*;

use cgmath::{Matrix4};
use cgmath::prelude::*;

use std::ffi::CString;
use std::str;
use std::fs;
use std::ptr;

pub struct Shader {
    shader_id: u32,
}

impl Shader {
    pub fn create(vertex_file: &str, fragment_file: &str) -> Shader {
        unsafe {
            // Vertex shader
            let vertex_shader_source = fs::read_to_string(vertex_file).expect("Could not read vertex file");
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR: Vertex shader compilation failed\n{}", str::from_utf8(&info_log).unwrap());
            }

            // Fragment shader
            let fragment_shader_source = fs::read_to_string(fragment_file).expect("Could not read fragment file");
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            info_log.set_len(512 - 1);
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR: Fragment shader compilation failed\n{}", str::from_utf8(&info_log).unwrap());
            }

            // Link shaders
            let shader = Shader {
                shader_id: gl::CreateProgram(),
            };
            gl::AttachShader(shader.shader_id, vertex_shader);
            gl::AttachShader(shader.shader_id, fragment_shader);
            gl::LinkProgram(shader.shader_id);

            gl::GetProgramiv(shader.shader_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader.shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR: Shader program linking failed\n{}", str::from_utf8(&info_log).unwrap());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.shader_id); }
    }

    pub fn pass_colour(&self, colour_name: &str, colour: (f32, f32, f32, f32)) {
        unsafe {
            let colour_name = CString::new(colour_name).unwrap();
            let colour_location = gl::GetUniformLocation(self.shader_id, colour_name.as_ptr());
            gl::Uniform4f(colour_location, colour.0, colour.1, colour.2, colour.3);
        }
    }

    pub fn pass_matrix(&self, matrix_name: &str, matrix: &Matrix4<f32>) {
        unsafe {
            let matrix_name = CString::new(matrix_name).unwrap();
            let matrix_location = gl::GetUniformLocation(self.shader_id, matrix_name.as_ptr());
            gl::UniformMatrix4fv(matrix_location, 1, gl::FALSE, matrix.as_ptr());
        }
    }

    pub fn pass_int(&self, int_name: &str, int_value: i32) {
        unsafe {
            let int_name = CString::new(int_name).unwrap();
            let int_location = gl::GetUniformLocation(self.shader_id, int_name.as_ptr());
            gl::Uniform1i(int_location, int_value);
        }
    }
}