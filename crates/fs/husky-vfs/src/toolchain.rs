mod date;
#[cfg(test)]
mod tests;

pub use db::*;

use super::*;
use date::*;
use husky_platform::Platform;
use std::path::PathBuf;

#[salsa::interned(jar = VfsJar, db = VfsDb)]
pub struct Toolchain {
    #[return_ref]
    pub data: ToolchainData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ToolchainData {
    Published(PublishedToolchain),
    Local { library_path: DiffPath },
}

impl salsa::DebugWithDb<dyn VfsDb + '_> for ToolchainData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Published(published_toolchain) => f
                .debug_tuple("Published")
                .field(&published_toolchain.debug_with(db, include_all_fields))
                .finish(),
            Self::Local { library_path } => f
                .debug_struct("Local")
                .field(
                    "library_path",
                    &library_path.debug_with(db, include_all_fields),
                )
                .finish(),
        }
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for ToolchainData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn VfsDb, include_all_fields)
    }
}

#[salsa::interned(jar=VfsJar)]
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

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn current_toolchain(db: &dyn VfsDb) -> VfsResult<Toolchain> {
    // ad hoc
    Ok(Toolchain::new(
        db,
        ToolchainData::Local {
            library_path: DiffPath::try_new(db, "/home/xiyuzhai/repos/husky/library")
                .expect("ad hoc; this only works on my computer"),
        },
    ))
}
