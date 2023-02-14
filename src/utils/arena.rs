use core::ops::{ Index, IndexMut };


pub struct Arena<T, const N: usize> {
    current: usize,
    arena: [T; N],
}


impl<T, const N: usize> Index<usize> for Arena<T, N> {
    type Output = T;
    fn index(&self, id: usize) -> &Self::Output {
        assert!(id < self.current);
        unsafe { self.arena.get_unchecked(id) }
    }
}


impl<T, const N: usize> IndexMut<usize> for Arena<T, N> {
    fn index_mut(&mut self, id: usize) -> &mut Self::Output {
        assert!(id < self.current);
        unsafe { self.arena.get_unchecked_mut(id) }
    }

}


impl<T, const N: usize> Arena<T, N> where T: Default {

    pub fn empty() -> Self {
        Self {
            current: 0,
            arena: core::array::from_fn(|_| Default::default()),
        }
    }

    pub fn add(&mut self, item: T) -> usize {
        assert!(self.current < N - 1);
        unsafe { *self.arena.get_unchecked_mut(self.current) = item }
        self.current += 1;
        self.current - 1
    }

    pub fn len(&self) -> usize {
        self.current
    }

    pub fn slice(&self) -> &[T] {
        &self.arena[0..self.current]
    }

    pub fn slice_mut(&mut self) -> &mut [T] {
        &mut self.arena[0..self.current]
    }

//     pub fn iter(&self) -> iter::Take<slice::Iter<'_, T>> {
//         self.arena.iter().take(self.current)
//     }

//     pub fn iter_mut(&mut self) -> iter::Take<slice::IterMut<'_, T>> {
//         self.arena.iter_mut().take(self.current)
//     }

}
