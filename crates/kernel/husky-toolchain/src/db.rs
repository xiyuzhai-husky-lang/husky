use crate::*;
use husky_path_utils::{derive_library_path_from_cargo_manifest_dir, Path};
use husky_platform::Platform;
use salsa::{storage::HasJar, DbWithJar};

pub trait ToolchainDb: DbWithJar<ToolchainJar> {
    fn lang_dev_toolchain(&self) -> Toolchain;
    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path;
}

impl<T> ToolchainDb for T
where
    T: DbWithJar<ToolchainJar>,
{
    fn lang_dev_toolchain(&self) -> Toolchain {
        let library_path = derive_library_path_from_cargo_manifest_dir();
        Toolchain::new(self, ToolchainData::Local { library_path })
    }
    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path {
        toolchain_library_path(self, toolchain)
    }
}
