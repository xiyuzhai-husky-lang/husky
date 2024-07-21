mod calc;
mod config;
mod jar;
mod protocol;
#[cfg(test)]
mod tests;

pub use config::*;
pub use jar::*;
pub use protocol::*;

use self::HoverJar as Jar;
use calc::*;
use husky_token::TokenIdx;
use husky_vfs::path::module_path::ModulePath;
use salsa::DebugWithDb;
use serde::{Deserialize, Serialize};

#[salsa::jar]
pub struct HoverJar(crate::config::hover_config);
