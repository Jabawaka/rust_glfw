extern crate gl;
use self::gl::types::*;

use cgmath::Vector3;
use cgmath::prelude::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;


pub struct Model {
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertices: Vec<Vertex>,
    lines: Vec<Line>,
    faces: Vec<Face>,
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

struct Line {
    verts: (usize, usize)
}

struct Face {
    verts: (usize, usize, usize),
    colour: Colour
}


impl Model {
    pub fn create_default() -> Model {
        // Crate the empty structure
        let mut model = Model {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertices: Vec::<Vertex>::new(),
            lines: Vec::<Line>::new(),
            faces: Vec::<Face>::new(),
            solid_index: 0,
            solid_length: 0,
            wireframe_index: 0,
            wireframe_length: 0
        };

        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
            gl::GenBuffers(1, &mut model.vbo);
            gl::GenBuffers(1, &mut model.ebo);
        }

        model.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, 0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        model.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, 0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        model.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, -0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        model.vertices.push(Vertex {
            pos_model: Vector3::new(0.5, -0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        model.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, 0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        model.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, 0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        model.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, -0.5, 0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });
        model.vertices.push(Vertex {
            pos_model: Vector3::new(-0.5, -0.5, -0.5),
            pos_screen: Vector3::zero(),
            index: 0,
            processed: false
        });

        model.lines.push(Line { verts: (0, 1) });
        model.lines.push(Line { verts: (0, 2) });
        model.lines.push(Line { verts: (0, 4) });
        model.lines.push(Line { verts: (1, 3) });
        model.lines.push(Line { verts: (1, 5) });
        model.lines.push(Line { verts: (2, 3) });
        model.lines.push(Line { verts: (2, 7) });
        model.lines.push(Line { verts: (3, 7) });
        model.lines.push(Line { verts: (4, 5) });
        model.lines.push(Line { verts: (4, 7) });
        model.lines.push(Line { verts: (5, 6) });
        model.lines.push(Line { verts: (6, 7) });

        model.faces.push(Face {
            verts: (0, 2, 3),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (0, 3, 1),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (0, 1, 5),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (0, 5, 4),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (0, 4, 7),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (0, 7, 2),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (1, 3, 6),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (1, 6, 5),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (2, 6, 3),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (2, 7, 6),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (4, 5, 6),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (4, 6, 7),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });

        model.clean();
        model.update();

        model
    }

    fn clean(&mut self) {
        // Check faces first
        for face in self.faces.iter_mut() {
            if  face.verts.0 >= self.vertices.len() ||
                face.verts.1 >= self.vertices.len() ||
                face.verts.2 >= self.vertices.len()
            {
                println!("[WRN] Dropping face because vertex index is wrong");
                //self.faces.remove(face);
            }
        }

        // Check lines
        for line in self.lines.iter_mut() {
            if  line.verts.0 >= self.vertices.len() ||
                line.verts.1 >= self.vertices.len()
            {
                println!("[WRN] Dropping line because vertex index is wrong");
                //self.lines.remove(line);
            }
        }
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
            let mut curr_vert = &mut self.vertices[face.verts.0];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 3 - 1;
            }
            indices.push(curr_vert.index);

            curr_vert = &mut self.vertices[face.verts.1];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 3 - 1;
            }
            indices.push(curr_vert.index);

            curr_vert = &mut self.vertices[face.verts.2];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 3 - 1;
            }
            indices.push(curr_vert.index);
        }
        self.solid_length = indices.len() as i32;

        // Process lines
        self.wireframe_index = self.solid_length as usize;
        for line in self.lines.iter_mut() {
            let mut curr_vert = &mut self.vertices[line.verts.0];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 3 - 1;
            }
            indices.push(curr_vert.index);

            curr_vert = &mut self.vertices[line.verts.1];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 3 - 1;
            }
            indices.push(curr_vert.index);
        }
        self.wireframe_length = indices.len() as i32 - self.solid_length;

        println!("{}, {}, {}, {}", self.solid_index, self.solid_length, self.wireframe_index, self.wireframe_length);

        let mut index = 0;
        while index < vertices.len() {
            print!("{}", vertices[index]);
            if (index + 1) % 3 == 0 {
                print!("\n");
            } else {
                print!(", ");
            }
            index += 1;
        }

        let mut index = 0;
        while index < indices.len() {
            print!("{}", indices[index]);
            if index < self.solid_length as usize {
                if (index + 1) % 3 == 0 {
                    print!("\n");
                } else {
                    print!(", ");
                }
            } else {
                if (index + 1) % 2 == 0 {
                    print!("\n");
                } else {
                    print!(", ");
                }
            }

            index += 1;
        }

        // ---- PASS DATA TO GPU ----
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData
               (gl::ARRAY_BUFFER,
               (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
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
