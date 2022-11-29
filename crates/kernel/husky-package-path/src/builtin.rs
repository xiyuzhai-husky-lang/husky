use crate::*;

#[salsa::tracked(jar = PackagePathJar)]
pub struct PackagePathMenu {
    core: PackagePath,
    std: PackagePath,
}

#[salsa::tracked(jar = PackagePathJar)]
pub(crate) fn builtin_package_path(
    db: &dyn PackagePathDb,
    toolchain: Toolchain,
    ident: Identifier,
) -> Option<PackagePath> {
    let word_menu = db.word_menu();
    let word = ident.word();
    if word == word_menu.core() || word == word_menu.std() {
        Some(PackagePath::new(
            db,
            ident,
            PackagePathData::Builtin { toolchain },
        ))
    } else {
        None
    }
}
