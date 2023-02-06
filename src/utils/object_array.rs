use std::{iter, slice};
use crate::engine::Object3d;
use crate::math::Vec3;


const OBJ_ARRAY_SIZE: usize = 20;


pub struct ObjectArray {
    current: usize,
    objects: [Object3d; OBJ_ARRAY_SIZE],
}


impl ObjectArray {

    pub fn empty() -> Self {
        Self {
            current: 0,
            objects: [Object3d::empty(); OBJ_ARRAY_SIZE]
        }
    }

    pub fn add_object(
        &mut self, position: Vec3, scale: Vec3, rotation: Vec3, meta: &str
    ) -> usize {

        let id = self.current.clone();

        self.objects[self.current] = Object3d {
            id: id as f32, position, scale, rotation,
        };

        unsafe {
            crate::add_object(id, meta.as_ptr(), meta.len());
        }

        self.current += 1;
        id
    }

    pub fn iter(&self) -> iter::Take<slice::Iter<'_, Object3d>> {
        self.objects.iter().take(self.current)
    }

    pub fn iter_mut(&mut self) -> iter::Take<slice::IterMut<'_, Object3d>> {
        self.objects.iter_mut().take(self.current)
    }

}
