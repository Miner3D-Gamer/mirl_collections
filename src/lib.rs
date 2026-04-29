//! A collections of containers?
//!
//! Tbh, idk what "collections" means but there are usually containers ¯\_(ツ)_/¯
#![feature(const_default)]
#![feature(const_trait_impl)]

/// A map implemented using `Vec`s
pub mod vec_map;

pub use vec_map::VecMap;