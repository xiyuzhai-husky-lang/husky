use super::*;
use maybe_result::*;

/// it's guaranteed via construction that root module path will be valid
#[salsa::interned(db = VfsDb, jar = VfsJar, constructor = new_inner)]
pub struct CratePath {
    pub package_path: PackagePath,
    pub kind: CrateKind,
}

impl CratePath {
    /// it's guaranteed via construction that root module path will be valid
    pub fn new(
        package_path: PackagePath,
        kind: CrateKind,
        db: &::salsa::Db,
    ) -> VfsMaybeResult<Self> {
        let slf = Self::new_inner(db, package_path, kind);
        ModulePath::new_root(db, slf)?;
        JustOk(slf)
    }

    pub fn dir(self, db: &::salsa::Db) -> VfsResult<VirtualPath> {
        // todo: cache this?
        Ok(self
            .package_path(db)
            .dir(db)?
            .join(self.relative_dir(db).to_string(), db))
    }

    pub fn relative_dir(self, db: &::salsa::Db) -> std::borrow::Cow<'static, str> {
        match self.kind(db) {
            CrateKind::Lib => "src".into(),
            CrateKind::Main => "src".into(),
            CrateKind::Bin(_ident) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
            CrateKind::Task => "task".into(),
            CrateKind::Requirements => "requirements".into(),
        }
    }

    pub fn relative_path(self, db: &::salsa::Db) -> std::borrow::Cow<'static, str> {
        match self.kind(db) {
            CrateKind::Lib => "src/lib.hsy".into(),
            CrateKind::Main => "src/main.hsy".into(),
            CrateKind::Bin(_ident) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
            CrateKind::Task => "task.hsy".into(),
            CrateKind::Requirements => "requirements.hsy".into(),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.package_path(db).toolchain(db)
    }

    pub fn package_ident(self, db: &::salsa::Db) -> Ident {
        self.package_path(db).ident(db)
    }

    pub fn root_module_path(self, db: &::salsa::Db) -> ModulePath {
        ModulePath::new_root(db, self).expect("guaranteed to be valid")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CrateKind {
    Lib,
    Main,
    Requirements,
    Task,
    Bin(Ident),
    IntegratedTest(Ident),
    Example,
}

impl PackagePath {
    pub fn crate_paths<'a>(self, db: &'a ::salsa::Db) -> VfsResult<&'a [CratePath]> {
        match package_crate_paths(db, self) {
            Ok(crate_paths) => Ok(crate_paths),
            Err(e) => Err(e.clone()),
        }
    }

    // todo: change to MaybeResult and cached
    pub fn lib_crate_path(self, db: &::salsa::Db) -> Option<CratePath> {
        CratePath::new(self, CrateKind::Lib, db).ok()
    }

    // todo: change to MaybeResult and cached
    pub fn lib_root_module_path(self, db: &::salsa::Db) -> Option<ModulePath> {
        Some(self.lib_crate_path(db)?.root_module_path(db))
    }

    // todo: change to MaybeResult and cached
    pub fn main_crate_path(self, db: &::salsa::Db) -> Option<CratePath> {
        CratePath::new(self, CrateKind::Main, db).ok()
    }

    // todo: change to MaybeResult and cached
    pub fn main_root_module_path(self, db: &::salsa::Db) -> Option<ModulePath> {
        Some(self.main_crate_path(db)?.root_module_path(db))
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
fn package_crate_paths(db: &::salsa::Db, package_path: PackagePath) -> VfsResult<Vec<CratePath>> {
    let mut crate_paths = vec![];
    crate_paths.extend(CratePath::new(package_path, CrateKind::Lib, db).into_result_option()?);
    crate_paths.extend(CratePath::new(package_path, CrateKind::Main, db).into_result_option()?);
    crate_paths
        .extend(CratePath::new(package_path, CrateKind::Requirements, db).into_result_option()?);
    crate_paths.extend(CratePath::new(package_path, CrateKind::Task, db).into_result_option()?);
    let package_dir = package_path.dir(db).as_ref().unwrap().data(db);
    if package_dir.join("src/bin").exists() {
        todo!()
    }
    if package_dir.join("tests").exists() {
        todo!()
    }
    Ok(crate_paths)
}
