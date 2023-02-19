
mod engine;
mod math;
mod utils;

use crate::engine::{Engine, Camera, Tree};
use crate::math::{Vec3, Matrix4};
use crate::utils::{MemoryBuffer};


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

    let mut scenegraph = Tree::empty();
    let scene = scenegraph.root();

    let cube1 = scenegraph.add_object(scene, Some(
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
        }"#),
    );
    scenegraph[cube1].scale.set(50.0, 50.0, 50.0);


    let cube2 = scenegraph.add_object(scene, Some(
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
        }"#),
    );
    scenegraph[cube2].scale.set(50.0, 50.0, 50.0);
    scenegraph[cube2].position.x = 250.0;

    let cube3 = scenegraph.add_object(scene, Some(
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
        }"#),
    );
    scenegraph[cube3].scale.set(50.0, 50.0, 50.0);
    scenegraph[cube3].position.x = -250.0;

    let engine = Engine {
        camera: Camera::perspective(
            Vec3::new(0.0, 1000.0, 1000.0),
            30.0, 1.0, 500.0, 8000.0
        ),
        buffer: MemoryBuffer::empty(),
        scenegraph: scenegraph,
    };
    Box::into_raw(Box::new(engine))
}



#[no_mangle]
pub extern fn render(ptr: *mut Engine, t: f32, keys: u8) -> *const f32 {
    let engine = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    engine.buffer.reset();


    if keys & (1 << 0) > 0 {
        engine.scenegraph[1].position.z -= 5.0;
    }
    if keys & (1 << 1) > 0 {
        engine.scenegraph[1].position.x -= 5.0;
    }
    if keys & (1 << 2) > 0 {
        engine.scenegraph[1].position.z += 5.0;
    }
    if keys & (1 << 3) > 0 {
        engine.scenegraph[1].position.x += 5.0;
    }
    engine.camera.look_at(engine.scenegraph[1].position);


    engine.scenegraph[0].position.y = (t / 100.0).sin() * 100.0;

    for i in 1..=3 {
        engine.scenegraph[i].rotation.x = -t * 0.5 * (i as f32 + 1.0);
        engine.scenegraph[i].rotation.y = -t * 0.5 * (i as f32 + 1.0);
    }

    engine.scenegraph.update_matrices(
        engine.camera.view_projection_matrix(),
        &mut engine.buffer,
    );

    engine.buffer.as_ptr()
}
