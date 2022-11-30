use crate::*;

#[salsa::tracked(jar = PackagePathJar, return_ref)]
pub(crate) fn package_name(db: &dyn PackagePathDb, package: PackagePath) -> String {
    match db.package_path_data(package) {
        PackagePathData::Builtin { ident, toolchain } => db.ident_to_dashed(*ident),
        PackagePathData::Global { version } => todo!(),
        PackagePathData::Local(_) => todo!(),
        PackagePathData::Git(_) => todo!(),
    }
}
