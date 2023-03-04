
use crate::engine::{Camera, Tree, BoundingBox};
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

        let ship = scenegraph.add_object(scene,
            Some(r#"{
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
        scenegraph[ship].add_bbox(BoundingBox::new(10.0, 10.0));


        let asteroid_meta = r#"{
            "shader": "vertex_color",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix", "u_collide"]
            }
        }"#;

        let mut asteroids = Arena::empty();

        let temp = scenegraph.add_object(scene, Some(asteroid_meta));
        scenegraph[temp].scale.set(280.0, 280.0, 280.0);
        scenegraph[temp].position.set(200.0, -1000.0, 0.0);
        scenegraph[temp].rotation.x = 45.0;
        scenegraph[temp].rotation.y = 45.0;

        let temp = scenegraph.add_object(scene, Some(asteroid_meta));
        scenegraph[temp].scale.set(20.0, 20.0, 20.0);
        scenegraph[temp].position.set(20.0, -300.0, -100.0);
        scenegraph[temp].rotation.x = 15.0;
        scenegraph[temp].rotation.y = 25.0;

        let temp = scenegraph.add_object(scene, Some(asteroid_meta));
        scenegraph[temp].scale.set(15.0, 15.0, 15.0);
        scenegraph[temp].position.set(100.0, 100.0, -140.0);
        scenegraph[temp].rotation.x = 15.0;
        scenegraph[temp].rotation.y = 25.0;


        let testmeta = r#"{
            "shader": "test",
            "count": 6,
            "attributes": {
                "a_position": "quad"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#;

        for i in 0..5 {
            let asteroid = scenegraph.add_object(scene, Some(asteroid_meta));
            scenegraph[asteroid].scale.set(5.0, 5.0, 5.0);
            scenegraph[asteroid].position.set(-100.0, 0.0, -100.0);
            scenegraph[asteroid].position.x += 40.0 * i as f32;
            scenegraph[asteroid].add_bbox(BoundingBox::new(12.0, 12.0));
            asteroids.add(asteroid);

            let test = scenegraph.add_object(scene, Some(testmeta));
            scenegraph[test].scale.set(12.0, 12.0, 12.0);
            scenegraph[test].position = scenegraph[asteroid].position;
        }



        Self {
            scenegraph, scene,
            ship, asteroids,
        }
    }


    pub fn render_frame(
        game: &mut Game, t: f32, keys: u8,
        camera: &mut Camera, buffer: &mut MemoryBuffer
    ) {

        for asteroid_id in game.asteroids.slice() {
            if game.scenegraph.collide(game.ship, *asteroid_id) {
                buffer.add_float(*asteroid_id as f32, 0.0, 1.0, 1.0);
            } else {
                buffer.add_float(*asteroid_id as f32, 0.0, 1.0, 0.0);
            }
        }

        let mut ship = &mut game.scenegraph[game.ship];
        if keys & (1 << 0) > 0 { ship.position.z -= 0.4; }
        if keys & (1 << 1) > 0 { ship.position.x -= 0.4; }
        if keys & (1 << 2) > 0 { ship.position.z += 0.4; }
        if keys & (1 << 3) > 0 { ship.position.x += 0.4; }

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
