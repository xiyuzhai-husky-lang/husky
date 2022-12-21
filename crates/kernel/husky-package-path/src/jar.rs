use crate::*;

#[salsa::jar(db = VfsDb)]
pub struct VfsJar(PackagePath, CratePath, package_path_menu);
