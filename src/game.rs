
use crate::engine::{Camera, Arena, MemoryBuffer};
use crate::engine::scenegraph::{ Scenegraph, NodeID };
use crate::engine::entity::{ Entity, Ship, Object3d };


pub struct Game {
    pub scenegraph: Scenegraph,

    pub scene: NodeID,
    pub shipid: NodeID,
    pub asteroid_ids: Arena<NodeID, 10>,
}


impl Game {

    pub fn init_scenegraph() -> Self {
        let mut scenegraph = Scenegraph::empty();
        let scene = scenegraph.root();

        let ship = Ship::new();
        let shipid = scenegraph.add_entity(scene, Entity::from(ship));


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


        let mut temp = Object3d::new(Some(asteroid_meta));
        temp.scale.set(280.0, 280.0, 280.0);
        temp.position.set(200.0, -1000.0, 0.0);
        temp.rotation.x = 45.0 * crate::PI / 180.0;
        temp.rotation.y = 45.0 * crate::PI / 180.0;
        scenegraph.add_entity(scene, Entity::from(temp));

        let mut temp = Object3d::new(Some(asteroid_meta));
        temp.scale.set(20.0, 20.0, 20.0);
        temp.position.set(20.0, -300.0, -100.0);
        // temp.rotation.x = 15.0 * crate::PI / 180.0;
        // temp.rotation.y = 25.0 * crate::PI / 180.0;
        scenegraph.add_entity(scene, Entity::from(temp));

        let mut temp = Object3d::new(Some(asteroid_meta));
        temp.scale.set(15.0, 15.0, 15.0);
        temp.position.set(100.0, 100.0, -140.0);
        // temp.rotation.x = 15.0 * crate::PI / 180.0;
        // temp.rotation.y = 25.0 * crate::PI / 180.0;
        scenegraph.add_entity(scene, Entity::from(temp));


        // let testmeta = r#"{
        //     "shader": "test",
        //     "count": 6,
        //     "attributes": {
        //         "a_position": "quad"
        //     },
        //     "uniforms": {
        //         "objectData": ["u_matrix"]
        //     }
        // }"#;

        let mut asteroid_ids = Arena::empty();
        // for i in 0..5 {
        //     let mut asteroid = Node::new(Some(asteroid_meta));
        //     asteroid.scale.set(5.0, 5.0, 5.0);
        //     asteroid.position.set(-100.0, 0.0, -100.0);
        //     asteroid.position.x += 40.0 * i as f32;
        //     asteroid.rigidbody.enable(100.0, 2.0);
        //     asteroid.aabb.enable(12.0, 12.0);

        //     let mut test = Node::new(Some(testmeta));
        //     test.scale.set(12.0, 12.0, 12.0);
        //     test.position = asteroid.position;

        //     asteroid_ids.add(scenegraph.add_object(scene, asteroid));
        //     scenegraph.add_object(scene, test);
        // }



        Self {
            scenegraph, scene,
            shipid, asteroid_ids,
        }
    }


    pub fn render_frame(
        game: &mut Game, t: f32, dt: f32, keys: u8,
        camera: &mut Camera, buffer: &mut MemoryBuffer
    ) {

        game.scenegraph.with(game.shipid, |mut ship: Ship| {
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

            ship
        });


        game.scenegraph.with(3.into(), |mut obj: Object3d| {
            obj.rotation.x = -t * 0.5 * crate::PI / 180.0;
            obj.rotation.y = -t * 0.5 * crate::PI / 180.0;
            obj
        });

        game.scenegraph.with(4.into(), |mut obj: Object3d| {
            obj.rotation.x = -t * 0.5 * crate::PI / 180.0;
            obj.rotation.y = -t * 0.5 * crate::PI / 180.0;
            obj
        });

    }

}
