use core::{iter, slice};


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
        let id = self.current;
        self.arena[self.current] = item;
        self.current += 1;
        id
    }

    pub fn get(&self, i: usize) -> &T {
        assert!(i < self.current);
        unsafe { self.arena.get_unchecked(i) }
        // self.arena.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> &mut T {
        assert!(i < self.current);
        unsafe { self.arena.get_unchecked_mut(i) }
        // self.arena.get_mut(i)
    }

    pub fn iter(&self) -> iter::Take<slice::Iter<'_, T>> {
        self.arena.iter().take(self.current)
    }

    pub fn iter_mut(&mut self) -> iter::Take<slice::IterMut<'_, T>> {
        self.arena.iter_mut().take(self.current)
    }
}
