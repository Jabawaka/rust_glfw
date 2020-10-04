extern crate gl;
use self::gl::types::*;

use cgmath::Vector3;

use std::ptr;
use std::mem;
use std::os::raw::c_void;


pub struct Model {
    vao: u32,
    vertices: Vec<Vertex>,
    solid_index: usize,
    solid_length: i32,
    wireframe_index: usize,
    wireframe_length: i32
}

struct Colour {
    red: f32,
    green: f32,
    blue: f32
}

struct Vertex {
    pos_model: Vector3<f32>,
    pos_screen: Vector3<f32>,
    colour: Colour
}


impl Model {
    pub fn create_default() -> Model {
         let vertices: [f32; 24] = [
            0.5,  0.5,  0.5,
            0.5,  0.5, -0.5,
            0.5, -0.5,  0.5,
            0.5, -0.5, -0.5,
           -0.5,  0.5,  0.5,
           -0.5,  0.5, -0.5,
           -0.5, -0.5,  0.5,
           -0.5, -0.5, -0.5
        ];

        let indices: [i32; 60] = [
           // Solid
           0, 1, 3,
           0, 3, 2,
           0, 5, 4,
           0, 1, 5,
           0, 4, 6,
           2, 6, 2,
           6, 5, 7,
           6, 4, 5,
           2, 6, 7,
           2, 7, 3,
           7, 5, 1,
           7, 1, 3,
           // WF
           0, 2,
           0, 1,
           0, 4,
           1, 3,
           1, 5,
           2, 3,
           2, 6,
           3, 7,
           4, 5,
           4, 6,
           5, 7,
           6, 7,
        ];

        // Create the OpenGL structures
        let (mut vbo, mut ebo, mut vao) = (0, 0, 0);

        // Crate the empty structure
        let mut model = Model {
            vao: vao,
            vertices: Vec::<Vertex>::new(),
            solid_index: 0,
            solid_length: 36,
            wireframe_index: 36,
            wireframe_length: 24
        };

        model.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, 0.5, 0.5),
            pos_screen: Vector3::zero(),
            colour: Colour {red: 0.0, green: 0.0, blue: 0.0}
        });

        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(model.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData
               (gl::ARRAY_BUFFER,
               (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
               (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::DYNAMIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);
        }

        model
    }

    pub fn render_solid(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements
               (gl::TRIANGLES,
                self.solid_length, gl::UNSIGNED_INT,
               (self.solid_index * mem::size_of::<GLfloat>()) as *const c_void);
            gl::BindVertexArray(0);
        }
    }

    pub fn render_wf(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::POINTS, 0, 8);
            gl::DrawElements
               (gl::LINES,
                self.wireframe_length, gl::UNSIGNED_INT,
               (self.wireframe_index * mem::size_of::<GLfloat>()) as *const c_void);
            gl::BindVertexArray(0);
        }
    }
}
