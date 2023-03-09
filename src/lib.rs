
mod engine;
mod math;
mod physics;
mod utils;
mod game;

use crate::engine::{Engine, Camera};
use crate::math::{Vec3, Matrix4, PI};
use crate::utils::{MemoryBuffer};
use crate::game::{Game};


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

    add_shader!(vertex_color);
    add_shader!(test);

    add_buffer!(float, cube_vertices, 3.0, false);
    add_buffer!(bytes, cube_vertex_colors, 3.0, true);
    add_buffer!(float, quad, 3.0, false);

    let mut engine = Engine {
        camera: Camera::perspective(
            Vec3::new(0.0, 300.0, 0.1),
            25.0, 1.0, 1.0, 4000.0
        ),
        buffer: MemoryBuffer::empty(),
        game: Game::init_scenegraph(),
    };

    engine.game.scenegraph.update(
        0.0,
        engine.camera.view_projection_matrix(),
        &mut engine.buffer,
    );

    Box::into_raw(Box::new(engine))
}



#[no_mangle]
pub extern fn render(
    ptr: *mut Engine, t: f32, dt: f32, keys: u8
) -> *const f32 {

    let engine = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    engine.buffer.reset();

    Game::render_frame(
        &mut engine.game, t, dt, keys, &mut engine.camera, &mut engine.buffer
    );

    engine.game.scenegraph.update(
        dt,
        engine.camera.view_projection_matrix(),
        &mut engine.buffer,
    );

    engine.buffer.as_ptr()
}
