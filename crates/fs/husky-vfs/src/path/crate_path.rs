use super::*;

/// it's guaranteed via construction that root module path will be valid
#[salsa::interned(db = VfsDb, jar = VfsJar, constructor = new_inner)]
pub struct CratePath {
    pub package_path: PackagePath,
    pub crate_kind: CrateKind,
}

impl CratePath {
    /// it's guaranteed via construction that root module path will be valid
    pub fn new(
        package_path: PackagePath,
        crate_kind: CrateKind,
        db: &dyn VfsDb,
    ) -> VfsResult<Self> {
        let slf = Self::new_inner(db, package_path, crate_kind);
        ModulePath::new_root(db, slf)?;
        Ok(slf)
    }

    pub fn relative_path(&self, db: &dyn VfsDb) -> std::borrow::Cow<'static, str> {
        match self.crate_kind(db) {
            CrateKind::Lib => "src/lib.hsy".into(),
            CrateKind::Main => "src/main.hsy".into(),
            CrateKind::Bin(_ident) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
        }
    }

    pub fn toolchain(self, db: &dyn VfsDb) -> Toolchain {
        self.package_path(db).toolchain(db)
    }

    pub fn package_ident(self, db: &dyn VfsDb) -> Ident {
        self.package_path(db).ident(db)
    }

    pub fn root_module_path<Db: ?Sized + VfsDb>(self, db: &Db) -> ModulePath {
        let db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
        ModulePath::new_root(db, self).expect("guaranteed to be valid")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CrateKind {
    Lib,
    Main,
    Bin(Ident),
    IntegratedTest(Ident),
    Example,
}

impl PackagePath {
    pub fn crate_paths<'a>(self, db: &'a dyn VfsDb) -> &'a [CratePath] {
        package_crate_paths(db, self)
    }

    pub fn lib_crate_path(self, db: &dyn VfsDb) -> Option<CratePath> {
        CratePath::new(self, CrateKind::Lib, db).ok()
    }

    pub fn lib_root_module_path(self, db: &dyn VfsDb) -> Option<ModulePath> {
        Some(self.lib_crate_path(db)?.root_module_path(db))
    }

    pub fn main_crate_path(self, db: &dyn VfsDb) -> Option<CratePath> {
        CratePath::new(self, CrateKind::Main, db).ok()
    }

    pub fn main_root_module_path(self, db: &dyn VfsDb) -> Option<ModulePath> {
        Some(self.main_crate_path(db)?.root_module_path(db))
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
fn package_crate_paths(db: &dyn VfsDb, package_path: PackagePath) -> Vec<CratePath> {
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
