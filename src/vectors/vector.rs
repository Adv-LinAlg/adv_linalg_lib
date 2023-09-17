#![cfg(feature = "full")]

use crate::vectors::{
    MutVector, MutVectorSlice, Vector, VectorSlice,
    private::{VectorType, Map, Combine}
};
use core::ops::{Index, Range};
use alloc::vec::Vec;
use cfg_if::cfg_if;

impl<T> Vector<T> {
    /// Returns the length of the `Vector<T>`.
    pub fn len(&self) -> usize {
        <Self as VectorType<T>>::len(&self)
    }

    /// Cheaply creates a sliced view of a `Vector<T>` instance.
    /// 
    /// ## Example
    /// ```
    /// use adv_linalg_lib::vector;
    /// use adv_linalg_lib::vectors::Vector;
    /// 
    /// // Initialization of an existing vector
    /// let vector = vector![1, 2, 3, 4];
    /// let len = vector.len();
    /// 
    /// // create specific views of the vector
    /// let full_view = vector.as_slice(0..len);
    /// let lhs_view = vector.as_slice(0..len/2);
    /// let rhs_view = vector.as_slice(len/2..len);
    /// 
    /// assert_eq!(vector![1, 2, 3, 4], full_view.into());
    /// assert_eq!(vector![1, 2], lhs_view.into());
    /// assert_eq!(vector![3, 4], rhs_view.into())
    /// ```
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
}

// cheap converts
impl<T> From<Vec<T>> for Vector<T> {
    fn from(values: Vec<T>) -> Self {
        Vector { values }
    }
}
impl<T> From<MutVector<T>> for Vector<T> {
    fn from(values: MutVector<T>) -> Self {
        Vector {
            values: values.values,
        }
    }
}

cfg_if! {
    if #[cfg(not(feature = "cheap_casts"))] {
        // costly converts: clone is needed
        // note: this could still be "cheap" if every element is an Rc or Arc pointer
        //       which could work since a Vector had interior immutability.
        //       Also possibly an adv_linalg compile-time optimization???
        impl<T: Clone> From<&[T]> for Vector<T> {
            fn from(values: &[T]) -> Self {
                Vector::from(values.to_vec())
            }
        }

        impl<T: Clone> From<&mut [T]> for Vector<T> {
            fn from(values: &mut [T]) -> Self {
                Vector::from(values.to_vec())
            }
        }

        impl<T: Clone> From<&Vector<T>> for Vector<T> {
            fn from(vector: &Vector<T>) -> Self {
                Vector::from(vector.values.clone())
            }
        }

        impl<T: Clone> From<&MutVector<T>> for Vector<T> {
            fn from(vector: &MutVector<T>) -> Self {
                Vector::from(vector.values.clone())
            }
        }

        impl<T: Clone> From<&mut MutVector<T>> for Vector<T> {
            fn from(vector: &mut MutVector<T>) -> Self {
                Vector::from(vector.values.clone())
            }
        }

        impl<'v, T: Clone> From<VectorSlice<'v, T>> for Vector<T> {
            fn from(vector: VectorSlice<'v, T>) -> Self {
                Vector::from(vector.values.clone())
            }
        }

        impl<'v, T: Clone> From<&VectorSlice<'v, T>> for Vector<T> {
            fn from(vector: &VectorSlice<'v, T>) -> Self {
                Vector::from(vector.values.clone())
            }
        }

        impl<'v, T: Clone> From<MutVectorSlice<'v, T>> for Vector<T> {
            fn from(vector: MutVectorSlice<'v, T>) -> Self {
                Vector::from(vector.values)
            }
        }

        impl<'v, T: Clone> From<&MutVectorSlice<'v, T>> for Vector<T> {
            fn from(vector: &MutVectorSlice<'v, T>) -> Self {
                Vector::from(vector.values.as_ref().clone())
            }
        }

        impl<'v, T: Clone> From<&mut MutVectorSlice<'v, T>> for Vector<T> {
            fn from(vector: &mut MutVectorSlice<'v, T>) -> Self {
                Vector::from(vector.values.as_ref().clone())
            }
        }
    }
}

// dependent implementations
impl<T> Index<usize> for Vector<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}