#![feature(try_trait_v2)]
mod figure;
mod gui_message;
mod init;
mod key;
mod label;
mod presentation;
mod sample;
mod server_message;
mod trace;
mod world;

pub use self::figure::*;
pub use self::gui_message::*;
pub use self::init::*;
pub use self::key::*;
pub use self::label::*;
pub use self::presentation::*;
pub use self::sample::*;
pub use self::server_message::*;
pub use self::trace::*;
pub use self::world::*;

use serde::*;
use std::collections::HashMap;
