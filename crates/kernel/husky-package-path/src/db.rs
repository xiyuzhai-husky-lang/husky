use crate::*;
use husky_toolchain::ToolchainDb;
use husky_word::WordDb;
use salsa::DbWithJar;

pub trait PackagePathDb: DbWithJar<PackagePathJar> + ToolchainDb + WordDb {
    fn builtin_package_path(&self, ident: Identifier) -> Option<PackagePath>;
}

impl<T> PackagePathDb for T
where
    T: DbWithJar<PackagePathJar> + ToolchainDb + WordDb,
{
    fn builtin_package_path(&self, ident: Identifier) -> Option<PackagePath> {
        let toolchain = self.toolchain();
        builtin_package_path(self, toolchain, ident)
    }
}
