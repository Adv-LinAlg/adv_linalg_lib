#![cfg(feature = "full")]
use crate::vectors::{
    MutVector, Vector, VectorSlice,
    private::{VectorType, Map, Combine}
};
use core::ops::{Index, Range, IndexMut};

use crate::vectors::private::{MapMut, CombineMut};

impl<T> MutVector<T> {
    pub fn len(&self) -> usize {
        <Self as VectorType<T>>::len(&self)
    }

    pub fn as_slice(&self, range: Range<usize>) -> VectorSlice<'_, T> {
        VectorSlice {
            values: self
                .values
                .as_slice()
                .split_at(range.start)
                .1
                .split_at(range.len())
                .0,
        }
    }

    pub fn map<F, Output>(&self, f: F) -> Vector<Output>
    where
        F: Fn(&T) -> Output
    {
        <Self as Map<T>>::map(&self, f)
    }

    pub fn map_index<F, Output>(&self, f: F) -> Vector<Output>
    where
        F: Fn(usize) -> Output
    {
        <Self as Map<T>>::map_index(&self, f)
    }

    pub fn map_enumerate<F, Output>(&self, f: F) -> Vector<Output>
    where
        F: Fn(usize, &T) -> Output
    {
        <Self as Map<T>>::map_enumerate(&self, f)
    }

    pub fn map_mut<'v, F>(&'v mut self, f: F) -> &'v mut Self
    where
        F: Fn(&mut T)
    {
        <Self as MapMut<T>>::map_mut(self, f)
    }

    pub fn map_index_mut<'v, F, Output>(&'v mut self, f: F) -> &'v Self
    where
        F: Fn(usize) -> T
    {
        <Self as MapMut<T>>::map_index_mut(self, f)
    }

    pub fn map_enumerate_mut<'v, F, Output>(&'v mut self, f: F) -> &'v Self
    where
        F: Fn(usize, &mut T)
    {
        <Self as MapMut<T>>::map_enumerate_mut(self, f)
    }

    pub fn combine<'v, F, Rhs, Output, Iter>(&'v self, other: &'v dyn VectorType<'v, Rhs, Iter = Iter>, f: F) -> crate::vectors::Vector<Output>
    where
        F: Fn(&T, &Rhs) -> Output,
        Iter: Iterator<Item = &'v Rhs>,
        Rhs: 'v
    {
        <Self as Combine<T>>::combine(&self, other, f)
    }

    pub fn combine_enumerate<'v, F, Rhs, Output, Iter>(&'v self, other: &'v dyn VectorType<'v, Rhs, Iter = Iter>, f: F) -> crate::vectors::Vector<Output>
    where
        F: Fn(usize, &T, &Rhs) -> Output,
        Iter: Iterator<Item = &'v Rhs>,
        Rhs: 'v
    {
        <Self as Combine<T>>::combine_enumerate(&self, other, f)
    }

    pub fn combine_mut<'v, F, Rhs, Iter>(&'v mut self, other: &'v dyn VectorType<'v, Rhs, Iter = Iter>, f: F) -> &'v mut Self
    where
        F: Fn(&mut T, &Rhs),
        Iter: Iterator<Item = &'v Rhs>,
        Rhs: 'v
    {
        <Self as CombineMut<T>>::combine_mut(self, other, f)
    }

    pub fn combine_enumerate_mut<'v, F, Rhs, Iter>(&'v mut self, other: &'v dyn VectorType<'v, Rhs, Iter = Iter>, f: F) -> &'v mut Self
    where
        F: Fn(usize, &mut T, &Rhs),
        Iter: Iterator<Item = &'v Rhs>,
        Rhs: 'v
    {
        <Self as CombineMut<T>>::combine_enumerate_mut(self, other, f)
    }
}
impl<T> Index<usize> for MutVector<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
impl<T> IndexMut<usize> for MutVector<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}