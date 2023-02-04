
use crate::{ Vec3, Matrix4 };


// #[derive(Clone, Copy)]
pub struct Object3d {
    pub id: f32,
    pub position: Vec3,
}

impl Object3d {
    pub fn update(&self, t: f32, view_projection_matrix: Matrix4) -> Matrix4 {
        // crate::console_print(format!("x {}", t).as_str());

        let mut matrix = Matrix4::identity();
        // crate::console_print(format!("matrix {:?}", matrix).as_str());
        matrix.rotate_x(-t * 0.5 * self.id);
        matrix.rotate_y(-t * 0.5 * self.id);
        matrix.scale(50.0, 50.0, 50.0);
        matrix.translate_vec(self.position);
        view_projection_matrix * matrix
    }
}


pub struct Game {
    pub objects: Vec<Object3d>,
    pub buffer: Vec<f32>,
}


impl Game {

    pub fn init() -> Self {
        Self {
            buffer: vec![],
            objects: vec![
                Object3d {
                    id: 1.0,
                    position: Vec3::zero(),
                },

                Object3d {
                    id: 2.0,
                    position: Vec3::new(250.0, 0.0, 0.0),
                },

                Object3d {
                    id: 3.0,
                    position: Vec3::new(-250.0, 0.0, 0.0),
                },
            ]
        }
    }

}

