#![warn(missing_docs)]

//! This crate is the core of [`multichain-soroban-bridge`]. 
//! It serves as a batteries-included structure to easily implement nodes
//! from any smart contract chain to Soroban.
//! 
//! The project's modularity allows the implementors to bridge assets atomically
//! and in a completely trustless manner from any chain to Soroban by writing
//! significantly less code and only providing logic and types for the counter-chain's
//! event logging functionality, and optionally also for the counter-chain's tx execution flow. 

//! # Example
//! TODO: write example
//! 


pub mod node;
pub mod rpc;
pub mod messaging;
pub mod config;
pub mod utils;

pub use node::Node;
pub use messaging::{NodeError, TryIntoMessage, EventLogger};
pub use config::soroban_events_stream::SorobanEventsSteamConfig;
pub use config::bridge::NodeConfiguration;
