mod consts;
mod credential;
mod error;
pub mod rest;
pub mod websocket;

pub use crate::error::BitFlyerError;

use fehler::throws;

pub const API_VERSION: &str = "1.2.0";
