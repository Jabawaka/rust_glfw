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
    wireframe_length: i32,
    vert_length: i32
}

struct Vertex {
    pos_model: Vector3<f32>,
    pos_screen: Vector2<f32>,
    normal_model: Option<Vector3<f32>>,
    indices: Vec<u32>,
    colours: Vec<f32>,
    pushed: bool,
    highlight: bool,
    selected: bool
}

struct Line {
    verts: (usize, usize)
}

struct Face {
    verts: (usize, usize, usize),
    colour: f32
}

// ---- CONSTANTS FOR COLOURS ----
const COLOUR_GREY: f32 = 0.0;
const COLOUR_RED: f32 = 1.0;
const COLOUR_BLUE: f32 = 2.0;
const COLOUR_GREEN: f32 = 3.0;

// ---- GPU CONSTANTS ----
const SIZE_VERTEX_F32: u32 = 8;


impl Model {
    // -------------------------------------------------------------------------
    // CREATION METHODS
    // -------------------------------------------------------------------------
    // ---- EMPTY MODEL ----
    pub fn create_empty() -> Model {
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
            wireframe_length: 0,
            vert_length: 0
        };

        // Create OpenGL variables
        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
            gl::GenBuffers(1, &mut model.vbo);
            gl::GenBuffers(1, &mut model.ebo);
        }

        model
    }

    // ---- DEFAULT CUBE ----
    pub fn create_cube() -> Model {
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
            wireframe_length: 0,
            vert_length: 0
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
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (0, 3, 1),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (0, 1, 5),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (0, 5, 4),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (0, 4, 6),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (0, 6, 2),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (1, 3, 7),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (1, 7, 5),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (2, 7, 3),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (2, 6, 7),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (4, 5, 7),
            colour: COLOUR_GREY
        });
        model.faces.push(Face {
            verts: (4, 7, 6),
            colour: COLOUR_GREY
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
    pub fn clean(&mut self) {
        // Check faces first
        let n_vert = self.vertices.len();
        self.faces.retain(|face| face.verts.0 < n_vert && 
                                 face.verts.1 < n_vert &&
                                 face.verts.2 < n_vert);

        // Check lines
        self.lines.retain(|line| line.verts.0 < n_vert &&
                                 line.verts.1 < n_vert);
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
            vert.indices = Vec::new();
            vert.colours = Vec::new();
            vert.pushed = false;
        }

        // ---- PROCESS FACES ----
        self.solid_index = 0;

        for face in self.faces.iter_mut() {
            // Calculate face normal
            let vec1 = self.vertices[face.verts.1].pos_model -
                       self.vertices[face.verts.0].pos_model;
            let vec2 = self.vertices[face.verts.2].pos_model -
                       self.vertices[face.verts.0].pos_model;
            let normal = vec1.cross(vec2).normalize();

            // Process each of the vertices
            process_vertex_flat(&self.vertices[face.verts.0],
                           face.colour,
                           normal,
                           &mut vertices,
                           &mut indices);
            process_vertex_flat(&self.vertices[face.verts.1],
                           face.colour,
                           normal,
                           &mut vertices,
                           &mut indices);
            process_vertex_flat(&self.vertices[face.verts.2],
                           face.colour,
                           normal,
                           &mut vertices,
                           &mut indices);
        }

        self.solid_length = indices.len() as i32;

        // Process lines
        self.wireframe_index = self.solid_length as usize;

        let mut final_vertex_index;
        for line in self.lines.iter_mut() {
            let mut curr_vert = &mut self.vertices[line.verts.0];
            if curr_vert.pushed == false {
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

                curr_vert.pushed = true;
                final_vertex_index =
                    vertices.len() as u32 / SIZE_VERTEX_F32 - 1;
                curr_vert.indices.push(final_vertex_index);
                curr_vert.colours.push(0.0);
            } else {
                final_vertex_index = curr_vert.indices[0];
            }
            indices.push(final_vertex_index as i32);

            curr_vert = &mut self.vertices[line.verts.1];
            if curr_vert.pushed == false {
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

                curr_vert.pushed = true;
                final_vertex_index =
                    vertices.len() as u32 / SIZE_VERTEX_F32 - 1;
                curr_vert.indices.push(final_vertex_index);
                curr_vert.colours.push(0.0);
            } else {
                final_vertex_index = curr_vert.indices[0];
            }
            indices.push(final_vertex_index as i32);
        }
        self.wireframe_length = indices.len() as i32 - self.solid_length;

        // Process remaining vertices
        for vertex in self.vertices.iter() {
            if vertex.pushed == false {
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

        self.vert_length = vertices.len() as i32 / SIZE_VERTEX_F32 as i32;

        /*
        let mut index = 0;
        println!("THE DATA");
        while index < vertices.len() {
            print!("{}", vertices[index]);
            if (index + 1) as u32 % SIZE_VERTEX_F32 == 0 {
                print!("\n");
            }
            else {
                print!(", ");
            }

            index += 1;
        }
        */

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
                SIZE_VERTEX_F32 as i32 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 1, gl::FLOAT, gl::FALSE,
                SIZE_VERTEX_F32 as i32 * mem::size_of::<GLfloat>() as GLsizei,
               (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE,
                SIZE_VERTEX_F32 as i32 * mem::size_of::<GLfloat>() as GLsizei,
               (4 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribPointer(3, 1, gl::FLOAT, gl::FALSE,
                SIZE_VERTEX_F32 as i32 * mem::size_of::<GLfloat>() as GLsizei,
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

            if (vertex.pos_screen - cursor_pos_screen).magnitude() < 5.0
            {
                vertex.highlight = true;
            }
            else { vertex.highlight = false; }
        }

        self.update_gpu_data();
    }

    // -------------------------------------------------------------------------
    // PROCESS FACES TO SCREEN SPACE
    // -------------------------------------------------------------------------
    pub fn process_faces
       (&mut self,
        cursor_pos_screen: Vector2::<f32>) {
        
        for face in self.faces.iter_mut() {
            let centroid_pos = (self.vertices[face.verts.0].pos_screen +
                                self.vertices[face.verts.1].pos_screen +
                                self.vertices[face.verts.2].pos_screen) / 3.0;
            
            if (centroid_pos - cursor_pos_screen).magnitude() < 50.0
            {
                self.vertices[face.verts.0].highlight = true;
                self.vertices[face.verts.1].highlight = true;
                self.vertices[face.verts.2].highlight = true;
            }
            else {
                self.vertices[face.verts.0].highlight = false;
                self.vertices[face.verts.1].highlight = false;
                self.vertices[face.verts.2].highlight = false;
            }
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
            gl::DrawArrays(gl::POINTS, 0, self.vert_length);
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
    pub fn add_vert(&mut self, pos_model: Vector3<f32>) {
        self.vertices.push(Vertex {
            pos_model,
            pos_screen: Vector2::zero(),
            normal_model: None,
            indices: Vec::new(),
            colours: Vec::new(),
            pushed: false,
            highlight: false,
            selected: false
        });
    }

    pub fn add_vert_with_normal(&mut self, pos_model: Vector3<f32>, normal_model: Vector3<f32>) {
        self.vertices.push(Vertex {
            pos_model,
            pos_screen: Vector2::zero(),
            normal_model: Some(normal_model.normalize()),
            indices: Vec::new(),
            colours: Vec::new(),
            pushed: false,
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
    // REMOVE VERTEX
    // -------------------------------------------------------------------------
    pub fn remove_vert(&mut self, vert_index: usize) {
        // ---- REMOVE EVERYTHING TO DO WITH THE VERTEX ----
        // Clean faces
        self.faces.retain(|face| face.verts.0 != vert_index && 
                                 face.verts.1 != vert_index &&
                                 face.verts.2 != vert_index);

        // Clean lines
        self.lines.retain(|line| line.verts.0 != vert_index &&
                                 line.verts.1 != vert_index);

        // Remove vertex
        self.vertices.remove(vert_index);

        // ---- UPDATE FACES AND LINES ----
        for face in self.faces.iter_mut() {
            if face.verts.0 > vert_index {
                face.verts.0 = face.verts.0 - 1;
            }
            if face.verts.1 > vert_index {
                face.verts.1 = face.verts.1 - 1;
            }
            if face.verts.2 > vert_index {
                face.verts.2 = face.verts.2 - 1;
            }
        }

        for line in self.lines.iter_mut() {
            if line.verts.0 > vert_index {
                line.verts.0 = line.verts.0 - 1;
            }
            if line.verts.1 > vert_index {
                line.verts.1 = line.verts.1 - 1;
            }
        }
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
    pub fn add_face(&mut self, vert_indices: &Vec::<usize>, colour: f32) {
        self.faces.push(Face {
            verts: (vert_indices[0], vert_indices[1], vert_indices[2]),
            colour
        });

        self.vertices[vert_indices[0]].selected = false;
        self.vertices[vert_indices[1]].selected = false;
        self.vertices[vert_indices[2]].selected = false;
    }
}


fn process_vertex_flat(curr_vert: &Vertex,
                       colour: f32,
                       normal: Vector3::<f32>,
                       vertices: &mut Vec<f32>,
                       indices: &mut Vec<i32>) {
    // Push an entire vertex
    vertices.push(curr_vert.pos_model.x);
    vertices.push(curr_vert.pos_model.y);
    vertices.push(curr_vert.pos_model.z);

    if curr_vert.highlight || curr_vert.selected {
        vertices.push(1.0);
    } else {
        vertices.push(0.0);
    }

    vertices.push(normal.x);
    vertices.push(normal.y);
    vertices.push(normal.z);

    vertices.push(colour);

    indices.push((vertices.len() as u32 / SIZE_VERTEX_F32 - 1) as i32);
}


fn process_vertex(curr_vert: &mut Vertex,
                  colour: f32,
                  face_normal: Vector3::<f32>,
                  vertices: &mut Vec<f32>,
                  indices: &mut Vec<i32>) {
    // ---- PROCESS THE FIRST VERTEX ----
    // Index of the vertex in the final array
    let mut final_vertex_index: u32 = 0;

    // Check if the vertex is already there
    if curr_vert.pushed {
        // Update normals of already pushed vertices that don't have one defined
        let mut updated_normal = Vector3::<f32>::zero();
        if curr_vert.normal_model == None {
            for index in curr_vert.indices.iter() {
                vertices[(index * SIZE_VERTEX_F32 + 4) as usize] += face_normal.x;
                vertices[(index * SIZE_VERTEX_F32 + 5) as usize] += face_normal.y;
                vertices[(index * SIZE_VERTEX_F32 + 6) as usize] += face_normal.z;

                // Cache updated normal for later
                updated_normal = Vector3::new
                    (vertices[(index * SIZE_VERTEX_F32 + 4) as usize],
                    vertices[(index * SIZE_VERTEX_F32 + 5) as usize],
                    vertices[(index * SIZE_VERTEX_F32 + 6) as usize]);
            }
        } else {
            updated_normal = curr_vert.normal_model.unwrap();
        }

        // Vertex was already pushed, but maybe with a different colour
        let mut new_colour_flag = true;
        for (index, curr_colour) in curr_vert.colours.iter().enumerate() {
            if *curr_colour == colour {
                new_colour_flag = false;
                final_vertex_index = curr_vert.indices[index];
            }
        }
        
        // Check if the colour is new
        if new_colour_flag {
            // If the colour is new, push an entire new vertex with the
            // updated normal
            vertices.push(curr_vert.pos_model.x);
            vertices.push(curr_vert.pos_model.y);
            vertices.push(curr_vert.pos_model.z);

            if curr_vert.highlight || curr_vert.selected {
                vertices.push(1.0);
            } else {
                vertices.push(0.0);
            }

            vertices.push(updated_normal.x);
            vertices.push(updated_normal.y);
            vertices.push(updated_normal.z);

            vertices.push(colour);

            // Tell the Vertex struct that it has a new vertex now
            curr_vert.indices.push
                (vertices.len() as u32 / SIZE_VERTEX_F32 - 1);
            curr_vert.colours.push(colour);

            // Capture index for indices array
            final_vertex_index =
                vertices.len() as u32 / SIZE_VERTEX_F32 - 1;
        }
    } else {
        // Push an entire vertex
        vertices.push(curr_vert.pos_model.x);
        vertices.push(curr_vert.pos_model.y);
        vertices.push(curr_vert.pos_model.z);

        if curr_vert.highlight || curr_vert.selected {
            vertices.push(1.0);
        } else {
            vertices.push(0.0);
        }

        if curr_vert.normal_model == None {
            vertices.push(face_normal.x);
            vertices.push(face_normal.y);
            vertices.push(face_normal.z);
        } else {
            vertices.push(curr_vert.normal_model.unwrap().x);
            vertices.push(curr_vert.normal_model.unwrap().y);
            vertices.push(curr_vert.normal_model.unwrap().z);
        }

        vertices.push(colour);

        // Tell the Vertex struct that it has been pushed
        curr_vert.pushed = true;
        curr_vert.indices.push
            (vertices.len() as u32 / SIZE_VERTEX_F32 - 1);
        curr_vert.colours.push(colour);

        // Capture index for indices array
        final_vertex_index =
            vertices.len() as u32 / SIZE_VERTEX_F32 - 1;
    }

    indices.push(final_vertex_index as i32);
}