mod calc;
mod config;
mod jar;
mod protocol;
#[cfg(test)]
mod tests;

pub use config::*;
pub use jar::*;
pub use protocol::*;

use calc::*;
use husky_token::TokenIdx;
use husky_vfs::path::module_path::ModulePath;
use salsa::DebugWithDb;
use serde::{Deserialize, Serialize};

#[salsa::jar]
pub struct HoverJar(HoverConfig, hover_config);

#[salsa::tracked(jar = HoverJar)]
pub(crate) fn hover_config(db: &::salsa::Db) -> HoverConfig {
    HoverConfig::new(db, HoverConfigData { debug: true })
}
