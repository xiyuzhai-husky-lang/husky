use crate::*;

#[salsa::tracked(jar = PackagePathJar, return_ref)]
pub(crate) fn package_name(db: &dyn PackagePathDb, package: PackagePath) -> String {
    db.ident_to_dashed(package.ident(db))
}
