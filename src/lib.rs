use std::panic;


mod game;
mod math;
mod utils;

use crate::game::{Game, Object3d, Camera};
use crate::math::{Vec3, Matrix4};
use crate::utils::{MemoryBuffer, ObjectArray};


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
    let mut game = Game {
        camera: Camera::perspective(
            Vec3::new(0.0, 300.0, 1800.0),
            30.0, 1.0, 1.0, 2500.0
        ),
        buffer: MemoryBuffer::empty(),
        objects: ObjectArray::empty(),
    };

    game.objects.add_object(Object3d {
        id: 1.0,
        position: Vec3::zero(),
        scale: Vec3::new(50.0, 50.0, 50.0),
        rotation: Vec3::zero(),
    });
    game.objects.add_object(Object3d {
        id: 2.0,
        position: Vec3::new(250.0, 0.0, 0.0),
        scale: Vec3::new(50.0, 50.0, 50.0),
        rotation: Vec3::zero(),
    });
    game.objects.add_object(Object3d {
        id: 3.0,
        position: Vec3::new(-250.0, 0.0, 0.0),
        scale: Vec3::new(50.0, 50.0, 50.0),
        rotation: Vec3::zero(),
    });

    Box::into_raw(Box::new(game))
}

#[no_mangle]
pub extern fn render(ptr: *mut Game, t: f32) -> *const f32 {
    let game = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    game.buffer.reset();

    let view_projection_matrix = game.camera.view_projection_matrix();
    // game.buffer.add_matrix(&view_projection_matrix);

    for x in game.objects.iter_mut() {
        // console_print(format!("id {}", x.id).as_str());
        game.buffer.add_f32(x.id);
        game.buffer.add_f32(16.0);
        game.buffer.add_f32(0.0);
        game.buffer.add_f32(0.0);

        x.rotation.x = -t * 0.5 * x.id;
        x.rotation.y = -t * 0.5 * x.id;
        game.buffer.add_matrix(&x.get_matrix(view_projection_matrix));
    }

    game.buffer.as_ptr()
}
