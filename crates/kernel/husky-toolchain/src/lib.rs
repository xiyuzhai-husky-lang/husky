mod config;
mod date;
mod db;
mod jar;

pub use config::{HasVfsConfig, VfsConfig, VfsConfigImpl, VfsConfigMimic};
pub use db::*;
pub use jar::*;

use date::*;
use husky_platform::Platform;
use std::path::PathBuf;

#[salsa::input(jar = ToolchainJar)]
pub struct Toolchain {
    #[return_ref]
    data: ToolchainData,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ToolchainData {
    Published {
        channel: ToolchainChannel,
        date: ToolchainDate,
        platform: Platform,
    },
    Local {
        path: PathBuf,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ToolchainChannel {
    Nightly,
    Stable,
}

impl ToolchainChannel {
    pub fn new_ad_hoc() -> Self {
        ToolchainChannel::Nightly
    }
}
