#[cfg(feature = "client")]
pub mod client;
pub mod id;
#[cfg(feature = "message")]
mod message;
pub mod protocol;
#[cfg(feature = "server")]
pub mod server;
pub mod settings;
pub mod stalk;
pub mod synchrotron;
pub mod view;

use self::id::*;
use self::protocol::*;
use self::stalk::*;
use self::synchrotron::*;
use husky_visual_protocol::IsFigure;
use serde::{Deserialize, Serialize};
