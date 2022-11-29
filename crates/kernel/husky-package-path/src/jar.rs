use crate::*;

#[salsa::jar(db = PackagePathDb)]
pub struct PackagePathJar(PackagePath, PackagePathMenu, builtin_package_path);
