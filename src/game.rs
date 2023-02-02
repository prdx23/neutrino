
use crate::{ Vec3, Matrix4 };


// #[derive(Clone, Copy)]
pub struct Object3d {
    pub position: Vec3,
    pub matrix: Matrix4,
}


pub struct Game {
    objects: Vec<Object3d>,
    buffer: Vec<f32>,
}


impl Game {

    pub fn init() -> Self {
        Self {
            buffer: vec![],
            objects: vec![
                Object3d {
                    position: Vec3::zero(),
                    matrix: Matrix4::identity(),
                },

                Object3d {
                    position: Vec3::zero(),
                    matrix: Matrix4::identity(),
                },

                Object3d {
                    position: Vec3::zero(),
                    matrix: Matrix4::identity(),
                },
            ]
        }
    }

    pub fn test(ptr: *mut Game, cw: f32, ch: f32) -> *const f32 {
        let game = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        let mut camera_matrix = Matrix4::identity();
        camera_matrix.translate(0.0, 300.0, 1800.0);

        camera_matrix = Matrix4::look_at(
            Vec3::new(
                camera_matrix.matrix[3][0],
                camera_matrix.matrix[3][1],
                camera_matrix.matrix[3][2]),
            Vec3::zero(),
            Vec3::new(0.0, 1.0, 0.0)
        );

        let view_matrix = camera_matrix.inverse();

        let projection_matrix = Matrix4::perspective(
            30.0, cw / ch, 1.0, 2000.0
        );

        let view_projection_matrix = projection_matrix * view_matrix;

        // crate::console_print(format!("cw {}", cw).as_str());
        // crate::console_print(format!("ch {}", ch).as_str());
        view_projection_matrix.add_bytes_to_buffer(&mut game.buffer);

        // for x in &game.objects {
        //     crate::console_print(format!("{}", x.test).as_str());
        // }

        game.buffer.as_ptr()
    }
}

