use crate::*;
use husky_path_utils::{derive_library_path_from_cargo_manifest_dir, Path};
use husky_platform::Platform;
use salsa::{storage::HasJar, DbWithJar};

pub trait ToolchainDb: DbWithJar<ToolchainJar> {
    fn lang_dev_toolchain(&self) -> Toolchain;
    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path;
    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path;
}

impl<Db> ToolchainDb for Db
where
    Db: DbWithJar<ToolchainJar>,
{
    fn lang_dev_toolchain(&self) -> Toolchain {
        let library_path = derive_library_path_from_cargo_manifest_dir();
        Toolchain::new(self, ToolchainData::Local { library_path })
    }
    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path {
        match toolchain.data(self) {
            ToolchainData::Published(_) => todo!(),
            ToolchainData::Local { library_path } => &library_path,
        }
    }

    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path {
        published_toolchain_library_path(self, toolchain)
    }
}
