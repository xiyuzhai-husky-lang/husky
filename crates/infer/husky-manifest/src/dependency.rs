use crate::*;
use husky_manifest_ast::{HasManifestAst, ManifestDependencyAst};

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn manifest_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResult<Vec<ManifestDependency>> {
    let manifest_ast = package_path.manifest_ast(db)?;
    let Some(dependencies_section) = manifest_ast.dependencies_section(db).as_ref()? else {
        return Ok(vec![])
    };
    let dependencies = dependencies_section.dependencies();
    Ok(dependencies
        .iter()
        .map(|ast| ManifestDependency::from_ast(db, ast))
        .collect())
}

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependency {
    package_path: PackagePath,
}

impl ManifestDependency {
    pub fn package_path(&self) -> PackagePath {
        self.package_path
    }

    fn from_ast(db: &dyn ManifestDb, ast: &ManifestDependencyAst) -> Self {
        todo!()
    }
}
