//! # The vector types
//! 
//! A module that stores all available vectors.
//! 
//! The easiest way to start using vectors is to use the [`vector![]`](crate::vector) macro.
//! 
//! To read the naming convention of types, read [the home page](crate).
//! 
//! # Functionality
//! All vector types support 
//! - Addition (`+`)
//! - Dot Product (`*`)
//! - Subtraction (`-`)
//! 
//! ## Example: Operator Overloads
//! ```
//! use adv_linalg_lib::vector;
//! 
//! let vector1 = vector![1, 2, 3];
//! let vector2 = vector![3, 2, 1];
//! 
//! let sum_vector = &vector1 + &vector2;
//! let dot_product = &vector1 * &vector2;
//! let sub_vector = &vector1 - &vector2;
//! 
//! assert_eq!(sum_vector, vector![4, 4, 4]);
//! assert_eq!(dot_product, 10);
//! assert_eq!(sub_vector, vector![-2, 0, 2]);
//! ```
//! 
//! Additionally, all vectors implement element-wise transformations:
//! - `map` methods: maps the vector to another vector as defined by a transformation function `f`
//!     - `.map(&self, f: F)`: applies `f` by element value
//!     - `.map_index(&self, f: F)`: applies `f` by internal index
//!     - `.map_enumerate(&self, f: F)` applies `f` by internal index AND by element value
//! - `combine` methods: combines two vectors pair-wise as defined by a transformation function `f`
//!     - `.combine(.., f: F)`: applies `f` by pair-wise tuple (lhs, rhs)
//!     - `.combine_enumerate(.., f: F)`: applies `f` by pair-wise tuple AND internal index
//! 
//! ## Example: Map and Combine Methods
//! ```
//! use adv_linalg_lib::vector;
//! 
//! // Map Demo
//! let vector1 = vector![1, 2, 3];
//! let squared = vector1.map(|x| x*x);
//! assert_eq!(squared, vector![1, 4, 9]);
//! 
//! // Combine Demo
//! let lhs_vector = vector![1, 2, 3];
//! let rhs_vector = vector![3, 2, 1];
//! let combined = lhs_vector.combine(&rhs_vector, |lhs, rhs| lhs * rhs);
//! assert_eq!(combined, vector![3, 4, 3]);
//! ```

use adv_linalg_proc_macro::{impl_dot_product, impl_vector_add, impl_vector_sub};
use core::ops::{Add, Mul, Sub};
use cfg_if::cfg_if;

mod vector_slice;
mod mut_vector_slice;

mod private {
    pub trait VectorType<'v, T>
    where
        Self: 'v,
        Self::Iter: Iterator<Item = &'v T>
    {
        type Iter;

        fn iter(&'v self) -> Self::Iter;

        fn len(&'v self) -> usize;
    }

    pub trait MutVectorType<'v, T>: VectorType<'v, T>
    where
        Self: 'v,
        Self::IterMut: Iterator<Item = &'v mut T>
    {
        type IterMut;

        fn iter_mut(&'v mut self) -> Self::IterMut;
    }

    pub trait Map<'v, I>: VectorType<'v, I>
    {
        fn map<F, O>(&'v self, f: F) -> crate::vectors::Vector<O>
        where
            F: Fn(&'v I) -> O
        {
            use alloc::vec::Vec;

            crate::vectors::Vector::from(
                self.iter()
                    .map(|value| f(value))
                    .collect::<Vec<O>>()
            )
        }

        fn map_index<F, O>(&'v self, f: F) -> crate::vectors::Vector<O>
        where
            F: Fn(usize) -> O
        {
            use alloc::vec::Vec;

            crate::vectors::Vector::from(
                self.iter()
                    .enumerate()
                    .map(|(index, _)| f(index))
                    .collect::<Vec<O>>()
            )
        }

        fn map_enumerate<F, O>(&'v self, f: F) -> crate::vectors::Vector<O>
        where
            F: Fn(usize, &'v I) -> O
        {
            use alloc::vec::Vec;

            crate::vectors::Vector::from(
                self.iter()
                    .enumerate()
                    .map(|(index, value)| f(index, value))
                    .collect::<Vec<O>>()
            )
        }
    }

    pub trait MapMut<'v, T>: MutVectorType<'v, T>
    {
        fn map_mut<F>(&'v mut self, f: F) -> &'v mut Self
        where
            F: FnMut(&'v mut T)
        {
            for item in self.iter_mut() {

            }
            self
        }

        fn map_index_mut<F>(&'v mut self, f: F) -> &'v mut Self
        where
            F: FnMut(usize)
        {
            self.iter_mut()
                .enumerate()
                .map(|(index, ..)| index)
                .for_each(f);
            self
        }

        fn map_enumerate_mut<F>(&'v mut self, f: F) -> &'v mut Self
        where
            F: FnMut(usize, &'v mut T)
        {
            self.iter_mut()
                .enumerate()
                .for_each(|(index, value)| f(index, value));
            self
        }
    }
}

cfg_if! {
    if #[cfg(feature = "full")] {
        use alloc::vec::Vec;

        /// All `impl`s for `Vector<T>`
        mod vector;
        /// All `impl`s for `MutVector<T>`
        mod mut_vector;

        /// The basic vector type.
        /// 
        /// This type is also the default output from the
        /// [`vector![]`](crate::vector) macro.
        /// 
        /// ## Initialization Example
        /// ```
        /// use adv_linalg_lib::vector;
        /// use adv_linalg_lib::vectors::Vector;
        /// 
        /// let vector1 = Vector::from(vec![1, 2, 3]);
        /// let vector2 = vector![1, 2, 3];
        /// 
        /// assert_eq!(vector1, vector2);
        /// ```
        #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Vector<T> {
            values: Vec<T>,
        }
        impl<'v, T: 'v> private::VectorType<'v> for Vector<T> {
            type Iter = core::slice::Iter<'v, T>;

            fn iter(&'v self) -> Self::Iter {
                self.values.iter()
            }
    
            fn len(&'v self) -> usize {
                self.values.len()
            }
        }

        /// This vector type allows interior mutability.
        /// 
        /// ## Initialization Example
        /// ```
        /// use adv_linalg_lib::vector;
        /// use adv_linalg_lib::vectors::MutVector;
        /// 
        /// // Initializing by `vec![]`
        /// let mut_vector1 = MutVector::from(vec![1, 2, 3]);
        /// 
        /// // Initializing by `vector![]`
        /// let mut_vector2 = MutVector::from(vector![1, 2, 3]);
        /// 
        /// assert_eq!(mut_vector1, mut_vector2)
        /// ```
        /// 
        /// ## Run-time Optimization Example
        /// This can be useful with pre-allocation optimizations.
        /// 
        /// For example, consider the following code:
        /// ```
        /// use adv_linalg_lib::vector;
        /// 
        /// // example vector values
        /// let vector1 = vector![0, 0, 0, 1];
        /// let vector2 = vector![0, 0, 1, 0];
        /// let vector3 = vector![0, 1, 0, 0];
        /// let vector4 = vector![1, 0, 0, 0];
        /// 
        /// // We add all the vectors together
        /// let sum_vector = vector1 + vector2 + vector3 + vector4;
        /// 
        /// // expected result
        /// assert_eq!(sum_vector, vector![1, 1, 1, 1])
        /// ```
        /// 
        /// This works, but this uses 3 wasteful reallocations for each addition operation. This is
        /// because each result of a [`Vector<T>`](crate::vectors::Vector) operation creates a new allocation
        /// of [`Vector<T>`](crate::vectors::Vector).
        /// 
        /// By leveraging a [`MutVector<T>`](crate::vectors::MutVector), this above code can reuse memory during these operations:
        /// ```
        /// use adv_linalg_lib::vector;
        /// use adv_linalg_lib::vectors::{Vector, MutVector};
        /// 
        /// // example vector values
        /// let vector1 = vector![0, 0, 0, 1];
        /// let vector2 = vector![0, 0, 1, 0];
        /// let vector3 = vector![0, 1, 0, 0];
        /// let vector4 = vector![1, 0, 0, 0];
        /// 
        /// // We crate a buffer and store the results in the buffer
        /// let mut buffer = MutVector::from(vector1 + vector2);
        /// &mut buffer + vector3 + vector4;
        /// 
        /// // Optional: Convert back to `Vector<T>`
        /// let sum_vector = Vector::from(buffer);
        /// 
        /// // expected result
        /// assert_eq!(sum_vector, vector![1, 1, 1, 1])
        /// ```
        /// 
        /// This optimization is expected to reduce run-time
        /// for multiple repeated operations. It is still 
        /// recommended to verify run-time improvements by testing.
        #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct MutVector<T> {
            values: Vec<T>,
        }
        impl<'v, T: 'v> private::VectorType<'v> for MutVector<T> {
            type Iter = core::slice::Iter<'v, T>;

            fn iter(&'v self) -> Self::Iter {
                self.values.iter()
            }
    
            fn len(&'v self) -> usize {
                self.values.len()
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "full")]  {
        /// A slice-range of a [`Vector<T>`](crate::vectors::Vector).
        /// 
        /// ## Initialization Example
        /// ```
        /// use adv_linalg_lib::vector;
        /// use adv_linalg_lib::vectors::VectorSlice;
        /// 
        /// // initialization via vector
        /// let vector1 = vector![1, 2, 3];
        /// let vector_slice1: VectorSlice<i32> = vector1.as_slice(0..vector1.len());
        /// 
        /// // initialization via raw slice
        /// let vector_slice2 = VectorSlice::from([1, 2, 3].as_slice());
        /// 
        /// assert_eq!(vector_slice1, vector_slice2);
        /// ```
        #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct VectorSlice<'v, T> {
            values: &'v [T]
        }
        impl<'v, T> private::VectorType<'v> for VectorSlice<'v, T> {
            type Iter = core::slice::Iter<'v, T>;

            fn iter(&'v self) -> Self::Iter {
                self.values.iter()
            }
    
            fn len(&'v self) -> usize {
                self.values.len()
            }
        }

        /// A mutable slice-range of a [`MutVector<T>`](crate::vectors::Vector).
        /// 
        /// ## Initialization Example
        /// ```
        /// use adv_linalg_lib::vectors::{MutVector, MutVectorSlice};
        /// 
        /// // initialization via [`MutVector<T>`](crate::vectors::MutVector)
        /// let mut mut_vector1 = MutVector::from(vec![1, 2, 3]);
        /// let mut mut_vector_slice1 = mut_vector1.as_slice_mut(0..mut_vector1.len());
        /// 
        /// // initialization via `&mut [T]`
        /// let mut binding = [1, 2, 3];
        /// let mut_vector_slice2 = MutVectorSlice::from(binding.as_mut_slice());
        /// 
        /// assert_eq!(mut_vector_slice1, mut_vector_slice2);
        /// ```
        #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct MutVectorSlice<'v, T> {
            values: &'v mut [T]
        }
        impl<'v, T> private::VectorType<'v> for MutVectorSlice<'v, T> {
            type Iter = core::slice::Iter<'v, T>;

            fn iter(&'v self) -> Self::Iter {
                self.values.iter()
            }
    
            fn len(&'v self) -> usize {
                self.values.len()
            }
        }
    } else if #[cfg(feature = "no_std")] {
        /// The basic vector type in `#![no_std]` mode.
        /// 
        /// ## Initialization Example
        /// ```
        /// use adv_linalg_lib::vector;
        /// use adv_linalg_lib::vectors::VectorSlice;
        /// 
        /// // initialization via raw slice
        /// let vector_slice = vector![1, 2, 3];
        /// 
        /// assert_eq!(vector_slice, VectorSlice::from([1, 2, 3].as_slice()));
        /// ```
        #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct VectorSlice<'v, T> {
            values: &'v [T]
        }
        impl<'v, T> private::VectorType<'v> for VectorSlice<'v, T> {
            type Iter = core::slice::Iter<'v, T>;

            fn iter(&'v self) -> Self::Iter {
                self.values.iter()
            }
    
            fn len(&'v self) -> usize {
                self.values.len()
            }
        }

        /// Implements interior mutability, unlike
        /// [`VectorSlice<'v, T>`](crate::vectors::VectorSlice).
        /// 
        /// ## Initialization Example
        /// ```
        /// use adv_linalg_lib::vector;
        /// use adv_linalg_lib::vectors::{VectorSlice, MutVectorSlice};
        /// 
        /// // initialization via `&mut [T]`
        /// let mut binding = [1, 2, 3];
        /// let mut_vector_slice1 = MutVectorSlice::from(binding.as_mut_slice());
        /// ```
        #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct MutVectorSlice<'v, T> {
            values: &'v mut [T]
        }
        impl<'v, T> private::VectorType<'v> for MutVectorSlice<'v, T> {
            type Iter = core::slice::Iter<'v, T>;

            fn iter(&'v self) -> Self::Iter {
                self.values.iter()
            }
    
            fn len(&'v self) -> usize {
                self.values.len()
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "full")] {
        impl_vector_add!(
            impl<T: Clone + Add<Output = T>> Add for [Vector<T>] + [Vector<T>];
            impl<'lhs, T: Clone + Add<Output = T>> Add for [VectorSlice<'lhs, T>] + [Vector<T>];
            impl<'rhs, T: Clone + Add<Output = T>> Add for [Vector<T>] + [VectorSlice<'rhs, T>];
            impl<'lhs, 'rhs, T: Clone + Add<Output = T>> Add for [VectorSlice<'lhs, T>] + [VectorSlice<'rhs, T>];

            #[mut_left] impl<T: Clone + Add<Output = T>> Add for [MutVector<T>] + [Vector<T>];
            #[mut_left] impl<'lhs, T: Clone + Add<Output = T>> Add for [MutVectorSlice<'lhs, T>] + [Vector<T>];
            #[mut_left] impl<'rhs, T: Clone + Add<Output = T>> Add for [MutVector<T>] + [VectorSlice<'rhs, T>];
            #[mut_left] impl<'lhs, 'rhs, T: Clone + Add<Output = T>> Add for [MutVectorSlice<'lhs, T>] + [VectorSlice<'rhs, T>];

            #[mut_right] impl<T: Clone + Add<Output = T>> Add for [Vector<T>] + [MutVector<T>];
            #[mut_right] impl<'lhs, T: Clone + Add<Output = T>> Add for [VectorSlice<'lhs, T>] + [MutVector<T>];
            #[mut_right] impl<'rhs, T: Clone + Add<Output = T>> Add for [Vector<T>] + [MutVectorSlice<'rhs, T>];
            #[mut_right] impl<'lhs, 'rhs, T: Clone + Add<Output = T>> Add for [VectorSlice<'lhs, T>] + [MutVectorSlice<'rhs, T>];

            #[mut_both] impl<T: Clone + Add<Output = T>> Add for [MutVector<T>] + [MutVector<T>];
            #[mut_both] impl<'lhs, T: Clone + Add<Output = T>> Add for [MutVectorSlice<'lhs, T>] + [MutVector<T>];
            #[mut_both] impl<'rhs, T: Clone + Add<Output = T>> Add for [MutVector<T>] + [MutVectorSlice<'rhs, T>];
            #[mut_both] impl<'lhs, 'rhs, T: Clone + Add<Output = T>> Add for [MutVectorSlice<'lhs, T>] + [MutVectorSlice<'rhs, T>]
        );

        impl_dot_product!(
            impl<T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [Vector<T>] * [Vector<T>];
            impl<'lhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [VectorSlice<'lhs, T>] * [Vector<T>];
            impl<'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [Vector<T>] * [VectorSlice<'rhs, T>];
            impl<'lhs, 'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [VectorSlice<'lhs, T>] * [VectorSlice<'rhs, T>];

            #[mut_left] impl<T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVector<T>] * [Vector<T>];
            #[mut_left] impl<'lhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVectorSlice<'lhs, T>] * [Vector<T>];
            #[mut_left] impl<'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVector<T>] * [VectorSlice<'rhs, T>];
            #[mut_left] impl<'lhs, 'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVectorSlice<'lhs, T>] * [VectorSlice<'rhs, T>];

            #[mut_right] impl<T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [Vector<T>] * [MutVector<T>];
            #[mut_right] impl<'lhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [VectorSlice<'lhs, T>] * [MutVector<T>];
            #[mut_right] impl<'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [Vector<T>] * [MutVectorSlice<'rhs, T>];
            #[mut_right] impl<'lhs, 'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [VectorSlice<'lhs, T>] * [MutVectorSlice<'rhs, T>];

            #[mut_both] impl<T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVector<T>] * [MutVector<T>];
            #[mut_both] impl<'lhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVectorSlice<'lhs, T>] * [MutVector<T>];
            #[mut_both] impl<'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVector<T>] * [MutVectorSlice<'rhs, T>];
            #[mut_both] impl<'lhs, 'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>> Mul for [MutVectorSlice<'lhs, T>] * [MutVectorSlice<'rhs, T>]
        );

        impl_vector_sub!(
            impl<T: Clone + Sub<Output = T>> Sub for [Vector<T>] - [Vector<T>];
            impl<'lhs, T: Clone + Sub<Output = T>> Sub for [VectorSlice<'lhs, T>] - [Vector<T>];
            impl<'rhs, T: Clone + Sub<Output = T>> Sub for [Vector<T>] - [VectorSlice<'rhs, T>];
            impl<'lhs, 'rhs, T: Clone + Sub<Output = T>> Sub for [VectorSlice<'lhs, T>] - [VectorSlice<'rhs, T>];

            #[mut_left] impl<T: Clone + Sub<Output = T>> Sub for [MutVector<T>] - [Vector<T>];
            #[mut_left] impl<'lhs, T: Clone + Sub<Output = T>> Sub for [MutVectorSlice<'lhs, T>] - [Vector<T>];
            #[mut_left] impl<'rhs, T: Clone + Sub<Output = T>> Sub for [MutVector<T>] - [VectorSlice<'rhs, T>];
            #[mut_left] impl<'lhs, 'rhs, T: Clone + Sub<Output = T>> Sub for [MutVectorSlice<'lhs, T>] - [VectorSlice<'rhs, T>];

            #[mut_right] impl<T: Clone + Sub<Output = T>> Sub for [Vector<T>] - [MutVector<T>];
            #[mut_right] impl<'lhs, T: Clone + Sub<Output = T>> Sub for [VectorSlice<'lhs, T>] - [MutVector<T>];
            #[mut_right] impl<'rhs, T: Clone + Sub<Output = T>> Sub for [Vector<T>] - [MutVectorSlice<'rhs, T>];
            #[mut_right] impl<'lhs, 'rhs, T: Clone + Sub<Output = T>> Sub for [VectorSlice<'lhs, T>] - [MutVectorSlice<'rhs, T>];

            #[mut_both] impl<T: Clone + Sub<Output = T>> Sub for [MutVector<T>] - [MutVector<T>];
            #[mut_both] impl<'lhs, T: Clone + Sub<Output = T>> Sub for [MutVectorSlice<'lhs, T>] - [MutVector<T>];
            #[mut_both] impl<'rhs, T: Clone + Sub<Output = T>> Sub for [MutVector<T>] - [MutVectorSlice<'rhs, T>];
            #[mut_both] impl<'lhs, 'rhs, T: Clone + Sub<Output = T>> Sub for [MutVectorSlice<'lhs, T>] - [MutVectorSlice<'rhs, T>]
        );
    } else if #[cfg(feature = "no_std")] {
        impl_vector_add!(
            #[no_std] #[mut_left]
            impl<'lhs, 'rhs, T: Clone + Add<Output = T>>
                Add for
                    [MutVectorSlice<'lhs, T>] + [VectorSlice<'rhs, T>];
    
            #[no_std] #[mut_both]
            impl<'lhs, 'rhs, T: Clone + Add<Output = T>>
                Add for
                    [MutVectorSlice<'lhs, T>] + [MutVectorSlice<'rhs, T>]
        );
    
        impl_dot_product!(
            #[mut_left]
            impl<'lhs, 'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>>
                Mul for
                    [MutVectorSlice<'lhs, T>] * [VectorSlice<'rhs, T>];
    
            #[mut_both]
            impl<'lhs, 'rhs, T: Clone + Default + Add<Output = T> + Mul<Output = T>>
                Mul for
                    [MutVectorSlice<'lhs, T>] * [MutVectorSlice<'rhs, T>]
        );
    
        impl_vector_sub!(
            #[no_std]
            #[mut_left]
            impl<'lhs, 'rhs, T: Clone + Sub<Output = T>> Sub for [MutVectorSlice<'lhs, T>] - [VectorSlice<'rhs, T>];
    
            #[no_std]
            #[mut_both] 
            impl<'lhs, 'rhs, T: Clone + Sub<Output = T>> Sub for [MutVectorSlice<'lhs, T>] - [MutVectorSlice<'rhs, T>]
        );
    }
}
