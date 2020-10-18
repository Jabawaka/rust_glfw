extern crate glfw;

extern crate gl;
use gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;

use cgmath::{Matrix4, vec3, Deg, Rad, perspective, Point3, Vector3, Vector4};
use cgmath::prelude::*;

use super::window::*;

pub struct Camera {
    pos_glob: Point3<f32>,
    att_glob: Vector3<f32>,
    vel_loc: Vector3<f32>,
    rot_glob: Vector3<f32>,
    size: (u32, u32),
    vao: u32,
    framebuffer: u32,
    texture_color_buffer: u32,
    proj_mat: Matrix4<f32>,
    view_mat: Matrix4<f32>,
    pub total_mat: Matrix4<f32>,
}

const CAM_SPEED: f32 = 2.0;
const CAM_ROTATION: f32 = 0.7;

impl Camera {
    pub fn create(fov_deg: f32, render_size: (u32, u32), pos_glob: Point3<f32>, att_glob: Vector3<f32>) -> Camera {
        // Create empty object
        let mut cam = Camera {
            pos_glob: pos_glob,
            att_glob: att_glob,
            vel_loc: Vector3::zero(),
            rot_glob: Vector3::zero(),
            size: render_size,
            vao: 0,
            framebuffer: 0,
            texture_color_buffer: 0,
            proj_mat: Matrix4::identity(),
            view_mat: Matrix4::identity(),
            total_mat: Matrix4::identity()
        };

        // Calculate projection and view matrices
        cam.proj_mat = cam.proj_mat * perspective(Deg(fov_deg), render_size.0 as f32 / render_size.1 as f32, 0.1, 100.0);

        let att_mat: Matrix4<f32> =
            Matrix4::<f32>::from_angle_z(Rad(att_glob.z))
          * Matrix4::<f32>::from_angle_y(Rad(att_glob.y))
          * Matrix4::<f32>::from_angle_x(Rad(att_glob.x));
        let look_dir = att_mat * Vector4::new(1.0, 0.0, 0.0, 0.0);
        let cam_target = Point3::new(pos_glob.x + look_dir.x, pos_glob.y + look_dir.y, pos_glob.z + look_dir.z);

        cam.view_mat = Matrix4::look_at(pos_glob, cam_target, vec3(0.0, 0.0, 1.0));

        cam.total_mat = cam.proj_mat * cam.view_mat;

        // Generate VAO for the screen quad
        unsafe {
            let mut vbo = 0;

            let vertices: [f32; 24] = [
                -1.0,  1.0,  0.0, 1.0,
                -1.0, -1.0,  0.0, 0.0,
                 1.0, -1.0,  1.0, 0.0,
    
                -1.0,  1.0,  0.0, 1.0,
                 1.0, -1.0,  1.0, 0.0,
                 1.0,  1.0,  1.0, 1.0
            ];

            gl::GenVertexArrays(1, &mut cam.vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(cam.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData
               (gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(0);
            let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
            gl::VertexAttribPointer
               (0, 4, gl::FLOAT, gl::FALSE,
                stride, ptr::null());

            // Generate framebuffer object
            gl::GenFramebuffers(1, &mut cam.framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, cam.framebuffer);
            
            gl::GenTextures(1, &mut cam.texture_color_buffer);
            gl::BindTexture(gl::TEXTURE_2D, cam.texture_color_buffer);
            gl::TexImage2D
               (gl::TEXTURE_2D, 0, gl::RGB as i32,
                render_size.0 as i32, render_size.1 as i32, 0,
                gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
            gl::TexParameteri
               (gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri
               (gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::FramebufferTexture2D
               (gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D, cam.texture_color_buffer, 0);
            
            let mut rbo = 0;
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage
               (gl::RENDERBUFFER, gl::DEPTH24_STENCIL8,
                render_size.0 as i32, render_size.1 as i32);
            gl::FramebufferRenderbuffer
               (gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER, rbo);
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) !=
               gl::FRAMEBUFFER_COMPLETE {
                println!("[ERR] Framebuffer is not complete!");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        cam
    }

    pub fn process_input(&mut self, window: &Window) {
        self.vel_loc = Vector3::zero();
        self.rot_glob = Vector3::zero();

        // Update speed according to input
        if window.is_input_down(InputAction::MoveForward) {
            self.vel_loc.x += 1.0;
        }
        if window.is_input_down(InputAction::MoveBack) {
            self.vel_loc.x -= 1.0;
        }
        if window.is_input_down(InputAction::MoveLeft) {
            self.vel_loc.y += 1.0;
        }
        if window.is_input_down(InputAction::MoveRight) {
            self.vel_loc.y -= 1.0;
        }

        // Reset views to predefined ones
        if window.is_input_down(InputAction::ViewFrontX) {
            self.pos_glob = Point3::new(3.0, 0.0, 0.0);
            self.att_glob = Vector3::new(0.0, 0.0, std::f64::consts::PI as f32);
        }
        if window.is_input_down(InputAction::ViewRearX) {
            self.pos_glob = Point3::new(-3.0, 0.0, 0.0);
            self.att_glob = Vector3::zero();
        }
        if window.is_input_down(InputAction::ViewLeftY) {
            self.pos_glob = Point3::new(0.0, 3.0, 0.0);
            self.att_glob = Vector3::new(0.0, 0.0, -std::f64::consts::PI as f32 / 2.0);
        }
        if window.is_input_down(InputAction::ViewRightY) {
            self.pos_glob = Point3::new(0.0, -3.0, 0.0);
            self.att_glob = Vector3::new(0.0, 0.0, std::f64::consts::PI as f32 / 2.0);
        }
        if window.is_input_down(InputAction::ViewTopZ) {
            self.pos_glob = Point3::new(0.0, 0.0, 3.0);
            self.att_glob = Vector3::new(0.0, std::f64::consts::PI as f32 / 2.0 - 0.01 , 0.0);
        }
        if window.is_input_down(InputAction::ViewBotZ) {
            self.pos_glob = Point3::new(0.0, 0.0, -3.0);
            self.att_glob = Vector3::new(0.0, -std::f64::consts::PI as f32 / 2.0 + 0.01, 0.0);
        }

        // Update rotation according to input
        if window.is_input_down(InputAction::RotLeft) {
            self.rot_glob.z += 1.0;
        }
        if window.is_input_down(InputAction::RotRight) {
            self.rot_glob.z -= 1.0;
        }
        if window.is_input_down(InputAction::RotUp) {
            self.rot_glob.y -= 1.0;
        }
        if window.is_input_down(InputAction::RotDown) {
            self.rot_glob.y += 1.0;
        }

        // Normalise and apply magnitude
        if self.vel_loc.magnitude() > 0.0 {
            self.vel_loc = CAM_SPEED * self.vel_loc.normalize();
        }
        if self.rot_glob.magnitude() > 0.0 {
            self.rot_glob = CAM_ROTATION * self.rot_glob.normalize();
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update angles
        self.att_glob += self.rot_glob * delta_time;
        if self.att_glob.y > std::f64::consts::PI as f32 / 2.0 {
            self.att_glob.y = std::f64::consts::PI as f32 / 2.0 - 0.01;
        }
        if self.att_glob.y < -std::f64::consts::PI as f32 / 2.0 {
            self.att_glob.y = -std::f64::consts::PI as f32 / 2.0 + 0.01;
        }
        if self.att_glob.z > std::f64::consts::PI as f32 {
            self.att_glob.z -= 2.0 * std::f64::consts::PI as f32;
        }
        if self.att_glob.z < -std::f64::consts::PI as f32 {
            self.att_glob.z += 2.0 * std::f64::consts::PI as f32;
        }

        let att_mat: Matrix4<f32> =
            Matrix4::<f32>::from_angle_z(Rad(self.att_glob.z))
          * Matrix4::<f32>::from_angle_y(Rad(self.att_glob.y))
          * Matrix4::<f32>::from_angle_x(Rad(self.att_glob.x));

        // Rotate velocity into global frame and update position
        let vel_glob = (att_mat * self.vel_loc.extend(0.0)).truncate();
        self.pos_glob += vel_glob * delta_time;

        // Set view matrix to look ahead
        let look_dir = att_mat * Vector4::new(1.0, 0.0, 0.0, 0.0);
        let cam_target = self.pos_glob + look_dir.truncate();

        self.view_mat = Matrix4::look_at
           (self.pos_glob, cam_target, vec3(0.0, 0.0, 1.0));

        self.total_mat = self.proj_mat * self.view_mat;
    }

    pub fn activate(&self) {
        unsafe {
            gl::Viewport(0, 0, self.size.0 as i32, self.size.1 as i32);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);

            gl::ClearColor(0.1, 0.1, 0.1, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_color_buffer);

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
        }
    }
}
