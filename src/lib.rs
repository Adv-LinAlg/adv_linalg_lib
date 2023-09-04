#![no_std]
//! # Adv_LinAlg_Lib
//! 
//! This library is a linear algebra toolbox that aims to
//! provide the "right tool for the job" for vectors and matricies.
//! 
//! Due to this library being a "growing toolbox", this crate uses a naming convention on its types to
//! organize by functionality. For basic functionality, the basic types are
//! [`Vector<T>`](crate::vectors::Vector) and `Matrix<T>`(⚠️version 0.2⚠️),
//! which can be satisfactory in most cases.
//! To learn more, read [Default Memory Behavior](#default-memory-behavior).
//! 
//! This library also provides additional enviroment configuration via feature flags.
//! To learn more, read [Features](#features)
//! 
//! ## Default Memory Behavior
//! Providing the "right tool for the job" is often about how memory is used.
//! 
//! Therefore, one pillar of this library
//! is to provide "explicit grainular mutability", which
//! allows the developer to optimize memory usage (with greater intention than the `mut` keyword) while
//! maintaining linear algebra functionality (`+`, `*`, `-`, transposition, etc.) and
//! certain functional programming methods (`map`, `combine`).
//! 
//! By default, types follow the following rules:
//! 1. **"Interior *Im*mutability":**
//!     - This means that operations, like `vector + vector`, must store their results in new memory. 
//! 2. **"Mutability Implies Dynamic Sizing"**
//!     - This means that operations can change the internal structure if mutable.
//!         - This often happens during a `matrix * vector` transformation.
//! 
//! *Note: rule #2 does not imply invalid internal state. For instance,
//! a matrix will always be "rectuagular", but its row/column count
//! are allowed to change.*
//! 
//! These rules were chosen as the default to correlate the
//! simplicity of the internal implementation itself.
//! 
//! The ***"basic"*** types are types that follow these two rules.
//! 
//! The ***"advanced"*** types are types that break at least 
//! one of these rules.
//! 
//! ### Basic Types
//! The two basic types are:
//! - [`Vector<T>`](crate::vectors::Vector) and 
//! - `Matrix<T>` ⚠️version 0.2⚠️.
//! 
//! These two rules were chosen as the default due to how
//! simple the type is; it's just a wrapper around a
//! `Vec` from the standard library.
//! 
//! For instance, here is the the structure of [`Vector<T>`](crate::vectors::Vector):
//! ```rust
//! pub struct Vector<T> {
//!     values: Vec<T>
//! }
//! ```
//! 
//! ### Advanced Types
//! By knowing the default memory behavior of this typesystem, understanding
//! when exceptions are made from the rules is explained by adding one of the
//! following to a basic type: 
//! - first rule broken: the type will have the prefix "Mut"
//! - second rule broken: the type will have a suffix, which 
//! defines how the memory is used instead.
//! 
//! 
//! So, [`MutVectorSlice<'v, T>`](crate::vectors::MutVectorSlice) implies that:
//! 1. results to reuse its memory, and
//! 2. it is a slice, so the memory is statically sized.
//! 
//! #### Type Suffixes
//! As functionality grows, the list of possible suffixes will grow as well.
//! 
//! Currently, there are four suffixes in development are:
//! 1. `Slice`: enforces static sizing
//! 2. ⚠️experimental⚠️`Rc` : enforces static sizing, but cheap to clone
//! 3. ⚠️experimental⚠️`Simd` : size limited to a `std::simd::Simd<T; N>` buffer
//! 4. ⚠️experimental⚠️`Gpu` : memory is bound to a gpu
//! 
//! ## Features
//! The library currently features three feature flags:
//! 1. `full`: All types and features are enabled.
//! 2. `no_std`: When applied in absense of `full`, this recompiles the library
//! to use only the `core` crate. 
//! 3. `cheap_casts` (off by default): Forces casts betweens types to use moves only.

use cfg_if::cfg_if;

cfg_if! (
    if #[cfg(feature = "full")] {
        extern crate alloc;
    }
);

/// module for all vector types
pub mod vectors;
/// module for all matrix types
pub mod matricies;
/// default types
pub mod prelude;