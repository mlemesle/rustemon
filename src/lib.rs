#![recursion_limit = "256"]
#![doc = include_str!("../README.md")]

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

pub mod model;
