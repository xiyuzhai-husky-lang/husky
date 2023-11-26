use crate::*;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_transpilation(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> String {
    // ad hoc
    for _ in package_linkages(db, package_path) {
        todo!()
    }
    "// todo: linkages".to_string()
}

fn package_linkages(db: &::salsa::Db, package_path: PackagePath) -> Vec<()> {
    vec![]
}
