mod client;
mod models;

pub use client::{BitFlyerRest, BitFlyerRestBuilder};
pub use models::*;

use crate::error::BitFlyerError;
use serde::{Deserialize, Serialize};
use std::convert::From;

// The error response from bitmex;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct BitFlyerErrorResponse {
    pub(crate) error: BitFlyerErrorMessage,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct BitFlyerErrorMessage {
    pub(crate) message: String,
    pub(crate) name: String,
}

impl From<BitFlyerErrorMessage> for BitFlyerError {
    fn from(msg: BitFlyerErrorMessage) -> BitFlyerError {
        BitFlyerError::RemoteError {
            message: msg.message,
            name: msg.name,
        }
    }
}
