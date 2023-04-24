use core::ops::{ Index, IndexMut };



#[derive(Default, Clone, Copy)]
pub struct ArenaID(usize);

impl From<usize> for ArenaID {
    fn from(value: usize) -> Self { Self(value) }
}

impl From<ArenaID> for usize {
    fn from(value: ArenaID) -> usize { value.0 }
}

impl From<ArenaID> for f32 {
    fn from(value: ArenaID) -> f32 { value.0 as f32 }
}



pub struct Arena<T, const N: usize> {
    current: usize,
    arena: [T; N],
}


impl<T, const N: usize> Index<ArenaID> for Arena<T, N> {
    type Output = T;
    fn index(&self, id: ArenaID) -> &Self::Output {
        assert!(id.0 < self.current);
        unsafe { self.arena.get_unchecked(id.0) }
    }
}


impl<T, const N: usize> IndexMut<ArenaID> for Arena<T, N> {
    fn index_mut(&mut self, id: ArenaID) -> &mut Self::Output {
        assert!(id.0 < self.current);
        unsafe { self.arena.get_unchecked_mut(id.0) }
    }

}


impl<T, const N: usize> Arena<T, N> where T: Default {

    pub fn empty() -> Self {
        Self {
            current: 0,
            arena: core::array::from_fn(|_| Default::default()),
        }
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    pub fn add(&mut self, item: T) -> ArenaID {
        assert!(self.current < N);
        unsafe { *self.arena.get_unchecked_mut(self.current) = item }
        self.current += 1;
        ArenaID::from(self.current - 1)
    }

    pub fn len(&self) -> usize {
        self.current
    }

    pub fn as_ptr(&mut self) -> *const T {
        self.arena.as_ptr()
    }

    pub fn slice(&self) -> &[T] {
        &self.arena[0..self.current]
    }

    pub fn slice_mut(&mut self) -> &mut [T] {
        &mut self.arena[0..self.current]
    }

    // pub fn iter(&self) -> iter::Take<slice::Iter<'_, T>> {
    //     self.arena.iter().take(self.current)
    // }

    // pub fn iter_mut(&mut self) -> iter::Take<slice::IterMut<'_, T>> {
    //     self.arena.iter_mut().take(self.current)
    // }

}


impl<T, const N: usize> From<[T; N]> for Arena<T, N> {
    fn from(arena: [T; N]) -> Self {
        Self { arena, current: N }
    }
}
