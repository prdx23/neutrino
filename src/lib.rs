use std::panic;


mod engine;
mod math;
mod utils;

use crate::engine::{Engine, Camera};
use crate::math::{Vec3, Matrix4};
use crate::utils::{MemoryBuffer, ObjectArray};


#[link(wasm_import_module = "imports")]
extern {
    fn console_log(x: *const u8, l: usize);
    fn console_error(x: *const u8, l: usize);

    fn add_object(id: usize, ptr: *const u8, len: usize);
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
pub extern fn init() -> *mut Engine {

    set_panic_hook();

    let mut engine = Engine {
        camera: Camera::perspective(
            Vec3::new(0.0, 300.0, 1800.0),
            30.0, 1.0, 1.0, 2500.0
        ),
        buffer: MemoryBuffer::empty(),
        objects: ObjectArray::empty(),
    };

    engine.objects.add_object(
        Vec3::zero(),
        Vec3::new(50.0, 50.0, 50.0),
        Vec3::zero(),
        r#"{
            "shader": "main",
            "count": 36,
            "attributes": {
                "a_position": "cubeVertices",
                "a_color": "cubeColors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#,
    );
    engine.objects.add_object(
        Vec3::new(250.0, 0.0, 0.0),
        Vec3::new(50.0, 50.0, 50.0),
        Vec3::zero(),
        r#"{
            "shader": "main",
            "count": 36,
            "attributes": {
                "a_position": "cubeVertices",
                "a_color": "cubeColors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#,
    );
    engine.objects.add_object(
        Vec3::new(-250.0, 0.0, 0.0),
        Vec3::new(50.0, 50.0, 50.0),
        Vec3::zero(),
        r#"{
            "shader": "cube",
            "count": 36,
            "attributes": {
                "a_position": "cubeVertices",
                "a_color": "cubeColors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#,
    );

    Box::into_raw(Box::new(engine))
}

#[no_mangle]
pub extern fn render(ptr: *mut Engine, t: f32) -> *const f32 {
    let engine = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    engine.buffer.reset();

    let view_projection_matrix = engine.camera.view_projection_matrix();
    // engine.buffer.add_matrix(&view_projection_matrix);

    for obj in engine.objects.iter_mut() {
        // console_print(format!("id {}", obj.id).as_str());
        engine.buffer.add_f32(obj.id);
        engine.buffer.add_f32(16.0);
        engine.buffer.add_f32(0.0);
        engine.buffer.add_f32(0.0);

        obj.rotation.x = -t * 0.5 * (obj.id + 1.0);
        obj.rotation.y = -t * 0.5 * (obj.id + 1.0);
        engine.buffer.add_matrix(&obj.get_matrix(view_projection_matrix));
    }

    engine.buffer.as_ptr()
}
