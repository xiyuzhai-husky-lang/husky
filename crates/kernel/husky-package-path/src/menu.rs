use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagePathMenu {
    core: PackagePath,
    std: PackagePath,
}

impl PackagePathMenu {
    fn new(db: &dyn PackagePathDb, toolchain: Toolchain) -> Self {
        let word_menu = db.word_menu();
        let f = |ident| PackagePath::new(db, PackagePathData::Builtin { ident, toolchain });
        Self {
            core: f(word_menu.core()),
            std: f(word_menu.std()),
        }
    }
}

impl PackagePathMenu {
    pub fn core(&self) -> PackagePath {
        self.core
    }

    pub fn std(&self) -> PackagePath {
        self.std
    }
}

#[salsa::tracked(jar = PackagePathJar, return_ref)]
pub(crate) fn package_path_menu(db: &dyn PackagePathDb, toolchain: Toolchain) -> PackagePathMenu {
    PackagePathMenu::new(db, toolchain)
}
