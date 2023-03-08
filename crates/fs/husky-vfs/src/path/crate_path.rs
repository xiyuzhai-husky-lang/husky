use super::*;

#[salsa::interned(db = VfsDb, jar = VfsJar)]
pub struct CratePath {
    pub package_path: PackagePath,
    pub crate_kind: CrateKind,
}

impl CratePath {
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

    pub fn package_ident(self, db: &dyn VfsDb) -> VfsResult<Ident> {
        self.package_path(db).ident(db)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CrateKind {
    Library,
    Main,
    Binary(Ident),
    StandaloneTest(Ident),
}
