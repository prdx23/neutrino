use core::ops::{ Index, IndexMut };





#[derive(Default, Clone, Copy)]
pub struct ArenaID(usize);

impl From<usize> for ArenaID {
    fn from(value: usize) -> Self { Self(value) }
}

impl Into<usize> for ArenaID {
    fn into(self) -> usize { self.0 }
}





#[derive(Clone)]
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
        assert!(self.current < N - 1);
        unsafe { *self.arena.get_unchecked_mut(self.current) = item }
        self.current += 1;
        ArenaID(self.current - 1)
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
