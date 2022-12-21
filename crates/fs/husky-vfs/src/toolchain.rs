mod date;
#[cfg(test)]
mod tests;

pub use db::*;

use super::*;
use date::*;
use husky_platform::Platform;
use std::path::PathBuf;

#[salsa::interned(jar = VfsJar)]
pub struct Toolchain {
    #[return_ref]
    pub data: ToolchainData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ToolchainData {
    Published(PublishedToolchain),
    Local { library_path: PathBuf },
}

#[salsa::interned(jar =VfsJar)]
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

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn published_toolchain_library_path(
    _db: &dyn VfsDb,
    _toolchain: PublishedToolchain,
) -> PathBuf {
    todo!()
}
