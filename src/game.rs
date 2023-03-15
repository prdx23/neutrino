
use crate::engine::{Camera, Node, Scenegraph, Arena, ArenaID, MemoryBuffer};


const NOBJECTS: usize = 20;
const NCHILDREN: usize = 20;


pub struct Game {
    pub scenegraph: Scenegraph<NOBJECTS, NCHILDREN>,

    pub scene: ArenaID,
    pub shipid: ArenaID,
    pub asteroid_ids: Arena<ArenaID, 10>,
}


impl Game {

    pub fn init_scenegraph() -> Self {
        let mut scenegraph = Scenegraph::empty();
        let scene = scenegraph.root();

        let mut ship = Node::new(Some(r#"{
            "shader": "vertex_color",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#));
        ship.scale.set(5.0, 5.0, 5.0);
        ship.rigidbody.enable(100.0, 2.0);
        ship.aabb.enable(10.0, 10.0);
        let shipid = scenegraph.add_object(scene, ship);


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


        let mut temp = Node::new(Some(asteroid_meta));
        temp.scale.set(280.0, 280.0, 280.0);
        temp.position.set(200.0, -1000.0, 0.0);
        temp.rotation.x = 45.0 * crate::PI / 180.0;
        temp.rotation.y = 45.0 * crate::PI / 180.0;
        scenegraph.add_object(scene, temp);

        let mut temp = Node::new(Some(asteroid_meta));
        temp.scale.set(20.0, 20.0, 20.0);
        temp.position.set(20.0, -300.0, -100.0);
        // temp.rotation.x = 15.0 * crate::PI / 180.0;
        // temp.rotation.y = 25.0 * crate::PI / 180.0;
        scenegraph.add_object(scene, temp);

        let mut temp = Node::new(Some(asteroid_meta));
        temp.scale.set(15.0, 15.0, 15.0);
        temp.position.set(100.0, 100.0, -140.0);
        // temp.rotation.x = 15.0 * crate::PI / 180.0;
        // temp.rotation.y = 25.0 * crate::PI / 180.0;
        scenegraph.add_object(scene, temp);


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

        let mut asteroid_ids = Arena::empty();
        for i in 0..5 {
            let mut asteroid = Node::new(Some(asteroid_meta));
            asteroid.scale.set(5.0, 5.0, 5.0);
            asteroid.position.set(-100.0, 0.0, -100.0);
            asteroid.position.x += 40.0 * i as f32;
            asteroid.rigidbody.enable(100.0, 2.0);
            asteroid.aabb.enable(12.0, 12.0);

            let mut test = Node::new(Some(testmeta));
            test.scale.set(12.0, 12.0, 12.0);
            test.position = asteroid.position;

            asteroid_ids.add(scenegraph.add_object(scene, asteroid));
            scenegraph.add_object(scene, test);
        }



        Self {
            scenegraph, scene,
            shipid, asteroid_ids,
        }
    }


    pub fn render_frame(
        game: &mut Game, t: f32, dt: f32, keys: u8,
        camera: &mut Camera, buffer: &mut MemoryBuffer
    ) {

        // let ship = &mut game.scenegraph.nodes[game.shipid];
        let mut ship = game.scenegraph.load(game.shipid);

        for asteroid_id in game.asteroid_ids.slice() {
            let asteroid = game.scenegraph.load_mut_ref(*asteroid_id);

            if asteroid.aabb.collide(&ship.aabb) {
                buffer.add_float((*asteroid_id).into(), 0.0, 1.0, 1.0);
            } else {
                buffer.add_float((*asteroid_id).into(), 0.0, 1.0, 0.0);
            }

        }

        if keys & (1 << 0) > 0 {
            ship.rigidbody.apply_force_comps(0.0, 0.0, -1.0);
        }
        if keys & (1 << 1) > 0 {
            ship.rigidbody.apply_force_comps(-1.0, 0.0, 0.0);
        }
        if keys & (1 << 2) > 0 {
            ship.rigidbody.apply_force_comps(0.0, 0.0, 1.0);
        }
        if keys & (1 << 3) > 0 {
            ship.rigidbody.apply_force_comps(1.0, 0.0, 0.0);
        }
        // if keys & (1 << 0) > 0 { ship.position.z -= 1.0; }
        // if keys & (1 << 1) > 0 { ship.position.x -= 1.0; }
        // if keys & (1 << 2) > 0 { ship.position.z += 1.0; }
        // if keys & (1 << 3) > 0 { ship.position.x += 1.0; }

        camera.position.x = ship.position.x;
        camera.position.z = ship.position.z + 0.1;
        camera.look_at(ship.position);
        // camera.look_at(game.scenegraph[1].position);




        game.scenegraph.store(game.shipid, ship);




        let t3 = game.scenegraph.load_mut_ref(3.into());
        t3.rotation.x = -t * 0.5 * crate::PI / 180.0;
        t3.rotation.y = -t * 0.5 * crate::PI / 180.0;

        let mut t4 = game.scenegraph.load_mut_ref(4.into());
        t4.rotation.x = -t * 0.5 * crate::PI / 180.0;
        t4.rotation.y = -t * 0.5 * crate::PI / 180.0;
    }

}
