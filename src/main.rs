extern crate glfw;
use self::glfw::{Context};

extern crate gl;

use cgmath::{Point3, Vector3};
use cgmath::prelude::*;

mod graphics;
use graphics::*;

use std::cmp;


// ---- SETTINGS ----
const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

const CAM_WIDTH: u32 = 640;
const CAM_HEIGHT: u32 = 360;


// ---- STUFF ----
const INPUT_MODE_NOMINAL: i32 = 0;
const INPUT_MODE_ENTER_VERTEX: i32 = 1;
const INPUT_MODE_ENTER_LINE: i32 = 2;
const INPUT_MODE_ENTER_FACE: i32 = 3;

const MATH_PI: f32 = std::f32::consts::PI;


fn main() {
    // -------------------------------------------------------------------------
    // INITIALISATION
    // -------------------------------------------------------------------------
    // ---- GFLW INITIALISATION ----
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // ---- WINDOW CREATION ----
    let mut window = Window::create
       (&mut glfw, (SCREEN_WIDTH, SCREEN_HEIGHT), "TronWarp");

    let (scr_width, scr_height) = window.glfw_window.get_framebuffer_size();

    // ---- OPENGL INITIALISATION ----
    gl::load_with(|symbol| window.glfw_window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::PointSize(10.0);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);

        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);

        gl::Enable(gl::MULTISAMPLE);
    }

    let normal_shader = Shader::create("shaders/simple.vs", "shaders/simple.fs");
    let wf_shader = Shader::create("shaders/wireframe.vs", "shaders/wireframe.fs");
    let cam_shader = Shader::create("shaders/camera.vs", "shaders/camera.fs");

    // ---- OBJECT CREATION ----
    let mut model = Model::create_empty();

    // Generate ellipsoid
    let a1 = 1.5;
    let a2 = 0.5;
    let b = 1.0;
    let c1 = 0.8;
    let c2 = 0.3;

    let n_theta = 10;
    let n_lambda = 15;

    for theta_index in 0..n_theta {
        // Create point coords
        let theta = -MATH_PI / 2.0 + 0.1 + (MATH_PI - 0.2) * theta_index as f32 / (n_theta as f32 - 1.0);

        for lambda_index in 0..n_lambda {
            let lambda = 0.0 + 2.0 * MATH_PI * lambda_index as f32 / (n_lambda as f32 - 1.0);

            // Select appropriate variables for the semi axis
            let mut a = a1;
            if theta < 0.0 {
                a = a2;
            }

            let mut c = c1;
            if lambda > MATH_PI {
                c = c2;
            }

            // Add the vertex to the model
            model.add_vert(Vector3::new(a * theta.sin(),
                                        b * theta.cos() * lambda.cos(),
                                        c * theta.cos() * lambda.sin()));

            // Add the line that joins to the previous one at the same theta
            // slice
            if lambda_index != 0 {
                model.add_line
                   (&vec![lambda_index - 1 + theta_index * n_lambda,
                          lambda_index + theta_index * n_lambda]);
            }

            // Add the line that joins to the previous one at the same lambda
            // slice
            if theta_index != 0 {
                model.add_line
                   (&vec![lambda_index + (theta_index - 1) * n_lambda,
                          lambda_index + theta_index * n_lambda]);
            }

            // Add the faces corresponding to that vertex
            if lambda_index != 0 && theta_index != 0 {
                model.add_face
                   (&vec![lambda_index - 1 + theta_index * n_lambda,
                          lambda_index - 1 + (theta_index - 1) * n_lambda,
                          lambda_index + theta_index * n_lambda], 0.0);
                model.add_face
                   (&vec![lambda_index - 1 + (theta_index - 1) * n_lambda,
                          lambda_index + (theta_index - 1) * n_lambda,
                          lambda_index + theta_index * n_lambda], 0.0);
            }
        }
    }

    // Add last two vertices and lines joining them
    model.add_vert(Vector3::new(a1, 0.0, 0.0));
    model.add_vert(Vector3::new(-a2, 0.0, 0.0));

    for lambda_index in 0..n_lambda {
        model.add_line(&vec![lambda_index, n_lambda  * n_theta + 1]);
        model.add_line(&vec![lambda_index + (n_theta - 1) * n_lambda, n_lambda * n_theta]);

        if lambda_index != 0 {
            model.add_face(&vec![lambda_index, lambda_index - 1, n_lambda * n_theta + 1], 0.0);
            model.add_face(&vec![lambda_index - 1 + (n_theta - 1) * n_lambda, lambda_index + (n_theta - 1) * n_lambda, n_lambda * n_theta], 0.0);
        }
    }

    model.clean();
    model.update_gpu_data();

    // ---- CAMERA CREATION ----
    let mut camera = Camera::create
       (60.0, (CAM_WIDTH, CAM_HEIGHT),
        Point3::new(-2.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0));

    // ---- MISC VARIABLES ----
    let mut last_frame = 0.0;
    let mut delta_time;

    let mut is_wf = true;

    let mut quit_flag = false;

    let mut input_mode = INPUT_MODE_NOMINAL;
    let mut input_string = String::new();
    let mut vert_indices = Vec::<usize>::new();

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
                    model.update_gpu_data();
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

        if input_mode == INPUT_MODE_ENTER_LINE {
            if window.was_input_pressed(InputAction::Select) {
                match model.select_vert(window.last_mouse_pos) {
                    Some(index) => vert_indices.push(index),
                    None => ()
                }
            }
            if window.was_input_pressed(InputAction::AbortCommand) {
                // Return to nominal emptying string
                vert_indices = Vec::<usize>::new();
                input_mode = INPUT_MODE_NOMINAL;
            }

            if vert_indices.len() == 2 {
                model.add_line(&vert_indices);
                vert_indices = Vec::<usize>::new();
                input_mode = INPUT_MODE_NOMINAL;
            }
        }

        if input_mode == INPUT_MODE_ENTER_FACE {
            if window.was_input_pressed(InputAction::Select) {
                match model.select_vert(window.last_mouse_pos) {
                    Some(index) => vert_indices.push(index),
                    None => ()
                }
            }
            if window.was_input_pressed(InputAction::AbortCommand) {
                // Return to nominal emptying string
                vert_indices = Vec::<usize>::new();
                input_mode = INPUT_MODE_NOMINAL;
            }

            if vert_indices.len() == 3 {
                model.add_face(&vert_indices, 0.0);
                vert_indices = Vec::<usize>::new();
                input_mode = INPUT_MODE_NOMINAL;
            }
        }

        // Process input for objects
        camera.process_input(&window);

        // ---- UPDATE ----
        camera.update(delta_time);

        // Process vertices of model
        model.process_vertices
           (&camera.total_mat,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            window.last_mouse_pos);

        // ---- RENDER ----
        camera.activate();
        unsafe {
            // Render wireframe or solid color
            if is_wf {
                wf_shader.bind();
                wf_shader.pass_matrix("transMat", &camera.total_mat);
                model.render_wf();
            } else {
                normal_shader.bind();
                normal_shader.pass_matrix("transMat", &camera.total_mat);
                model.render_solid();
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, scr_width, scr_height);
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            cam_shader.bind();
            cam_shader.pass_int("layer", 1);
            cam_shader.pass_int("textureSampler", 0);
            camera.render();
        }

        // GLFW: swap buffers and poll IO events
        window.glfw_window.swap_buffers();
        glfw.poll_events();
    }
}
