mod date;
mod error;
#[cfg(test)]
mod tests;

use ::salsa::Db;
pub use db::*;
pub use error::*;
use husky_fs_specs::library_path;

use super::*;
use date::*;
use husky_platform::Platform;
use std::path::PathBuf;

#[salsa::interned(jar = VfsJar, db = VfsDb)]
pub struct Toolchain {
    #[return_ref]
    pub data: ToolchainData,
}

impl Toolchain {
    pub fn library_path(self, db: &Db) -> &Path {
        match self.data(db) {
            ToolchainData::Published(_) => todo!(),
            ToolchainData::Local { library_path } => library_path.data(db),
        }
    }
    pub fn registry_path(self, db: &Db) -> &Path {
        match self.data(db) {
            ToolchainData::Published(_) => todo!(),
            ToolchainData::Local { library_path: _ } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = VfsDb, jar = VfsJar)]
pub enum ToolchainData {
    Published(PublishedToolchain),
    Local { library_path: VirtualPath },
}

#[salsa::interned(db = VfsDb, jar = VfsJar)]
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
    _db: &Db,
    _toolchain: PublishedToolchain,
) -> PathBuf {
    todo!()
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn current_toolchain(db: &Db) -> VfsResult<Toolchain> {
    // ad hoc
    Ok(Toolchain::new(
        db,
        ToolchainData::Local {
            library_path: VirtualPath::try_new(db, library_path()?)?,
        },
    ))
}
