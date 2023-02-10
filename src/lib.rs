
mod engine;
mod math;
mod utils;

use crate::engine::{Engine, Camera, Object3d};
use crate::math::{Vec3, Matrix4};
use crate::utils::{MemoryBuffer, Arena};


#[link(wasm_import_module = "imports")]
extern {

    fn console_log_raw(x: *const u8, l: usize);

    fn console_error_raw(x: *const u8, l: usize);

    fn add_shader(
        name_ptr: *const u8, name_len: usize,
        vert_ptr: *const u8, vert_len: usize,
        frag_ptr: *const u8, frag_len: usize,
    );

    fn add_buffer_float(
        name_ptr: *const u8, name_len: usize,
        data_ptr: *const u8, data_len: usize,
        size: f32, normalize: bool
    );

    fn add_buffer_bytes(
        name_ptr: *const u8, name_len: usize,
        data_ptr: *const u8, data_len: usize,
        size: f32, normalize: bool
    );

    fn add_object(id: usize, ptr: *const u8, len: usize);

}


#[no_mangle]
pub extern fn init() -> *mut Engine {

    utils::set_panic_hook();

    add_shader!(main);
    add_shader!(cube);

    add_buffer!(float, cube_vertices, 3.0, false);
    add_buffer!(bytes, cube_vertex_colors, 3.0, true);

    let mut engine = Engine {
        camera: Camera::perspective(
            Vec3::new(0.0, 1000.0, 1000.0),
            30.0, 1.0, 500.0, 8000.0
        ),
        buffer: MemoryBuffer::empty(),
        objects: Arena::empty(),
    };

    engine.add_object(
        Object3d {
            position: Vec3::zero(),
            scale: Vec3::new(50.0, 50.0, 50.0),
            rotation: Vec3::zero(),
        },
        r#"{
            "shader": "main",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#,
    );
    engine.add_object(
        Object3d {
            position: Vec3::new(250.0, 0.0, 0.0),
            scale: Vec3::new(50.0, 50.0, 50.0),
            rotation: Vec3::zero(),
        },
        r#"{
            "shader": "main",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#,
    );
    engine.add_object(
        Object3d {
            position: Vec3::new(-250.0, 0.0, 0.0),
            scale: Vec3::new(50.0, 50.0, 50.0),
            rotation: Vec3::zero(),
        },
        r#"{
            "shader": "cube",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#,
    );

    Box::into_raw(Box::new(engine))
}



#[no_mangle]
pub extern fn render(ptr: *mut Engine, t: f32, keys: u8) -> *const f32 {
    let engine = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    if keys & (1 << 0) > 0 {
        engine.objects.get_mut(0).position.z -= 5.0;
    }

    if keys & (1 << 1) > 0 {
        engine.objects.get_mut(0).position.x -= 5.0;
    }

    if keys & (1 << 2) > 0 {
        engine.objects.get_mut(0).position.z += 5.0;
    }

    if keys & (1 << 3) > 0 {
        engine.objects.get_mut(0).position.x += 5.0;
    }

    engine.buffer.reset();

    engine.camera.look_at(engine.objects.get(0).position);

    let view_projection_matrix = engine.camera.view_projection_matrix();

    for (i, obj) in engine.objects.iter_mut().enumerate() {
        // utils::console_log(format!("id {}", obj.id).as_str());
        engine.buffer.add_f32(i as f32);
        engine.buffer.add_f32(16.0);
        engine.buffer.add_f32(0.0);
        engine.buffer.add_f32(0.0);

        obj.rotation.x = -t * 0.5 * (i as f32 + 1.0);
        obj.rotation.y = -t * 0.5 * (i as f32 + 1.0);
        engine.buffer.add_matrix(&obj.get_matrix(view_projection_matrix));
    }

    engine.buffer.as_ptr()
}
