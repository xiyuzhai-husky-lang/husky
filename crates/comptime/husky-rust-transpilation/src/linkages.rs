use crate::*;

#[salsa::tracked(jar = RustTranspilationJar)]
pub(crate) fn package_linkage_transpilation(
    db: &dyn RustTranspilationDb,
    package_path: PackagePath,
) -> String {
    // ad hoc
    for _ in package_linkages(db, package_path) {
        todo!()
    }
    "// todo: linkages".to_string()
}

fn package_linkages(db: &dyn RustTranspilationDb, package_path: PackagePath) -> Vec<()> {
    vec![]
}
