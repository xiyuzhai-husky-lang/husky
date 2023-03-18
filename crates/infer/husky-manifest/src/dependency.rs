use crate::*;
use husky_manifest_ast::{HasPackageManifestAst, ManifestDependencyAst};

#[derive(Debug, PartialEq, Eq)]
pub struct PackageDependency {
    package_path: PackagePath,
}

impl PackageDependency {
    pub fn package_path(&self) -> PackagePath {
        self.package_path
    }

    fn from_ast(
        db: &dyn ManifestDb,
        package_path: PackagePath,
        ast: &ManifestDependencyAst,
    ) -> ManifestResult<Self> {
        // let toolchain = package_path.toolchain(db);
        // let data = PackagePathData::Global {
        //     name: ast.name(),
        //     version: todo!(),
        // };
        // ad hoc
        todo!()
    }
}
