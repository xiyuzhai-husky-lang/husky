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

    pub(crate) fn from_ast(
        db: &dyn ManifestDb,
        toolchain: Toolchain,
        ast: &ManifestDependencyAst,
        errors: &mut Vec<ManifestError>,
    ) -> Option<Self> {
        // ad hoc ;
        let data = PackagePathData::Global {
            name: ast.name(),
            version: semver::Version::new(0, 1, 0), // ad hoc
        };
        let package_path = PackagePath::new(db, toolchain, data);
        check_package_path_validity(db, package_path, errors);
        Some(Self { package_path })
    }
}
fn check_package_path_validity(
    db: &dyn ManifestDb,
    package_path: PackagePath,
    errors: &mut Vec<ManifestError>,
) {
    match package_path.dir(db) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
