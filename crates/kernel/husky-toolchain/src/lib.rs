mod date;
mod db;
mod tests;

pub use db::*;

use date::*;
use husky_platform::Platform;
use std::path::PathBuf;

#[salsa::jar(db = ToolchainDb)]
pub struct ToolchainJar(Toolchain, toolchain_library_path);

#[salsa::interned(jar = ToolchainJar)]
pub struct Toolchain {
    #[return_ref]
    data: ToolchainData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ToolchainData {
    Published {
        channel: ToolchainChannel,
        date: ToolchainDate,
        platform: Platform,
    },
    Local {
        library_path: PathBuf,
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

#[salsa::tracked(jar = ToolchainJar, return_ref)]
fn toolchain_library_path(db: &dyn ToolchainDb, toolchain: Toolchain) -> PathBuf {
    match toolchain.data(db) {
        ToolchainData::Published {
            channel,
            date,
            platform,
        } => todo!(),
        ToolchainData::Local { library_path } => library_path.clone(),
    }
}
