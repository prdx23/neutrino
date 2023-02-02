use std::panic;


mod game;
mod math;

use crate::game::Game;
use crate::math::{Vec3, Matrix4};


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
pub extern fn init() -> *mut Game {
    set_panic_hook();
    Box::into_raw(Box::new(Game::init()))
}

#[no_mangle]
pub extern fn test(ptr: *mut Game, cw: f32, ch: f32) -> *const f32 {
    Game::test(ptr, cw, ch)
}
