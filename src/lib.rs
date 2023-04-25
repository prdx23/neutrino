
mod engine;
mod math;
mod physics;
mod utils;
mod game;

use crate::engine::{Engine, Camera, Frame};
use crate::math::{Vec3, PI};
use crate::game::{Game};


#[link(wasm_import_module = "imports")]
extern {

    fn js_console_log_raw(x: *const u8, l: usize);

    fn js_console_error_raw(x: *const u8, l: usize);

    fn js_add_shader(
        name_ptr: *const u8, name_len: usize,
        vert_ptr: *const u8, vert_len: usize,
        frag_ptr: *const u8, frag_len: usize,
    );

    fn js_add_buffer_float(
        name_ptr: *const u8, name_len: usize,
        data_ptr: *const u8, data_len: usize,
        size: f32, normalize: bool
    );

    fn js_add_buffer_bytes(
        name_ptr: *const u8, name_len: usize,
        data_ptr: *const u8, data_len: usize,
        size: f32, normalize: bool
    );

    fn js_add_entity(ptr: *const u8, len: usize) -> usize;

    // fn js_destroy_entity(id: usize);

}



#[no_mangle]
pub extern fn init() -> *mut Engine {

    utils::set_panic_hook();

    add_shader!(vertex_color);
    add_shader!(test);

    add_buffer!(float, cube_vertices, 3.0, false);
    add_buffer!(float, ship_vertices, 3.0, false);
    add_buffer!(bytes, cube_vertex_colors, 3.0, true);
    add_buffer!(float, quad, 3.0, false);

    let engine = Engine {
        camera: Camera::perspective(
            Vec3::new(0.0, 300.0, 0.1),
            25.0, 1.0, 1.0, 4000.0
        ),
        frame: Frame::new(),
        game: Game::new(),
    };

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

    engine.frame.update(
        t, dt, keys, engine.camera.view_projection_matrix()
    );

    engine.game.render_frame(&mut engine.frame, &mut engine.camera);

    engine.frame.buffer.buffer_as_ptr()
}
