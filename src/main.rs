extern crate glfw;
use self::glfw::{Context};

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;

use cgmath::{Point3, Vector3};

mod graphics;
use graphics::{Shader, Camera, Window, InputAction};

// settings
const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;


fn main() {
    // GLFW initialisation
    // -------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // GLFW window creation
    // --------------------
    let mut window = Window::create(&mut glfw, (SCREEN_WIDTH, SCREEN_HEIGHT), "TronWarp");

    // gl: load all OpenGL function pointers
    // -------------------------------------
    gl::load_with(|symbol| window.glfw_window.get_proc_address(symbol) as *const _);

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

    let mut camera = Camera::create
       (60.0, (SCREEN_WIDTH, SCREEN_HEIGHT),
        Point3::new(-2.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0));

    let mut last_frame = 0.0;
    let mut delta_time;

    let mut is_wf = true;

    let mut quit_flag = false;

    // ---- MAIN LOOP ----
    while quit_flag == false {
        // ---- Timing variables ----
        let curr_frame = glfw.get_time() as f32;
        delta_time = curr_frame - last_frame;
        last_frame = curr_frame;

        // ---- Process input ----
        // Start by storing it all in the window
        window.process_input();

        // Various flag setting
        quit_flag = window.was_input_pressed(InputAction::Close);
        if window.glfw_window.should_close() {
            quit_flag = true;
        }
        if window.was_input_pressed(InputAction::ToggleWF) {
            is_wf = !is_wf;
        }

        // Process input for objects
        camera.process_input(&window);

        // ---- Update ----
        camera.update(delta_time);

        // ---- Render ----
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Pass stuff to the shader
            shader_program.bind();
            shader_program.pass_colour("colour", (0.0, 0.5, 0.0, 1.0));
            shader_program.pass_matrix("transMat", &camera.total_mat);

            // Render wireframe or solid color
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
        window.glfw_window.swap_buffers();
        glfw.poll_events();
    }
}
