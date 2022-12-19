mod error;

pub use error::*;

use husky_package_path::{CratePath, PackagePathDb};
use husky_toolchain::Toolchain;
use husky_vfs::VfsResult;
use salsa::DbWithJar;

pub trait ToolchainInferDb: DbWithJar<ToolchainInferJar> + PackagePathDb {
    fn crate_toolchain(&self, crate_path: CratePath) -> &ToolchainInferResult<Toolchain>;
}

// ad hoc
impl<T> ToolchainInferDb for T
where
    T: DbWithJar<ToolchainInferJar> + PackagePathDb,
{
    fn crate_toolchain(&self, crate_path: CratePath) -> &ToolchainInferResult<Toolchain> {
        crate_toolchain(self, crate_path)
    }
}

#[salsa::jar(db = ToolchainInferDb)]
pub struct ToolchainInferJar(crate_toolchain);

#[salsa::tracked(jar = ToolchainInferJar, return_ref)]
fn crate_toolchain(
    db: &dyn ToolchainInferDb,
    crate_path: CratePath,
) -> ToolchainInferResult<Toolchain> {
    // ad hoc
    ad_hoc_crate_toolchain(db)
}

fn ad_hoc_crate_toolchain(db: &dyn ToolchainInferDb) -> ToolchainInferResult<Toolchain> {
    todo!()
}
