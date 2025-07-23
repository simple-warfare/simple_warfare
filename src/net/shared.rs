use std::net::Ipv6Addr;

use serde::{Deserialize, Serialize};

pub const SERVER_HOST: Ipv6Addr = Ipv6Addr::LOCALHOST;
pub const LOCAL_BIND_IP: Ipv6Addr = Ipv6Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 6000;

// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub name: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
        }
    }
}
