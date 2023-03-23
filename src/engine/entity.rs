
pub mod object3d;
pub use object3d::Object3d;

pub mod ship;
pub use ship::Ship;

pub mod shipengine_module;
pub use shipengine_module::ShipEngineModule;


use crate::math::{ Matrix4 };



pub trait EntityBehavior {

    fn update_matrix(&mut self, dt: f32, matrix: Matrix4) -> Matrix4;

    fn shader_metadata(&self) -> Option<&'static str>;

}


macro_rules! entity_enum { (
        pub enum Entity {
            $($variant:ident($entity:ident)),* $(,)?
        }
    ) => {
        pub enum Entity {
            $($variant($entity),)*
        }
        $( entity_impl_from!($variant, $entity); )*
        entity_impl_behavior!($( $variant, $entity )*);
    }
}

macro_rules! entity_impl_from {
    ($variant:ident, $entity:ident) => {
        impl From<Entity> for $entity {
            fn from(value: Entity) -> Self {
                match value {
                    Entity::$variant(x) => x,
                    _ => panic!("Entity variant mismatch!"),
                }
            }
        }
        impl From<$entity> for Entity {
            fn from(value: $entity) -> Self {
                Entity::$variant(value)
            }
        }
    }
}

macro_rules! entity_impl_behavior {
    ( $($variant:ident, $entity:ident)* ) => {
        impl EntityBehavior for Entity {
            fn update_matrix(&mut self, dt: f32, matrix: Matrix4) -> Matrix4 {
                match self {$(
                    Entity::$variant(x) => x.update_matrix(dt, matrix),
                )*}
            }
            fn shader_metadata(&self) -> Option<&'static str> {
                match self {$(
                    Entity::$variant(x) => x.shader_metadata(),
                )*}
            }
        }
    }
}


entity_enum! {
    pub enum Entity {
        Object3dEntity(Object3d),
        ShipEntity(Ship),
        ShipEngineModuleEntity(ShipEngineModule),
    }
}


impl Default for Entity {
    fn default() -> Self {
        Entity::Object3dEntity(Object3d::new(None))
    }
}

