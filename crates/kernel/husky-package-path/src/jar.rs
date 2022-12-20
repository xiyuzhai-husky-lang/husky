use crate::*;

#[salsa::jar(db = PackagePathDb)]
pub struct PackagePathJar(PackagePath, CratePath, package_path_menu);
