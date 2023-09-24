use core::ops::Index;

use cfg_if::cfg_if;

/*
use core::ops::Index;

use cfg_if::cfg_if;

// dependent implementations
*/

cfg_if!{
    if #[cfg(feature = "full")] {
        use crate::vectors::{Vector, VectorSlice, private::{Map, VectorType, Combine}};
    
        impl<'v, T> VectorSlice<'v, T> {
            pub fn len(&self) -> usize {
                <Self as VectorType<T>>::len(&self)
            }

            pub fn map<F, Output>(&'v self, f: F) -> Vector<Output>
            where
                F: Fn(&T) -> Output
            {
                <Self as Map<T>>::map(&self, f)
            }
    
            pub fn map_index<F, Output>(&'v self, f: F) -> Vector<Output>
            where
                F: Fn(usize) -> Output
            {
                <Self as Map<T>>::map_index(&self, f)
            }
    
            pub fn map_enumerate<F, Output>(&'v self, f: F) -> Vector<Output>
            where
                F: Fn(usize, &T) -> Output
            {
                <Self as Map<T>>::map_enumerate(&self, f)
            }
    
            pub fn combine<F, Rhs, Output, Iter>(&'v self, other: &'v dyn VectorType<'v, Rhs, Iter = Iter>, f: F) -> crate::vectors::Vector<Output>
            where
                F: Fn(&T, &Rhs) -> Output,
                Iter: Iterator<Item = &'v Rhs>,
                Rhs: 'v
            {
                <Self as Combine<T>>::combine(&self, other, f)
            }
    
            pub fn combine_enumerate<F, Rhs, Output, Iter>(&'v self, other: &'v dyn VectorType<'v, Rhs, Iter = Iter>, f: F) -> crate::vectors::Vector<Output>
            where
                F: Fn(usize, &T, &Rhs) -> Output,
                Iter: Iterator<Item = &'v Rhs>,
                Rhs: 'v
            {
                <Self as Combine<T>>::combine_enumerate(&self, other, f)
            }
        }

    } else if #[cfg(feature = "no_std")] {}
}

impl<'v, T> Index<usize> for VectorSlice<'v, T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}