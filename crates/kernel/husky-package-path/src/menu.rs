use husky_toolchain::ToolchainData;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagePathMenu {
    core: PackagePath,
    std: PackagePath,
    core_library: CratePath,
}

impl PackagePathMenu {
    fn new(db: &dyn PackagePathDb, toolchain: Toolchain) -> AbsolutePathResult<Self> {
        let word_menu = db.word_menu();
        let f = |ident| match toolchain.data(db) {
            ToolchainData::Published(toolchain) => Ok(PackagePath::new(
                db,
                PackagePathData::PublishedToolchain {
                    ident,
                    toolchain: *toolchain,
                },
            )),
            ToolchainData::Local { library_path } => {
                PackagePath::new_local(db, &library_path.join(ident.data(db)))
            }
        };
        let core = f(word_menu.core())?;
        let std = f(word_menu.std())?;
        let core_library = CratePath::new(db, core, CrateKind::Library);
        Ok(Self {
            core,
            std,
            core_library,
        })
    }

    pub fn core(&self) -> PackagePath {
        self.core
    }

    pub fn std(&self) -> PackagePath {
        self.std
    }

    pub fn core_library(&self) -> CratePath {
        self.core_library
    }
}

#[salsa::tracked(jar = PackagePathJar, return_ref)]
pub(crate) fn package_path_menu(
    db: &dyn PackagePathDb,
    toolchain: Toolchain,
) -> AbsolutePathResult<PackagePathMenu> {
    PackagePathMenu::new(db, toolchain)
}
