extern crate gl;
use self::gl::types::*;


pub struct Model {
    solid_index: u32,
    wireframe_index: u32
}

impl Model {
    fn create() -> Model {
        let model = Model;

        model
    }

    fn create_default(vao: u32) -> Model {
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
    }
}