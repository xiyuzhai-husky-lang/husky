mod date;
mod db;
mod tests;

pub use db::*;

use date::*;
use husky_platform::Platform;
use std::path::PathBuf;

#[salsa::jar(db = ToolchainDb)]
pub struct ToolchainJar(
    Toolchain,
    PublishedToolchain,
    published_toolchain_library_path,
);

#[salsa::interned(jar = ToolchainJar)]
pub struct Toolchain {
    #[return_ref]
    pub data: ToolchainData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ToolchainData {
    Published(PublishedToolchain),
    Local { library_path: PathBuf },
}

#[salsa::interned(jar = ToolchainJar)]
pub struct PublishedToolchain {
    channel: ToolchainChannel,
    date: ToolchainDate,
    platform: Platform,
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

#[salsa::tracked(jar = ToolchainJar, return_ref)]
fn published_toolchain_library_path(
    db: &dyn ToolchainDb,
    toolchain: PublishedToolchain,
) -> PathBuf {
    todo!()
}
