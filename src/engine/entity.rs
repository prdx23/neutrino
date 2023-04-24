
// pub mod object3d;
// pub use object3d::Object3d;

// pub mod ship;
// pub use ship::Ship;

// pub mod thruster;
// pub use thruster::Thruster;

// pub mod gun;
// pub use gun::Gun;


use crate::math::{ Matrix4 };
use crate::engine::{ Frame };
// use crate::game::{ Game };


pub trait EntityBehavior {

    // fn render_frame(&mut self, frame: &mut Frame, game: &mut Game);
    fn render_frame(&mut self, frame: &mut Frame);

    fn update_uniforms(&mut self, frame: &mut Frame, matrix: Matrix4);

}


// macro_rules! entity_enum { (
//         pub enum Entity {
//             $($variant:ident($entity:ident)),* $(,)?
//         }
//     ) => {
//         pub enum Entity {
//             $($variant($entity),)*
//             BlankEntity,
//         }

//         impl Default for Entity {
//             fn default() -> Self { Entity::BlankEntity }
//         }

//         $( entity_impl_from!($variant, $entity); )*
//         entity_impl_behavior!($( $variant, $entity )*);
//     }
// }

// macro_rules! entity_impl_from {
//     ($variant:ident, $entity:ident) => {
//         impl From<Entity> for $entity {
//             fn from(value: Entity) -> Self {
//                 match value {
//                     Entity::$variant(x) => x,
//                     _ => panic!("Entity variant mismatch!"),
//                 }
//             }
//         }
//         impl From<$entity> for Entity {
//             fn from(value: $entity) -> Self {
//                 Entity::$variant(value)
//             }
//         }
//     }
// }

// macro_rules! entity_impl_behavior {
//     ( $($variant:ident, $entity:ident)* ) => {
//         impl EntityBehavior for Entity {
//             // fn render_frame(&mut self, frame: &mut Frame, game: &mut Game) {
//             fn render_frame(&mut self, frame: &mut Frame) {
//                 match self {
//                     // $(Entity::$variant(x) => x.render_frame(frame, game),)*
//                     $(Entity::$variant(x) => x.render_frame(frame),)*
//                     Entity::BlankEntity => {},
//                 }
//             }
//             fn update_matrix(
//                 &mut self, frame: &mut Frame, matrix: Matrix4
//             ) -> Matrix4 {
//                 match self {
//                     $(Entity::$variant(x) => x.update_matrix(frame, matrix),)*
//                     Entity::BlankEntity => Matrix4::identity(),
//                 }
//             }

//             fn get_id(&self) -> usize {
//                 match self {
//                     $(Entity::$variant(x) => x.get_id(),)*
//                     Entity::BlankEntity => 2,
//                 }
//             }
//         }
//     }
// }


// entity_enum! {
//     pub enum Entity {
//         // Object3dEntity(Object3d),
//         ShipEntity(Ship),
//         // ThrusterEntity(Thruster),
//         // GunEntity(Gun),
//     }
// }



