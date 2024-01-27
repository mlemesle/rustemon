#![recursion_limit = "256"]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[cfg(all(feature = "filesystem-cache", feature = "in-memory-cache"))]
compile_error!("feature \"filesystem-cache\" and feature \"in-memory-cache\" cannot be enabled at the same time");

pub mod berries;
pub mod contests;
pub mod encounters;
pub mod evolution;
pub mod games;
pub mod items;
pub mod locations;
pub mod machines;
pub mod moves;
pub mod pokemon;
pub mod utility;

pub mod client;
pub mod error;

mod endpoint;
pub(crate) use endpoint::endpoint;

mod follow;
pub use follow::Follow;

pub mod model;
