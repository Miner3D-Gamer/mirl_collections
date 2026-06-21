//! A collections of containers?
//!
//! Tbh, idk what "collections" means but there are usually containers ¯\_(ツ)_/¯
#![feature(const_default)]
#![feature(const_trait_impl)]
//
#![feature(vec_try_remove)]
#![feature(slice_swap_unchecked)]

/// A map implemented using `Vec`s
pub mod vec_map;

pub use vec_map::VecMap;

/// A vec that doesn't shrink when objects are removed, making it O(1) for insertion, getting, and removing values.
pub mod sparse_vec;

pub use sparse_vec::SparseVec;

/// A vec with at least 1 element
pub mod non_empty_vec;
#[allow(deprecated)]
pub use non_empty_vec::NonEmptyVec;

/// A "smart" wrapper around [`Vec<u8>`] for handling files/raw data more easily
pub mod file_data;
pub use file_data::{BinaryData, GenericDataType};

/// Commonly used items from this list
pub mod prelude;
