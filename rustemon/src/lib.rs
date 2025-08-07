#![recursion_limit = "256"]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

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
#[cfg(feature = "static-resources")]
pub mod static_resources;
