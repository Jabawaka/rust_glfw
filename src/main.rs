extern crate glfw;
use self::glfw::{Context};

extern crate gl;

use cgmath::{Point3, Vector3};
use cgmath::prelude::*;

mod graphics;
use graphics::*;


// ---- SETTINGS ----
const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;


// ---- STUFF ----
const INPUT_MODE_NOMINAL: i32 = 0;
const INPUT_MODE_ENTER_VERTEX: i32 = 1;
const INPUT_MODE_ENTER_LINE: i32 = 2;
const INPUT_MODE_ENTER_FACE: i32 = 3;


fn main() {
    // -------------------------------------------------------------------------
    // INITIALISATION
    // -------------------------------------------------------------------------
    // ---- GFLW INITIALISATION ----
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // ---- WINDOW CREATION ----
    let mut window = Window::create
       (&mut glfw, (SCREEN_WIDTH, SCREEN_HEIGHT), "TronWarp");

    // ---- OPENGL INITIALISATION ----
    gl::load_with(|symbol| window.glfw_window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::PointSize(10.0);
    }

    let shader_program = Shader::create("shaders/simple.vs", "shaders/simple.fs");

    // ---- OBJECT CREATION ----
    let mut model = Model::create_default();

    let mut camera = Camera::create
       (60.0, (SCREEN_WIDTH, SCREEN_HEIGHT),
        Point3::new(-2.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0));

    // ---- MISC VARIABLES ----
    let mut last_frame = 0.0;
    let mut delta_time;

    let mut is_wf = true;

    let mut quit_flag = false;

    let mut input_mode = INPUT_MODE_NOMINAL;
    let mut input_string = String::new();

    // -------------------------------------------------------------------------
    // MAIN LOOP
    // -------------------------------------------------------------------------
    while quit_flag == false {
        // ---- TIMING VARIABLES ----
        let curr_frame = glfw.get_time() as f32;
        delta_time = curr_frame - last_frame;
        last_frame = curr_frame;

        // ---- PROCESS INPUT ----
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

        if input_mode == INPUT_MODE_NOMINAL {
            if window.was_input_pressed(InputAction::EnterVertex) {
                input_mode = INPUT_MODE_ENTER_VERTEX;
            }
            if window.was_input_pressed(InputAction::EnterLine) {
                input_mode = INPUT_MODE_ENTER_LINE;
            }
            if window.was_input_pressed(InputAction::EnterFace) {
                input_mode = INPUT_MODE_ENTER_FACE;
            }
        }

        // Process inputting a vertex
        if input_mode == INPUT_MODE_ENTER_VERTEX {
            if window.was_input_pressed(InputAction::Num0) {
                input_string.push_str("0");
            }
            if window.was_input_pressed(InputAction::Num1) {
                input_string.push_str("1");
            }
            if window.was_input_pressed(InputAction::Num2) {
                input_string.push_str("2");
            }
            if window.was_input_pressed(InputAction::Num3) {
                input_string.push_str("3");
            }
            if window.was_input_pressed(InputAction::Num4) {
                input_string.push_str("4");
            }
            if window.was_input_pressed(InputAction::Num5) {
                input_string.push_str("5");
            }
            if window.was_input_pressed(InputAction::Num6) {
                input_string.push_str("6");
            }
            if window.was_input_pressed(InputAction::Num7) {
                input_string.push_str("7");
            }
            if window.was_input_pressed(InputAction::Num8) {
                input_string.push_str("8");
            }
            if window.was_input_pressed(InputAction::Num9) {
                input_string.push_str("9");
            }
            if window.was_input_pressed(InputAction::Dot) {
                input_string.push_str(".");
            }
            if window.was_input_pressed(InputAction::Comma) {
                input_string.push_str(",");
            }
            if window.was_input_pressed(InputAction::Minus) {
                input_string.push_str("-");
            }
            if window.was_input_pressed(InputAction::EndCommand) {
                // Process input string
                let str_coords: Vec<&str> = input_string.split(',').collect();
                if str_coords.len() == 3 {
                    let mut float_coords: Vector3<f32> = Vector3::zero();
                    float_coords.x = str_coords[0].parse::<f32>().unwrap();
                    float_coords.y = str_coords[1].parse::<f32>().unwrap();
                    float_coords.z = str_coords[2].parse::<f32>().unwrap();

                    model.add_vert(float_coords);
                } else {
                    println!("You must enter three coords separated by commas");
                }

                // Return to nominal
                input_string = String::new();
                input_mode = INPUT_MODE_NOMINAL;
            }
            if window.was_input_pressed(InputAction::AbortCommand) {
                // Return to nominal emptying string
                input_string = String::new();
                input_mode = INPUT_MODE_NOMINAL;
            }
        }

        // Process input for objects
        camera.process_input(&window);

        // ---- UPDATE ----
        camera.update(delta_time);

        // Process vertices of model
        model.process_vertices(&camera.total_mat, (SCREEN_WIDTH, SCREEN_HEIGHT), window.last_mouse_pos);

        // ---- RENDER ----
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Pass stuff to the shader
            shader_program.bind();
            shader_program.pass_matrix("transMat", &camera.total_mat);

            // Render wireframe or solid color
            if is_wf {
                model.render_wf();
            } else {
                model.render_solid();
            }
        }

        // GLFW: swap buffers and poll IO events
        window.glfw_window.swap_buffers();
        glfw.poll_events();
    }
}
