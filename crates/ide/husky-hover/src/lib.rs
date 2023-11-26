#![feature(trait_upcasting)]
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
use salsa::DebugWithDb;
use serde::{Deserialize, Serialize};

#[salsa::jar(db = HoverDb)]
pub struct HoverJar(HoverConfig, hover_config);

#[salsa::tracked(jar = HoverJar)]
pub(crate) fn hover_config(db: &::salsa::Db) -> HoverConfig {
    HoverConfig::new(db, HoverConfigData { debug: true })
}
