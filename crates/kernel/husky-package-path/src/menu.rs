use husky_toolchain::ToolchainData;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagePathMenu {
    core: PackagePath,
    std: PackagePath,
    core_library: CratePath,
}

impl PackagePathMenu {
    fn new(db: &dyn VfsDb, toolchain: Toolchain) -> AbsolutePathResult<Self> {
        let ident_menu = db.ident_menu();
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
        let core = f(ident_menu.core())?;
        let std = f(ident_menu.std())?;
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

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn package_path_menu(
    db: &dyn VfsDb,
    toolchain: Toolchain,
) -> AbsolutePathResult<PackagePathMenu> {
    PackagePathMenu::new(db, toolchain)
}
