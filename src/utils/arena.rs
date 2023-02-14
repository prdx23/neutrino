use core::{iter, slice};


#[derive(Clone, Copy, Debug)]
pub struct Arena<T, const N: usize> {
    current: usize,
    arena: [T; N],
}


impl<T, const N: usize> Arena<T, N> where T: Copy + Default {

    pub fn empty() -> Self {
        Self {
            current: 0,
            arena: [Default::default(); N],
        }
    }

    pub fn add(&mut self, item: T) -> usize {
        assert!(self.current < N - 1);
        unsafe { *self.arena.get_unchecked_mut(self.current) = item }
        self.current += 1;
        self.current - 1
    }

    pub fn get(&self, i: usize) -> &T {
        assert!(i < self.current);
        unsafe { self.arena.get_unchecked(i) }
    }

    pub fn get_mut(&mut self, i: usize) -> &mut T {
        assert!(i < self.current);
        unsafe { self.arena.get_unchecked_mut(i) }
    }

    pub fn iter(&self) -> iter::Take<slice::Iter<'_, T>> {
        self.arena.iter().take(self.current)
    }

    pub fn iter_mut(&mut self) -> iter::Take<slice::IterMut<'_, T>> {
        self.arena.iter_mut().take(self.current)
    }

}
