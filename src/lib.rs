
mod engine;
mod math;
mod utils;

use crate::engine::{Engine, Camera, Object3d};
use crate::math::{Vec3, Matrix4};
use crate::utils::{MemoryBuffer, Arena};
use crate::engine::{Tree};


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

    pub fn temp(id: usize, meta: &str) {
        unsafe { crate::add_object(id, meta.as_ptr(), meta.len()); }
    }


#[no_mangle]
pub extern fn init() -> *mut Engine {

    utils::set_panic_hook();

    add_shader!(main);
    add_shader!(cube);

    add_buffer!(float, cube_vertices, 3.0, false);
    add_buffer!(bytes, cube_vertex_colors, 3.0, true);

    let mut tree = Tree::empty();
    let scene = tree.new_root();

    let cube1 = tree.new_node(scene);
    tree.node(cube1).matrix.scale_x(50.0);
    tree.node(cube1).matrix.scale_y(50.0);
    tree.node(cube1).matrix.scale_z(50.0);
    temp(cube1,
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


    let cube2 = tree.new_node(scene);
    tree.node(cube2).matrix.scale_x(50.0);
    tree.node(cube2).matrix.scale_y(50.0);
    tree.node(cube2).matrix.scale_z(50.0);
    tree.node(cube2).matrix.translate_x(250.0);
    temp(cube2,
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

    let cube3 = tree.new_node(scene);
    tree.node(cube3).matrix.scale_x(50.0);
    tree.node(cube3).matrix.scale_y(50.0);
    tree.node(cube3).matrix.scale_z(50.0);
    tree.node(cube3).matrix.translate_x(-250.0);
    temp(cube3,
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

    let mut engine = Engine {
        camera: Camera::perspective(
            Vec3::new(0.0, 1000.0, 1000.0),
            30.0, 1.0, 500.0, 8000.0
        ),
        buffer: MemoryBuffer::empty(),
        tree: tree,
    };








    // let mut tree: Arena<Node, NOBJ> = Arena::empty();

    // let mut scene = Node::new();

    // let mut cube1 = Node::new();
    // cube1.matrix.scale(Vec3::new(50.0, 50.0, 50.0));

    // let cube1id = tree.add(cube1);


    // scene.children.add(cube1id);


    // let sceneid = tree.add(scene);
    // // tree.arena[scene].children.add(cube1);


    // // utils::console_log(format!("tree {:?}", tree.arena).as_str());
    // tree = scene.update_world_matrix(Matrix4::identity(), tree);
    // utils::console_log(format!("tree {:?}", tree.arena).as_str());




    // engine.add_object(
    //     Object3d {
    //         position: Vec3::zero(),
    //         scale: Vec3::new(50.0, 50.0, 50.0),
    //         rotation: Vec3::zero(),
    //     },
    //     r#"{
    //         "shader": "main",
    //         "count": 36,
    //         "attributes": {
    //             "a_position": "cube_vertices",
    //             "a_color": "cube_vertex_colors"
    //         },
    //         "uniforms": {
    //             "objectData": ["u_matrix"]
    //         }
    //     }"#,
    // );
    // engine.add_object(
    //     Object3d {
    //         position: Vec3::new(250.0, 0.0, 0.0),
    //         scale: Vec3::new(50.0, 50.0, 50.0),
    //         rotation: Vec3::zero(),
    //     },
    //     r#"{
    //         "shader": "main",
    //         "count": 36,
    //         "attributes": {
    //             "a_position": "cube_vertices",
    //             "a_color": "cube_vertex_colors"
    //         },
    //         "uniforms": {
    //             "objectData": ["u_matrix"]
    //         }
    //     }"#,
    // );
    // engine.add_object(
    //     Object3d {
    //         position: Vec3::new(-250.0, 0.0, 0.0),
    //         scale: Vec3::new(50.0, 50.0, 50.0),
    //         rotation: Vec3::zero(),
    //     },
    //     r#"{
    //         "shader": "cube",
    //         "count": 36,
    //         "attributes": {
    //             "a_position": "cube_vertices",
    //             "a_color": "cube_vertex_colors"
    //         },
    //         "uniforms": {
    //             "objectData": ["u_matrix"]
    //         }
    //     }"#,
    // );

    Box::into_raw(Box::new(engine))
}



#[no_mangle]
pub extern fn render(ptr: *mut Engine, t: f32, keys: u8) -> *const f32 {
    let engine = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    // if keys & (1 << 0) > 0 {
    //     engine.objects.get_mut(0).position.z -= 5.0;
    // }

    // if keys & (1 << 1) > 0 {
    //     engine.objects.get_mut(0).position.x -= 5.0;
    // }

    // if keys & (1 << 2) > 0 {
    //     engine.objects.get_mut(0).position.z += 5.0;
    // }

    // if keys & (1 << 3) > 0 {
    //     engine.objects.get_mut(0).position.x += 5.0;
    // }

    engine.buffer.reset();

    // engine.camera.look_at(engine.objects.get(0).position);

    let view_projection_matrix = engine.camera.view_projection_matrix();

    engine.tree.node(0).matrix.translate_y((t / 10.0).sin() * 10.0);

    // engine.tree.node(0).matrix.scale_x(0.5 * 1.0);

    engine.tree.node(1).matrix.translate_x(0.0);
    engine.tree.node(2).matrix.translate_x(-250.0);
    engine.tree.node(3).matrix.translate_x(250.0);

    engine.tree.node(1).matrix.rotate_x(0.5 * 1.0);
    engine.tree.node(2).matrix.rotate_x(0.5 * 2.0);
    engine.tree.node(3).matrix.rotate_x(0.5 * 3.0);

    engine.tree.node(1).matrix.rotate_y(0.5 * 1.0);
    engine.tree.node(2).matrix.rotate_y(0.5 * 2.0);
    engine.tree.node(3).matrix.rotate_y(0.5 * 3.0);

    engine.tree.node(1).matrix.translate_x(0.0);
    engine.tree.node(2).matrix.translate_x(250.0);
    engine.tree.node(3).matrix.translate_x(-250.0);

    engine.tree.update_world_matrix(0, view_projection_matrix);

    engine.buffer.add_f32(1.0);
    engine.buffer.add_f32(16.0);
    engine.buffer.add_f32(0.0);
    engine.buffer.add_f32(0.0);
    // let matrix = view_projection_matrix * engine.tree.node(1).matrix;
    // engine.buffer.add_matrix(&matrix);
    engine.buffer.add_matrix(&engine.tree.node(1).world_matrix);


    engine.buffer.add_f32(2.0);
    engine.buffer.add_f32(16.0);
    engine.buffer.add_f32(0.0);
    engine.buffer.add_f32(0.0);
    // let matrix = view_projection_matrix * engine.tree.node(2).matrix;
    // engine.buffer.add_matrix(&matrix);
    engine.buffer.add_matrix(&engine.tree.node(2).world_matrix);


    engine.buffer.add_f32(3.0);
    engine.buffer.add_f32(16.0);
    engine.buffer.add_f32(0.0);
    engine.buffer.add_f32(0.0);
    // let matrix = view_projection_matrix * engine.tree.node(3).matrix;
    // engine.buffer.add_matrix(&matrix);
    engine.buffer.add_matrix(&engine.tree.node(3).world_matrix);

    // for (i, obj) in engine.objects.iter_mut().enumerate() {
    //     // utils::console_log(format!("id {}", obj.id).as_str());
    //     engine.buffer.add_f32(i as f32);
    //     engine.buffer.add_f32(16.0);
    //     engine.buffer.add_f32(0.0);
    //     engine.buffer.add_f32(0.0);

    //     obj.rotation.x = -t * 0.5 * (i as f32 + 1.0);
    //     obj.rotation.y = -t * 0.5 * (i as f32 + 1.0);

    //     let matrix = view_projection_matrix * obj.matrix();
    //     engine.buffer.add_matrix(&matrix);
    // }

    engine.buffer.as_ptr()
}
