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

    pub fn relative_path(&self, db: &::salsa::Db) -> std::borrow::Cow<'static, str> {
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
    pub fn crate_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [CratePath] {
        package_crate_paths(db, self)
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
fn package_crate_paths(db: &::salsa::Db, package_path: PackagePath) -> Vec<CratePath> {
    let mut crate_paths = vec![];
    if let Some(crate_path) = package_path.lib_crate_path(db) {
        crate_paths.push(crate_path)
    }
    if let Some(crate_path) = package_path.main_crate_path(db) {
        crate_paths.push(crate_path)
    }
    // todo!()
    crate_paths
}
