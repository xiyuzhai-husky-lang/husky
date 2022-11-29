use crate::*;

#[salsa::jar(db = PackagePathDb)]
pub struct PackagePathJar(PackagePath, package_path_menu);
