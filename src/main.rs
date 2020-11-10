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

// ---- CAMERA SIZE ----
// This should be fixed to 480x270 so that with 4 players it's 1920x1080
const CAM_WIDTH: u32 = 480;
const CAM_HEIGHT: u32 = 270;


// ---- STUFF ----
const INPUT_MODE_NOMINAL: i32 = 0;
const INPUT_MODE_ENTER_VERTEX: i32 = 1;
const INPUT_MODE_ENTER_LINE: i32 = 2;
const INPUT_MODE_ENTER_FACE: i32 = 3;
const INPUT_MODE_DELETE_VERTEX: i32 = 4;

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

    create_phat_ship(&mut model);
    model.write_to_file("models/phat_ship.mdl");

    // Load last model
    model.load_from_file("models/last_model.mdl");

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
            if window.was_input_pressed(InputAction::DeleteVertex) {
                input_mode = INPUT_MODE_DELETE_VERTEX;
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

        if input_mode == INPUT_MODE_DELETE_VERTEX {
            if window.was_input_pressed(InputAction::Select) {
                match model.select_vert(window.last_mouse_pos) {
                    Some(index) => {
                        model.remove_vert(index);
                        input_mode = INPUT_MODE_NOMINAL;
                    },
                    None => ()
                }
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

        if !is_wf {
            model.process_faces(window.last_mouse_pos);
        }

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

    model.write_to_file("models/last_model.mdl");
}


// -----------------------------------------------------------------------------
// CODE FOR CREATING THE PHATSHIP
// -----------------------------------------------------------------------------
fn create_phat_ship(model: &mut Model) {
    // ---- BODY ELLIPSOID ----
    // Define semi axis
    let a1 = 1.5;
    let a2 = 0.5;
    let b = 1.0;
    let c1 = 0.8;
    let c2 = 0.3;

    // Number of points on each polar coordinate
    let n_theta = 10;
    let n_lambda = 15;

    // Loop creating the points
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
            let x_pos = a * theta.sin();
            let y_pos = b * theta.cos() * lambda.cos();
            let z_pos = c * theta.cos() * lambda.sin();
            model.add_vert_with_normal(Vector3::new(x_pos, y_pos, z_pos),
                                       Vector3::new(x_pos / (a * a),
                                                    y_pos / (b * b),
                                                    z_pos / (c * c)));

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
    model.add_vert_with_normal(Vector3::new(a1, 0.0, 0.0),
                               Vector3::new(1.0, 0.0, 0.0));
    model.add_vert_with_normal(Vector3::new(-a2, 0.0, 0.0),
                               Vector3::new(-1.0, 0.0, 0.0));

    for lambda_index in 0..n_lambda {
        model.add_line(&vec![lambda_index, n_lambda  * n_theta + 1]);
        model.add_line(&vec![lambda_index + (n_theta - 1) * n_lambda, n_lambda * n_theta]);

        if lambda_index != 0 {
            model.add_face(&vec![lambda_index, lambda_index - 1, n_lambda * n_theta + 1], 0.0);
            model.add_face(&vec![lambda_index - 1 + (n_theta - 1) * n_lambda, lambda_index + (n_theta - 1) * n_lambda, n_lambda * n_theta], 0.0);
        }
    }

    let mut index_offset = n_lambda * n_theta + 2;

    // ---- LEFT PUSHER OUTER SURFACE----
    // Define guides
    let x_max = 2.0;
    let x_min = -0.5;
    let y1_top = 1.6;
    let y1_bot = 2.4;
    let y2_top = 1.8;
    let y2_bot = 3.0;

    let z_top = 1.5;
    let z_bot = -0.5;

    let a_top = (y2_top - y1_top) / ((x_min - x_max) * (x_min - x_max));
    let b_top = -2.0 * a_top * x_max;
    let c_top = y1_top + a_top * x_max * x_max;

    let a_bot = (y2_bot - y1_bot) / ((x_min - x_max) * (x_min - x_max));
    let b_bot = -2.0 * a_bot * x_max;
    let c_bot = y1_bot + a_bot * x_max * x_max;

    // Number of points along x and y
    let n_x = 5;
    let n_z = 5;

    for x_index in 0..n_x {
        // Get position in X and top and bottom Y position
        let x_pos = x_min + (x_max - x_min) * x_index as f32 / (n_x as f32 - 1.0);

        let y_pos_top = a_top * x_pos * x_pos + b_top * x_pos + c_top;
        let y_pos_bot = a_bot * x_pos * x_pos + b_bot * x_pos + c_bot;

        // Generate coefficients for YZ parabola
        let a_yz = (y_pos_bot - y_pos_top) / ((z_bot - z_top) * (z_bot - z_top));
        let b_yz = -2.0 * a_yz * z_top;
        let c_yz = y_pos_top + a_yz * z_top * z_top;

        for z_index in 0..n_z {
            let z_pos = z_bot + (z_top - z_bot) * z_index as f32 / (n_z as f32 - 1.0);
            let y_pos = a_yz * z_pos * z_pos + b_yz * z_pos + c_yz;

            model.add_vert(Vector3::new(x_pos, y_pos, z_pos));

            // Add the line that joins to the previous one at the same x
            // slice
            if z_index != 0 {
                model.add_line
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                        z_index + x_index * n_z + index_offset]);
            }

            // Add the line that joins to the previous one at the same z
            // slice
            if x_index != 0 {
                model.add_line
                (&vec![z_index + (x_index - 1) * n_z + index_offset,
                        z_index + x_index * n_z + index_offset]);
            }

            // Add the faces corresponding to that vertex
            if z_index != 0 && x_index != 0 {
                model.add_face
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                        z_index - 1 + (x_index - 1) * n_z + index_offset,
                        z_index + x_index * n_z + index_offset], 0.0);
                model.add_face
                (&vec![z_index - 1 + (x_index - 1) * n_z + index_offset,
                        z_index + (x_index - 1) * n_z + index_offset,
                        z_index + x_index * n_z + index_offset], 0.0);
            }
        }
    }

    index_offset = index_offset + n_x * n_z;

    // ---- LEFT PUSHER INNER SURFACE----
    // Define guides
    let x_max = 2.0;
    let x_min = -0.5;
    let y1_top = 1.3;
    let y1_bot = 2.1;
    let y2_top = 1.3;
    let y2_bot = 2.0;

    let z_top = 1.5;
    let z_bot = -0.5;

    let a_top = (y2_top - y1_top) / ((x_min - x_max) * (x_min - x_max));
    let b_top = -2.0 * a_top * x_max;
    let c_top = y1_top + a_top * x_max * x_max;

    let a_bot = (y2_bot - y1_bot) / ((x_min - x_max) * (x_min - x_max));
    let b_bot = -2.0 * a_bot * x_max;
    let c_bot = y1_bot + a_bot * x_max * x_max;

    for x_index in 0..n_x {
        // Get position in X and top and bottom Y position
        let x_pos = x_min + (x_max - x_min) * x_index as f32 / (n_x as f32 - 1.0);

        let y_pos_top = a_top * x_pos * x_pos + b_top * x_pos + c_top;
        let y_pos_bot = a_bot * x_pos * x_pos + b_bot * x_pos + c_bot;

        // Generate coefficients for YZ parabola
        let a_yz = (y_pos_bot - y_pos_top) / ((z_bot - z_top) * (z_bot - z_top));
        let b_yz = -2.0 * a_yz * z_top;
        let c_yz = y_pos_top + a_yz * z_top * z_top;

        for z_index in 0..n_z {
            let z_pos = z_bot + (z_top - z_bot) * z_index as f32 / (n_z as f32 - 1.0);
            let y_pos = a_yz * z_pos * z_pos + b_yz * z_pos + c_yz;

            model.add_vert(Vector3::new(x_pos, y_pos, z_pos));

            // Add the line that joins to the previous one at the same x
            // slice
            if z_index != 0 {
                model.add_line
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                       z_index + x_index * n_z + index_offset]);
            }

            // Add the line that joins to the previous one at the same z
            // slice
            if x_index != 0 {
                model.add_line
                (&vec![z_index + (x_index - 1) * n_z + index_offset,
                       z_index + x_index * n_z + index_offset]);
            }

            // Add the faces corresponding to that vertex
            if z_index != 0 && x_index != 0 {
                model.add_face
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                       z_index + x_index * n_z + index_offset,
                       z_index - 1 + (x_index - 1) * n_z + index_offset], 0.0);
                model.add_face
                (&vec![z_index - 1 + (x_index - 1) * n_z + index_offset,
                       z_index + x_index * n_z + index_offset,
                       z_index + (x_index - 1) * n_z + index_offset], 0.0);
            }
        }
    }

    // ---- STITCH THEM TOGETHER ----
    for x_index in 0..n_x {
        for z_index in 0..n_z {
            // Add lines
            if x_index == 0 || x_index == n_x - 1 ||
               z_index == 0 || z_index == n_z - 1 {
                model.add_line(&vec![z_index + x_index * n_z + index_offset,
                                     z_index + x_index * n_z + index_offset - n_x * n_z]);
            }

            // Add faces
            if x_index == 0 {
                if z_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index - 1 + x_index * n_z + index_offset], 0.0);
                    model.add_face(&vec![z_index - 1 + x_index * n_z + index_offset - n_x * n_z,
                                         z_index - 1 + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                }
            }
            if x_index == n_x - 1 {
                if z_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index - 1 + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                    model.add_face(&vec![z_index - 1 + x_index * n_z + index_offset - n_x * n_z,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index - 1 + x_index * n_z + index_offset], 0.0);
                }
            }
            if z_index == 0 {
                if x_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index + (x_index - 1) * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                    model.add_face(&vec![z_index + (x_index - 1) * n_z + index_offset - n_x * n_z,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index + (x_index - 1) * n_z + index_offset], 0.0);
                }
            }
            if z_index == n_z - 1 {
                if x_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index + (x_index - 1) * n_z + index_offset], 0.0);
                    model.add_face(&vec![z_index + (x_index - 1) * n_z + index_offset - n_x * n_z,
                                         z_index + (x_index - 1) * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                }
            }
        }
    }

    index_offset = index_offset + n_x * n_z;

    // ---- RIGHT PUSHER OUTER SURFACE----
    // Define guides
    let x_max = 2.0;
    let x_min = -0.5;
    let y1_top = 1.6;
    let y1_bot = 2.4;
    let y2_top = 1.8;
    let y2_bot = 3.0;

    let z_top = 1.5;
    let z_bot = -0.5;

    let a_top = (y2_top - y1_top) / ((x_min - x_max) * (x_min - x_max));
    let b_top = -2.0 * a_top * x_max;
    let c_top = y1_top + a_top * x_max * x_max;

    let a_bot = (y2_bot - y1_bot) / ((x_min - x_max) * (x_min - x_max));
    let b_bot = -2.0 * a_bot * x_max;
    let c_bot = y1_bot + a_bot * x_max * x_max;

    // Number of points along x and y
    let n_x = 5;
    let n_z = 5;

    for x_index in 0..n_x {
        // Get position in X and top and bottom Y position
        let x_pos = x_min + (x_max - x_min) * x_index as f32 / (n_x as f32 - 1.0);

        let y_pos_top = a_top * x_pos * x_pos + b_top * x_pos + c_top;
        let y_pos_bot = a_bot * x_pos * x_pos + b_bot * x_pos + c_bot;

        // Generate coefficients for YZ parabola
        let a_yz = (y_pos_bot - y_pos_top) / ((z_bot - z_top) * (z_bot - z_top));
        let b_yz = -2.0 * a_yz * z_top;
        let c_yz = y_pos_top + a_yz * z_top * z_top;

        for z_index in 0..n_z {
            let z_pos = z_bot + (z_top - z_bot) * z_index as f32 / (n_z as f32 - 1.0);
            let y_pos = a_yz * z_pos * z_pos + b_yz * z_pos + c_yz;

            model.add_vert(Vector3::new(x_pos, -y_pos, z_pos));

            // Add the line that joins to the previous one at the same x
            // slice
            if z_index != 0 {
                model.add_line
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                       z_index + x_index * n_z + index_offset]);
            }

            // Add the line that joins to the previous one at the same z
            // slice
            if x_index != 0 {
                model.add_line
                (&vec![z_index + (x_index - 1) * n_z + index_offset,
                       z_index + x_index * n_z + index_offset]);
            }

            // Add the faces corresponding to that vertex
            if z_index != 0 && x_index != 0 {
                model.add_face
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                       z_index + x_index * n_z + index_offset,
                       z_index - 1 + (x_index - 1) * n_z + index_offset], 0.0);
                model.add_face
                (&vec![z_index - 1 + (x_index - 1) * n_z + index_offset,
                       z_index + x_index * n_z + index_offset,
                       z_index + (x_index - 1) * n_z + index_offset], 0.0);
            }
        }
    }

    index_offset = index_offset + n_x * n_z;

    // ---- RIGHT PUSHER INNER SURFACE----
    // Define guides
    let x_max = 2.0;
    let x_min = -0.5;
    let y1_top = 1.3;
    let y1_bot = 2.1;
    let y2_top = 1.3;
    let y2_bot = 2.0;

    let z_top = 1.5;
    let z_bot = -0.5;

    let a_top = (y2_top - y1_top) / ((x_min - x_max) * (x_min - x_max));
    let b_top = -2.0 * a_top * x_max;
    let c_top = y1_top + a_top * x_max * x_max;

    let a_bot = (y2_bot - y1_bot) / ((x_min - x_max) * (x_min - x_max));
    let b_bot = -2.0 * a_bot * x_max;
    let c_bot = y1_bot + a_bot * x_max * x_max;

    for x_index in 0..n_x {
        // Get position in X and top and bottom Y position
        let x_pos = x_min + (x_max - x_min) * x_index as f32 / (n_x as f32 - 1.0);

        let y_pos_top = a_top * x_pos * x_pos + b_top * x_pos + c_top;
        let y_pos_bot = a_bot * x_pos * x_pos + b_bot * x_pos + c_bot;

        // Generate coefficients for YZ parabola
        let a_yz = (y_pos_bot - y_pos_top) / ((z_bot - z_top) * (z_bot - z_top));
        let b_yz = -2.0 * a_yz * z_top;
        let c_yz = y_pos_top + a_yz * z_top * z_top;

        for z_index in 0..n_z {
            let z_pos = z_bot + (z_top - z_bot) * z_index as f32 / (n_z as f32 - 1.0);
            let y_pos = a_yz * z_pos * z_pos + b_yz * z_pos + c_yz;

            model.add_vert(Vector3::new(x_pos, -y_pos, z_pos));

            // Add the line that joins to the previous one at the same x
            // slice
            if z_index != 0 {
                model.add_line
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                       z_index + x_index * n_z + index_offset]);
            }

            // Add the line that joins to the previous one at the same z
            // slice
            if x_index != 0 {
                model.add_line
                (&vec![z_index + (x_index - 1) * n_z + index_offset,
                       z_index + x_index * n_z + index_offset]);
            }

            // Add the faces corresponding to that vertex
            if z_index != 0 && x_index != 0 {
                model.add_face
                (&vec![z_index - 1 + x_index * n_z + index_offset,
                       z_index - 1 + (x_index - 1) * n_z + index_offset,
                       z_index + x_index * n_z + index_offset], 0.0);
                model.add_face
                (&vec![z_index - 1 + (x_index - 1) * n_z + index_offset,
                       z_index + (x_index - 1) * n_z + index_offset,
                       z_index + x_index * n_z + index_offset], 0.0);
            }
        }
    }

    // ---- STITCH THEM TOGETHER ----
    for x_index in 0..n_x {
        for z_index in 0..n_z {
            // Add lines
            if x_index == 0 || x_index == n_x - 1 ||
               z_index == 0 || z_index == n_z - 1 {
                model.add_line(&vec![z_index + x_index * n_z + index_offset,
                                     z_index + x_index * n_z + index_offset - n_x * n_z]);
            }

            // Add faces
            if x_index == 0 {
                if z_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index - 1 + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                    model.add_face(&vec![z_index - 1 + x_index * n_z + index_offset - n_x * n_z,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index - 1 + x_index * n_z + index_offset], 0.0);
                }
            }
            if x_index == n_x - 1 {
                if z_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index - 1 + x_index * n_z + index_offset], 0.0);
                    model.add_face(&vec![z_index - 1 + x_index * n_z + index_offset - n_x * n_z,
                                         z_index - 1 + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                }
            }
            if z_index == 0 {
                if x_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index + (x_index - 1) * n_z + index_offset], 0.0);
                    model.add_face(&vec![z_index + (x_index - 1) * n_z + index_offset - n_x * n_z,
                                         z_index + (x_index - 1) * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                }
            }
            if z_index == n_z - 1 {
                if x_index != 0 {
                    model.add_face(&vec![z_index + x_index * n_z + index_offset,
                                         z_index + (x_index - 1) * n_z + index_offset,
                                         z_index + x_index * n_z + index_offset - n_x * n_z], 0.0);
                    model.add_face(&vec![z_index + (x_index - 1) * n_z + index_offset - n_x * n_z,
                                         z_index + x_index * n_z + index_offset - n_x * n_z,
                                         z_index + (x_index - 1) * n_z + index_offset], 0.0);
                }
            }
        }
    }
}