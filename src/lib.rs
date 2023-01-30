// use wasm_bindgen::prelude::*;
use std::panic;


#[link(wasm_import_module = "imports")]
extern {
    fn console_log(x: *const u8, l: usize);
    fn console_error(x: *const u8, l: usize);
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let mut msg = "Panic occurred".to_string();

        if let Some(location) = panic_info.location() {
            msg.push_str(format!(" in file '{}' at line {}",
                location.file(),
                location.line(),
            ).as_str());
        }

        if let Some(e) = panic_info.payload().downcast_ref::<&str>() {
            msg.push_str(format!(": {}", e).as_str());
        }

        unsafe {
            console_error(msg.as_ptr(), msg.len());
        }
    }));
}


fn console_print(text: &str) {
    unsafe {
        console_log(text.as_ptr(), text.len());
    }
}



#[no_mangle]
pub extern fn test() -> f32 {
    set_panic_hook();
    console_print(format!("this is some string! {}", 123).as_str());
    panic!("panic!");
    1234.0
}


// #[wasm_bindgen]
// // #[derive(Clone, Copy)]
// pub struct Object3d {
//     pub test: f32,
// }


// // #[wasm_bindgen]
// #[wasm_bindgen(getter_with_clone)]
// pub struct Game {
//     objects: Vec<Object3d>,
// }


// #[wasm_bindgen]
// impl Game {

//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Self {
//         Self { objects: vec![] }
//     }

//     pub fn init(&mut self) {

//         self.objects = vec![
//             Object3d {
//                 test: 1.0
//             },

//             Object3d {
//                 test: 2.0
//             },

//             Object3d {
//                 test: 3.0
//             },
//         ]

//     }

//     pub fn get_object(&self, id: usize) -> f32 {
//         self.objects[id].test
//     }

//     pub fn test(self) -> *const Object3d {
//         self.objects.as_ptr()
//     }
// }
