
pub mod object3d;
pub use object3d::Object3d;

pub mod ship;
pub use ship::Ship;

pub mod thruster;
pub use thruster::Thruster;

pub mod gun;
pub use gun::Gun;

pub mod bullet;
pub use bullet::Bullet;


use crate::engine::{Camera, Arena, ArenaID, Frame};
use crate::engine::entity::{ EntityBehavior };
use crate::math::{ Matrix4 };



pub struct Game {
    pub ship: Ship,
    pub rot1: ArenaID,
    pub rot2: ArenaID,
    pub objects: Arena<Object3d, 10>,
}



impl Game {

    pub fn new() -> Self {

        let ship = Ship::new();

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


        let mut objects = Arena::empty();

        let mut temp = Object3d::new(asteroid_meta);
        temp.scale.set(280.0, 280.0, 280.0);
        temp.position.set(200.0, -1000.0, 0.0);
        temp.rotation.x = 45.0 * crate::PI / 180.0;
        temp.rotation.y = 45.0 * crate::PI / 180.0;
        objects.add(temp);

        let mut rot1 = Object3d::new(asteroid_meta);
        rot1.scale.set(20.0, 20.0, 20.0);
        rot1.position.set(20.0, -300.0, -100.0);
        let rot1 = objects.add(rot1);

        let mut rot2 = Object3d::new(asteroid_meta);
        rot2.scale.set(15.0, 15.0, 15.0);
        rot2.position.set(100.0, 100.0, -140.0);
        let rot2 = objects.add(rot2);


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


        // let mut asteroid_ids = Arena::empty();
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
            ship,
            rot1, rot2,
            objects,
        }
    }


    pub fn render_frame(&mut self, frame: &mut Frame, camera: &mut Camera) {

        self.ship.render_frame(frame);

        camera.position.x = self.ship.position.x;
        camera.position.z = self.ship.position.z + 0.1;
        camera.look_at(self.ship.position);

        // for object in game.objects.slice_mut() {
        //     object.render_frame(frame);
        // }


        self.objects[self.rot1].rotation.x = -frame.t * 0.5 * crate::PI / 180.0;
        self.objects[self.rot1].rotation.y = -frame.t * 0.5 * crate::PI / 180.0;
        self.objects[self.rot2].rotation.x = -frame.t * 0.5 * crate::PI / 180.0;
        self.objects[self.rot2].rotation.y = -frame.t * 0.5 * crate::PI / 180.0;


        self.ship.update_uniforms(frame, Matrix4::identity());

        for object in self.objects.slice_mut() {
            object.update_uniforms(frame, Matrix4::identity());
        }
    }

}
