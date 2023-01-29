use wasm_bindgen::prelude::*;


#[wasm_bindgen]
// #[derive(Clone, Copy)]
pub struct Object3d {
    pub test: f32,
}


// #[wasm_bindgen]
#[wasm_bindgen(getter_with_clone)]
pub struct Game {
    objects: Vec<Object3d>,
}


#[wasm_bindgen]
impl Game {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn init(&mut self) {

        self.objects = vec![
            Object3d {
                test: 1.0
            },

            Object3d {
                test: 2.0
            },

            Object3d {
                test: 3.0
            },
        ]

    }

    pub fn get_object(&self, id: usize) -> f32 {
        self.objects[id].test
    }

    pub fn test(self) -> *const Object3d {
        self.objects.as_ptr()
    }
}
