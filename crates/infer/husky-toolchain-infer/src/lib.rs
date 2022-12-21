#![feature(trait_upcasting)]
#![allow(incomplete_features)]
mod error;

pub use error::*;

use husky_path_utils::derive_library_path_from_cargo_manifest_dir;
use husky_vfs::*;

use salsa::DbWithJar;

pub trait ToolchainInferDb: DbWithJar<ToolchainInferJar> + VfsDb {
    fn crate_toolchain(&self, crate_path: CratePath) -> &ToolchainInferResult<Toolchain>;
}

// ad hoc
impl<T> ToolchainInferDb for T
where
    T: DbWithJar<ToolchainInferJar> + VfsDb,
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
    _crate_path: CratePath,
) -> ToolchainInferResult<Toolchain> {
    // ad hoc
    ad_hoc_crate_toolchain(db)
}

fn ad_hoc_crate_toolchain(db: &dyn ToolchainInferDb) -> ToolchainInferResult<Toolchain> {
    Ok(Toolchain::new(
        db,
        ToolchainData::Local {
            library_path: derive_library_path_from_cargo_manifest_dir(),
        },
    ))
}
