extern crate gl;
use self::gl::types::*;

use cgmath::Vector3;
use cgmath::prelude::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;


pub struct Model<'a> {
    vao: u32,
    vertices: Vec<Vertex>,
    lines: Vec<Line<'a>>,
    faces: Vec<Face<'a>>,
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
    index: i32,
    processed: bool
}

struct Line<'a> {
    verts: (&'a mut Vertex, &'a mut Vertex)
}

struct Face<'a> {
    verts: (&'a mut Vertex, &'a mut Vertex, &'a mut Vertex),
    colour: Colour
}


impl<'a> Model<'a> {
    pub fn create_default() -> Model<'a> {
        // Crate the empty structure
        let model = Model {
            vao: 0,
            vertices: Vec::<Vertex>::new(),
            lines: Vec::<Line>::new(),
            faces: Vec::<Face>::new(),
            solid_index: 0,
            solid_length: 0,
            wireframe_index: 0,
            wireframe_length: 0
        };

        model
    }

    pub fn populate_default(&'a mut self) {
        self.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, 0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        self.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, 0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        self.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, -0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        self.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, -0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        self.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, 0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        self.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, 0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        self.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, -0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        self.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, -0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });

        self.lines.push(Line {
            verts: (&mut self.vertices[0], &mut self.vertices[1])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[0], &mut self.vertices[2])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[0], &mut self.vertices[4])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[1], &mut self.vertices[3])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[1], &mut self.vertices[5])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[2], &mut self.vertices[3])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[2], &mut self.vertices[7])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[3], &mut self.vertices[6])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[4], &mut self.vertices[5])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[4], &mut self.vertices[7])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[5], &mut self.vertices[6])
        });
        self.lines.push(Line {
            verts: (&mut self.vertices[6], &mut self.vertices[7])
        });

        self.faces.push(Face {
            verts: (&mut self.vertices[0], &mut self.vertices[2], &mut self.vertices[3]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[0], &mut self.vertices[3], &mut self.vertices[1]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[0], &mut self.vertices[1], &mut self.vertices[5]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[0], &mut self.vertices[5], &mut self.vertices[4]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[0], &mut self.vertices[4], &mut self.vertices[7]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[0], &mut self.vertices[7], &mut self.vertices[2]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[1], &mut self.vertices[3], &mut self.vertices[6]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[1], &mut self.vertices[6], &mut self.vertices[5]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[2], &mut self.vertices[6], &mut self.vertices[3]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[2], &mut self.vertices[7], &mut self.vertices[6]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[4], &mut self.vertices[5], &mut self.vertices[6]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        self.faces.push(Face {
            verts: (&mut self.vertices[4], &mut self.vertices[6], &mut self.vertices[7]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
    }

    fn update(&mut self) {
        // ---- GENERATE ARRAYS FOR GPU ----
        let mut vertices = Vec::<f32>::new();
        let mut indices = Vec::<i32>::new();

        // Clean vertices flag
        for vert in self.vertices.iter_mut() {
            vert.processed = false;
        }

        // Process faces first
        self.solid_index = 0;

        for face in self.faces.iter_mut() {
            if face.verts.0.processed == false {
                vertices.push(face.verts.0.pos_model.x);
                vertices.push(face.verts.0.pos_model.y);
                vertices.push(face.verts.0.pos_model.z);

                face.verts.0.processed = true;
                face.verts.0.index = vertices.len() as i32 - 1;
            }
            indices.push(face.verts.0.index);

            if face.verts.1.processed == false {
                vertices.push(face.verts.1.pos_model.x);
                vertices.push(face.verts.1.pos_model.y);
                vertices.push(face.verts.1.pos_model.z);

                face.verts.1.processed = true;
                face.verts.1.index = vertices.len() as i32 - 1;
            }
            indices.push(face.verts.1.index);

            if face.verts.2.processed == false {
                vertices.push(face.verts.2.pos_model.x);
                vertices.push(face.verts.2.pos_model.y);
                vertices.push(face.verts.2.pos_model.z);

                face.verts.2.processed = true;
                face.verts.2.index = vertices.len() as i32 - 1;
            }
            indices.push(face.verts.2.index);
        }
        self.solid_length = indices.len() as i32;

        // Process lines
        self.wireframe_index = self.solid_length as usize;
        for line in self.lines.iter_mut() {
            if line.verts.0.processed == false {
                vertices.push(line.verts.0.pos_model.x);
                vertices.push(line.verts.0.pos_model.y);
                vertices.push(line.verts.0.pos_model.z);

                line.verts.0.processed = true;
                line.verts.0.index = vertices.len() as i32 - 1;
            }
            indices.push(line.verts.0.index);

            if line.verts.1.processed == false {
                vertices.push(line.verts.1.pos_model.x);
                vertices.push(line.verts.1.pos_model.y);
                vertices.push(line.verts.1.pos_model.z);

                line.verts.1.processed = true;
                line.verts.1.index = vertices.len() as i32 - 1;
            }
            indices.push(line.verts.0.index);
        }

        // ---- PASS DATA TO GPU ----
        unsafe {
            let (mut vbo, mut ebo) = (0, 0);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(self.vao);

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
