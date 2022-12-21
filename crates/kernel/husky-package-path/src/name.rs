use crate::*;

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn package_name(db: &dyn VfsDb, package: PackagePath) -> String {
    db.ident_to_dashed(package.ident(db))
}
