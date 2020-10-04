extern crate glfw;
use self::glfw::{Context};

extern crate gl;

use cgmath::{Point3, Vector3};

mod graphics;
use graphics::*;

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

    unsafe {
        gl::PointSize(10.0);
    }

    let shader_program = Shader::create("shaders/simple.vs", "shaders/simple.fs");

    let model = Model::create_default();

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
            if is_wf {
                model.render_wf();
            } else {
                model.render_solid();
            }
        }

        // glfw: swap buffers and poll IO events
        // -------------------------------------
        window.glfw_window.swap_buffers();
        glfw.poll_events();
    }
}
