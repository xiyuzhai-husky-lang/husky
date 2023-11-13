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
            CrateKind::Library => "src/lib.hsy".into(),
            CrateKind::Main => "src/main.hsy".into(),
            CrateKind::Binary(_ident) => todo!(),
            CrateKind::StandaloneTest(_) => todo!(),
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
    Library,
    Main,
    Binary(Ident),
    StandaloneTest(Ident),
}
