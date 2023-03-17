use crate::*;
use husky_manifest_ast::HasManifestAst;

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn manifest_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<Vec<ManifestDependency>> {
    let manifest_ast = package_path.manifest_ast(db);
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependency {
    package_path: PackagePath,
}

impl ManifestDependency {
    pub fn package_path(&self) -> PackagePath {
        self.package_path
    }
}
