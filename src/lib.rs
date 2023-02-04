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
pub extern fn render(ptr: *mut Game, t: f32, cw: f32, ch: f32) -> *const f32 {
    let game = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    game.buffer.clear();
    game.buffer.push(0.0);

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
        30.0, cw / ch, 1.0, 2500.0
    );

    let view_projection_matrix = projection_matrix * view_matrix;

    // crate::console_print(format!("x {}", t).as_str());
    // crate::console_print(format!("cw {}", cw).as_str());
    // crate::console_print(format!("ch {}", ch).as_str());
    view_projection_matrix.add_to_buffer(&mut game.buffer);

    for x in &game.objects {
        // crate::console_print(format!("{}", x.test).as_str());
        let matrix = x.update(t, view_projection_matrix);
        game.buffer.push(x.id);
        game.buffer.push(16.0);
        game.buffer.push(0.0);
        game.buffer.push(0.0);
        matrix.add_to_buffer(&mut game.buffer);
    }

    game.buffer[0] = game.buffer.len() as f32;
    game.buffer.as_ptr()
}
