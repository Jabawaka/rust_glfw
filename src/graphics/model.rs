extern crate gl;
use self::gl::types::*;

use cgmath::{Vector2, Vector3, Matrix4};
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
    pos_screen: Vector2<f32>,
    index: i32,
    processed: bool,
    highlight: bool,
    selected: bool
}

struct Line {
    verts: (usize, usize)
}

struct Face {
    verts: (usize, usize, usize),
    colour: Colour
}


impl Model {
    // -------------------------------------------------------------------------
    // CREATE DEFAULT CUBE
    // -------------------------------------------------------------------------
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

        // Create OpenGL variables
        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
            gl::GenBuffers(1, &mut model.vbo);
            gl::GenBuffers(1, &mut model.ebo);
        }

        // Push vertices to the model
        model.add_vert(Vector3::new( 0.5,  0.5,  0.5));
        model.add_vert(Vector3::new( 0.5,  0.5, -0.5));
        model.add_vert(Vector3::new( 0.5, -0.5,  0.5));
        model.add_vert(Vector3::new( 0.5, -0.5, -0.5));
        model.add_vert(Vector3::new(-0.5,  0.5,  0.5));
        model.add_vert(Vector3::new(-0.5,  0.5, -0.5));
        model.add_vert(Vector3::new(-0.5, -0.5,  0.5));
        model.add_vert(Vector3::new(-0.5, -0.5, -0.5));

        // Define lines
        model.lines.push(Line { verts: (0, 1) });
        model.lines.push(Line { verts: (0, 2) });
        model.lines.push(Line { verts: (0, 4) });
        model.lines.push(Line { verts: (1, 3) });
        model.lines.push(Line { verts: (1, 5) });
        model.lines.push(Line { verts: (2, 3) });
        model.lines.push(Line { verts: (2, 6) });
        model.lines.push(Line { verts: (3, 7) });
        model.lines.push(Line { verts: (4, 5) });
        model.lines.push(Line { verts: (4, 6) });
        model.lines.push(Line { verts: (5, 7) });
        model.lines.push(Line { verts: (6, 7) });

        // Define faces
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
            verts: (0, 4, 6),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (0, 6, 2),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (1, 3, 7),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (1, 7, 5),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (2, 7, 3),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (2, 6, 7),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (4, 5, 7),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });
        model.faces.push(Face {
            verts: (4, 7, 6),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });

        // Remove wrong faces and lines
        model.clean();

        // Pass stuff to GPU
        model.update_gpu_data();

        model
    }

    // -------------------------------------------------------------------------
    // CLEAN UP FACES AND LINES
    // -------------------------------------------------------------------------
    fn clean(&mut self) {
        // Check faces first
        for (index, face) in self.faces.iter_mut().enumerate() {
            if  face.verts.0 >= self.vertices.len() ||
                face.verts.1 >= self.vertices.len() ||
                face.verts.2 >= self.vertices.len()
            {
                println!("[WRN] Dropping face because vertex index is wrong");
                //self.faces.remove(index);
            }
        }

        // Check lines
        for (index, line) in self.lines.iter_mut().enumerate() {
            if  line.verts.0 >= self.vertices.len() ||
                line.verts.1 >= self.vertices.len()
            {
                println!("[WRN] Dropping line because vertex index is wrong");
                //self.lines.remove(index);
            }
        }
    }

    // -------------------------------------------------------------------------
    // UPDATE DATA IN THE GPU
    // -------------------------------------------------------------------------
    pub fn update_gpu_data(&mut self) {
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

                if curr_vert.highlight || curr_vert.selected {
                    vertices.push(1.0);
                } else {
                    vertices.push(0.0);
                }

                vertices.push(0.0);
                vertices.push(0.0);
                vertices.push(1.0);

                vertices.push(1.0);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 8 - 1;
            }
            indices.push(curr_vert.index);

            curr_vert = &mut self.vertices[face.verts.1];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                if curr_vert.highlight || curr_vert.selected {
                    vertices.push(1.0);
                } else {
                    vertices.push(0.0);
                }

                vertices.push(0.0);
                vertices.push(0.0);
                vertices.push(1.0);

                vertices.push(1.0);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 8 - 1;
            }
            indices.push(curr_vert.index);

            curr_vert = &mut self.vertices[face.verts.2];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                if curr_vert.highlight || curr_vert.selected {
                    vertices.push(1.0);
                } else {
                    vertices.push(0.0);
                }

                vertices.push(0.0);
                vertices.push(0.0);
                vertices.push(1.0);

                vertices.push(1.0);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 8 - 1;
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

                if curr_vert.highlight || curr_vert.selected {
                    vertices.push(1.0);
                } else {
                    vertices.push(0.0);
                }

                vertices.push(0.0);
                vertices.push(0.0);
                vertices.push(0.0);

                vertices.push(0.0);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 8 - 1;
            }
            indices.push(curr_vert.index);

            curr_vert = &mut self.vertices[line.verts.1];
            if curr_vert.processed == false {
                vertices.push(curr_vert.pos_model.x);
                vertices.push(curr_vert.pos_model.y);
                vertices.push(curr_vert.pos_model.z);

                if curr_vert.highlight || curr_vert.selected {
                    vertices.push(1.0);
                } else {
                    vertices.push(0.0);
                }

                vertices.push(0.0);
                vertices.push(0.0);
                vertices.push(0.0);

                vertices.push(0.0);

                curr_vert.processed = true;
                curr_vert.index = vertices.len() as i32 / 8 - 1;
            }
            indices.push(curr_vert.index);
        }
        self.wireframe_length = indices.len() as i32 - self.solid_length;

        // Process remaining vertices
        for vertex in self.vertices.iter() {
            if vertex.processed == false {
                vertices.push(vertex.pos_model.x);
                vertices.push(vertex.pos_model.y);
                vertices.push(vertex.pos_model.z);

                if vertex.highlight || vertex.selected {
                    vertices.push(1.0);
                } else {
                    vertices.push(0.0);
                }

                vertices.push(0.0);
                vertices.push(0.0);
                vertices.push(0.0);

                vertices.push(0.0);
            }
        }

        let mut index = 0;
        while index < vertices.len() {
            print!("{}", vertices[index]);
            if (index + 1) % 8 == 0 {
                print!("\n");
            }
            else {
                print!(", ");
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

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                8 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 1, gl::FLOAT, gl::FALSE,
                8 * mem::size_of::<GLfloat>() as GLsizei,
               (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE,
                8 * mem::size_of::<GLfloat>() as GLsizei,
               (4 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribIPointer(3, 1, gl::INT,
                8 * mem::size_of::<GLfloat>() as GLsizei,
               (7 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(3);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);
        }
    }

    // -------------------------------------------------------------------------
    // PROCESS VERTICES TO SCREEN SPACE
    // -------------------------------------------------------------------------
    pub fn process_vertices
       (&mut self,
        proj_view_mat: &Matrix4<f32>,
        size: (u32, u32),
        cursor_pos_screen: Vector2::<f32>) {
        for vertex in self.vertices.iter_mut() {
            let pos_screen = proj_view_mat * vertex.pos_model.extend(1.0);
            vertex.pos_screen = pos_screen.truncate().truncate() / pos_screen.w;
            vertex.pos_screen.x =
                (vertex.pos_screen.x + 1.0) * size.0 as f32 / 2.0;
            vertex.pos_screen.y =
                (1.0 - vertex.pos_screen.y) * size.1 as f32 / 2.0;

            if  (vertex.pos_screen - cursor_pos_screen).magnitude() < 5.0
            {
                vertex.highlight = true;
            }
            else { vertex.highlight = false; }
        }

        self.update_gpu_data();
    }

    // -------------------------------------------------------------------------
    // RENDER SOLID FACES
    // -------------------------------------------------------------------------
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

    // -------------------------------------------------------------------------
    // RENDER VERTICES AND LINES
    // -------------------------------------------------------------------------
    pub fn render_wf(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::POINTS, 0, self.vertices.len() as i32);
            gl::DrawElements
               (gl::LINES,
                self.wireframe_length, gl::UNSIGNED_INT,
               (self.wireframe_index * mem::size_of::<GLfloat>()) as *const c_void);
            gl::BindVertexArray(0);
        }
    }

    // -------------------------------------------------------------------------
    // ADD VERTEX
    // -------------------------------------------------------------------------
    pub fn add_vert(&mut self, vertex_pos_model: Vector3<f32>) {
        self.vertices.push(Vertex {
            pos_model: vertex_pos_model,
            pos_screen: Vector2::zero(),
            index: 0,
            processed: false,
            highlight: false,
            selected: false
        });
    }

    // -------------------------------------------------------------------------
    // SELECT VERTEX
    // -------------------------------------------------------------------------
    pub fn select_vert(&mut self, cursor_pos_screen: Vector2<f32>)
      -> Option<usize> {
        for (index, vertex) in self.vertices.iter_mut().enumerate() {
            if (vertex.pos_screen - cursor_pos_screen).magnitude() < 5.0
             && vertex.selected == false {
                vertex.selected = true;
                return Some(index);
            }
        }
        return None;
    }

    // -------------------------------------------------------------------------
    // ADD LINE
    // -------------------------------------------------------------------------
    pub fn add_line(&mut self, vert_indices: &Vec::<usize>) {
        self.lines.push(Line {
            verts: (vert_indices[0], vert_indices[1])
        });

        self.vertices[vert_indices[0]].selected = false;
        self.vertices[vert_indices[1]].selected = false;
    }

    // -------------------------------------------------------------------------
    // ADD FACE
    // -------------------------------------------------------------------------
    pub fn add_face(&mut self, vert_indices: &Vec::<usize>) {
        self.faces.push(Face {
            verts: (vert_indices[0], vert_indices[1], vert_indices[2]),
            colour: Colour { red: 0.0, green: 0.0, blue: 0.0}
        });

        self.vertices[vert_indices[0]].selected = false;
        self.vertices[vert_indices[1]].selected = false;
        self.vertices[vert_indices[2]].selected = false;
    }
}
