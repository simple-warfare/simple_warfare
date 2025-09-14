use bevy::prelude::*;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    /// A deserialization error
    #[error("Could not parse TOML: {0}")]
    DeToml(#[from] toml::de::Error),
    /// A deserialization error
    #[error("Could not parse TOML: {0}")]
    Json(#[from] serde_json::error::Error),
    /// A serialization error
    #[error("Could not parse TOML: {0}")]
    EnToml(#[from] toml::ser::Error),
}

#[derive(Default, Debug, Resource, Clone, Copy, PartialEq, Eq)]
pub enum MessageDecodeKind {
    #[default]
    Json,
    Toml,
}

#[derive(Default, Debug, Resource, Clone, Copy, PartialEq, Eq)]
pub enum MessageEncodeKind {
    #[default]
    Json,
    Toml,
}

pub trait MessageEncode<'de> {
    fn encode(kind: MessageEncodeKind, msg: &'de str) -> Result<Self, MessageError>
    where
        Self: Sized + Deserialize<'de>,
    {
        match kind {
            MessageEncodeKind::Toml => Ok(toml::from_str(msg)?),
            MessageEncodeKind::Json => Ok(serde_json::from_str(&msg)?),
        }
    }
}

pub trait MessageDecode {
    fn decode(&self, kind: MessageDecodeKind) -> Result<String, MessageError>
    where
        Self: Sized + Serialize,
    {
        match kind {
            MessageDecodeKind::Toml => Ok(toml::to_string(self)?),
            MessageDecodeKind::Json => Ok(serde_json::to_string(self)?),
        }
    }
    fn to_bytes(&self, kind: MessageDecodeKind) -> Result<Bytes, MessageError>
    where
        Self: Sized + Serialize,
    {
        Ok(Bytes::from(self.decode(kind)?))
    }
}
