use std::{iter, slice};
use crate::game::Object3d;


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

    pub fn add_object(&mut self, obj: Object3d) {
        self.objects[self.current] = obj;
        self.current += 1;
    }

    pub fn iter(&self) -> iter::Take<slice::Iter<'_, Object3d>> {
        self.objects.iter().take(self.current)
    }

    pub fn iter_mut(&mut self) -> iter::Take<slice::IterMut<'_, Object3d>> {
        self.objects.iter_mut().take(self.current)
    }

}
