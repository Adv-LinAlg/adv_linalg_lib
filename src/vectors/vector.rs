#![cfg(feature = "full")]

use crate::vectors::{MutVector, MutVectorSlice, Vector, VectorSlice};
use core::ops::{Index, Range};
use alloc::vec::Vec;
use cfg_if::cfg_if;

impl<T> Vector<T> {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn to_slice<'v>(&'v self, range: Range<usize>) -> VectorSlice<'v, T> {
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
}

/// Map Implementations
///
/// Applies a function element-wise to every value in the
/// vector to produce a new vector.
///
/// Internally, this is simply using the `map()` method from
/// the `Iterator` trait.
///
/// ## Panic!
///
/// This function will panic if the vectors are two different sizes.
impl<I> Vector<I> {
    pub fn map<F, O>(&self, f: F) -> Vector<O>
    where
        F: Fn(&I) -> O,
    {
        Vector::from(self.values.iter().map(|value| f(value)).collect::<Vec<O>>())
    }

    pub fn map_index<F, O>(&self, f: F) -> Vector<O>
    where
        F: Fn(usize) -> O,
    {
        Vector::from(
            self.values
                .iter()
                .enumerate()
                .map(|(index, _)| f(index))
                .collect::<Vec<O>>(),
        )
    }

    pub fn map_enumerate<F, O>(&self, f: F) -> Vector<O>
    where
        F: Fn(usize, &I) -> O,
    {
        Vector::from(
            self.values
                .iter()
                .enumerate()
                .map(|(index, value)| f(index, value))
                .collect::<Vec<O>>(),
        )
    }
}

/// Combine Implementations
///
/// Applies a function pair-wise between two vectors that produces a new vector.
///
/// ## Example
/// ```rust
/// use adv_linalg_lib::vector;
/// use adv_linalg_lib::vectors::Vector;
///
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![]
/// ```
impl<L> Vector<L> {
    pub fn combine<F, R, O>(&self, other: &Vector<R>, f: F) -> Vector<O>
    where
        F: Fn(&L, &R) -> O,
    {
        if self.len() != other.len() {
            panic!("Cannot map vectors of different sizes")
        }
        let mut iter = self.values.iter().zip(other.values.iter());

        let mut params = Vec::with_capacity(self.len());
        while let Some((lhs_value, rhs_value)) = iter.next() {
            params.push(f(lhs_value, rhs_value))
        }
        Vector::from(params)
    }

    pub fn combine_enumerate<F, R, O>(&self, other: &Vector<R>, f: F) -> Vector<O>
    where
        F: Fn(&L, &R, usize) -> O,
    {
        if self.len() != other.len() {
            panic!("Cannot map vectors of different sizes")
        }

        let mut iter = self.values.iter().zip(other.values.iter()).enumerate();

        let mut params = Vec::with_capacity(self.len());
        while let Some((index, (lhs_value, rhs_value))) = iter.next() {
            params.push(f(lhs_value, rhs_value, index));
        }
        Vector { values: params }
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
impl<'v, T> Index<usize> for Vector<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}