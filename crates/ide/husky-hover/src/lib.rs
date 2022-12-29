mod calc;
mod config;
mod db;
mod protocol;
#[cfg(test)]
mod tests;

pub use config::*;
pub use db::*;
pub use protocol::*;

use calc::*;
use husky_token::TokenIdx;
use husky_vfs::*;
use serde::{Deserialize, Serialize};

#[salsa::jar(db = HoverDb)]
pub struct HoverJar(HoverConfig, hover_config);

#[salsa::tracked(jar = HoverJar)]
pub(crate) fn hover_config(db: &dyn HoverDb) -> HoverConfig {
    HoverConfig::new(db)
}
