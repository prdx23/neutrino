
// pub mod boundingbox;
// pub use boundingbox::BoundingBox;



use crate::engine::{Camera, Tree};
use crate::utils::{Arena, MemoryBuffer};


const NOBJECTS: usize = 20;
const NCHILDREN: usize = 20;


pub struct Game {
    pub scenegraph: Tree<NOBJECTS, NCHILDREN>,

    pub scene: usize,
    pub ship: usize,
    pub asteroids: Arena<usize, 10>,
}


impl Game {

    pub fn init_scenegraph() -> Self {
        let mut scenegraph = Tree::empty();
        let scene = scenegraph.root();

        let ship = scenegraph.add_object(scene, Some(
            r#"{
                "shader": "vertex_color",
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
        scenegraph[ship].scale.set(5.0, 5.0, 5.0);


        let asteroid_meta = r#"{
            "shader": "vertex_color",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#;

        let mut asteroids = Arena::empty();

        for i in 0..8 {
            let asteroid = scenegraph.add_object(scene, Some(asteroid_meta));
            scenegraph[asteroid].scale.set(5.0, 5.0, 5.0);
            scenegraph[asteroid].position.set(-100.0, 0.0, -100.0);
            scenegraph[asteroid].position.x += 40.0 * i as f32;
            asteroids.add(asteroid);
        }

        scenegraph[2].scale.set(280.0, 280.0, 280.0);
        scenegraph[2].position.set(200.0, -1000.0, 0.0);
        scenegraph[2].rotation.x = 45.0;
        scenegraph[2].rotation.y = 45.0;

        scenegraph[3].scale.set(20.0, 20.0, 20.0);
        scenegraph[3].position.set(20.0, -300.0, -100.0);
        scenegraph[3].rotation.x = 15.0;
        scenegraph[3].rotation.y = 25.0;

        scenegraph[4].scale.set(15.0, 15.0, 15.0);
        scenegraph[4].position.set(100.0, 100.0, -140.0);
        scenegraph[4].rotation.x = 15.0;
        scenegraph[4].rotation.y = 25.0;

        // let test = scenegraph.add_object(scene, Some(
        //     r#"{
        //         "shader": "test",
        //         "count": 6,
        //         "attributes": {
        //             "a_position": "quad"
        //         },
        //         "uniforms": {
        //             "objectData": ["u_matrix"]
        //         }
        //     }"#),
        // );
        // scenegraph[test].scale.set(6.0, 6.0, 6.0);
        // scenegraph[test].position = scenegraph[5].position;

        Self {
            scenegraph, scene,
            ship, asteroids,
        }
    }


    pub fn render_frame(
        game: &mut Game, t: f32, keys: u8,
        camera: &mut Camera, buffer: &mut MemoryBuffer
    ) {

        let mut ship = &mut game.scenegraph[game.ship];

        if keys & (1 << 0) > 0 { ship.position.z -= 1.0; }
        if keys & (1 << 1) > 0 { ship.position.x -= 1.0; }
        if keys & (1 << 2) > 0 { ship.position.z += 1.0; }
        if keys & (1 << 3) > 0 { ship.position.x += 1.0; }

        camera.position.x = ship.position.x;
        camera.position.z = ship.position.z + 0.1;
        camera.look_at(ship.position);
        // camera.look_at(game.scenegraph[1].position);


        game.scenegraph[3].rotation.x = -t * 0.5;
        game.scenegraph[3].rotation.y = -t * 0.5;

        game.scenegraph[4].rotation.x = -t * 0.5;
        game.scenegraph[4].rotation.y = -t * 0.5;
    }

}
