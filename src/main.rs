#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::fs;
use std::os::raw::c_void;

use cgmath::{Matrix4, vec3, Deg, Rad, perspective, Point3, Vector3};
use cgmath::prelude::*;

// settings
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// Structure definitions
struct Window {
    glfw_window: glfw::Window,
}

struct Camera {
    pos_glob: Vector3<f32>,
    att_glob: Vector3<f32>,
    vel_glob: Vector3<f32>,
    rot_glob: Vector3<f32>,
    proj_mat: Matrix4<f32>,
    view_mat: Matrix4<f32>,
    total_mat: Matrix4<f32>,
}

impl Camera {
    fn create(fov_deg: f32, render_size: (u32, u32), pos_glob: Vector3<f32>, att_glob: Vector3<f32>) -> Camera {
        let mut cam = Camera {
            pos_glob: pos_glob,
            att_glob: att_glob,
            vel_glob: Vector3::zero(),
            rot_glob: Vector3::zero(),
            proj_mat: Matrix4::identity(),
            view_mat: Matrix4::identity(),
            total_mat: Matrix4::identity()
        };

        cam.proj_mat = cam.proj_mat * perspective(Deg(fov_deg), render_size.0 as f32 / render_size.1 as f32, 0.1, 100.0);

        cam.view_mat = cam.view_mat * Matrix4::<f32>::from_translation(pos_glob);
        cam.view_mat = cam.view_mat * Matrix4::<f32>::from_angle_z(Rad(att_glob.z));
        cam.view_mat = cam.view_mat * Matrix4::<f32>::from_angle_y(Rad(att_glob.y));
        cam.view_mat = cam.view_mat * Matrix4::<f32>::from_angle_x(Rad(att_glob.x));

        cam.total_mat = cam.proj_mat * cam.view_mat;

        cam
    }

    fn process_input(&mut self, window: &mut glfw::Window) {
    }

    fn update(&mut self, delta_time: f32) {
        self.pos_glob += self.vel_glob * delta_time;
        self.rot_glob += self.rot_glob * delta_time;

        self.view_mat = Matrix4::identity();
        self.view_mat = self.view_mat * Matrix4::<f32>::from_translation(self.pos_glob);
        self.view_mat = self.view_mat * Matrix4::<f32>::from_angle_z(Rad(self.att_glob.z));
        self.view_mat = self.view_mat * Matrix4::<f32>::from_angle_y(Rad(self.att_glob.y));
        self.view_mat = self.view_mat * Matrix4::<f32>::from_angle_x(Rad(self.att_glob.x));

        self.total_mat = self.proj_mat * self.view_mat;
    }
}

struct Shader {
    shader_id: u32,
}

impl Shader {
    fn create(vertex_file: &str, fragment_file: &str) -> Shader {
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

    fn bind(&self) {
        unsafe { gl::UseProgram(self.shader_id); }
    }

    fn pass_colour(&self, colour_name: &str, colour: (f32, f32, f32, f32)) {
        unsafe {
            let colour_name = CString::new(colour_name).unwrap();
            let colour_location = gl::GetUniformLocation(self.shader_id, colour_name.as_ptr());
            gl::Uniform4f(colour_location, colour.0, colour.1, colour.2, colour.3);
        }
    }

    fn pass_matrix(&self, matrix_name: &str, matrix: &Matrix4<f32>) {
        unsafe {
            let matrix_name = CString::new(matrix_name).unwrap();
            let matrix_location = gl::GetUniformLocation(self.shader_id, matrix_name.as_ptr());
            gl::UniformMatrix4fv(matrix_location, 1, gl::FALSE, matrix.as_ptr());
        }
    }
}

fn main() {
    // GLFW initialisation
    // -------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // GLFW window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // -------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader_program = Shader::create("shaders/simple.vs", "shaders/simple.fs");

    let vao = unsafe {
        // set up vertex data and configure attributes
        // -------------------------------------------
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

        let (mut vbo, mut ebo, mut vao) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

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

        vao
    };

    let mut camera = Camera::create(60.0, (SCREEN_WIDTH, SCREEN_HEIGHT), Vector3::new(3.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));

    let mut last_frame = 0.0;
    let mut delta_time;

    let mut is_wf = true;
    let mut is_space_down = false;

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let curr_frame = glfw.get_time() as f32;
        delta_time = curr_frame - last_frame;
        last_frame = curr_frame;

        // events
        // ------
        process_events(&mut window, &events);

        // input
        // -----
        process_input(&mut window, delta_time, &mut is_wf, &mut is_space_down);
        camera.process_input(&mut window);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Calculate transform

            // Pass stuff to the shader
            shader_program.bind();
            shader_program.pass_colour("colour", (0.0, 0.5, 0.0, 1.0));
            shader_program.pass_matrix("transMat", &camera.total_mat);

            gl::BindVertexArray(vao);
            if is_wf {
                gl::DrawElements(gl::LINES, 24, gl::UNSIGNED_INT, (36 * mem::size_of::<GLfloat>()) as *const c_void);
            } else {
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, ptr::null());
            }
            gl::BindVertexArray(0);
        }

        // glfw: swap buffers and poll IO events
        // -------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

fn process_input(window: &mut glfw::Window, delta_time: f32, is_wf: &mut bool, is_space_down: &mut bool) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    if window.get_key(Key::Space) == Action::Press  && *is_space_down == false {
        *is_wf = !*is_wf;
        *is_space_down = true;
    }
    if window.get_key(Key::Space) == Action::Release {
        *is_space_down = false;
    }
}
