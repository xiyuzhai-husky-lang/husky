use crate::*;
use husky_word::Identifier;

#[salsa::interned(jar = PackagePathJar)]
pub struct CratePath {
    pub package_path: PackagePath,
    pub crate_kind: CrateKind,
}

impl CratePath {
    pub fn path(&self, db: &dyn PackagePathDb) -> &'static str {
        match self.crate_kind(db) {
            CrateKind::Library => "src/lib.hsy",
            CrateKind::Main => "src/main.hsy",
            CrateKind::Binary(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CrateKind {
    Library,
    Main,
    Binary(Identifier),
}
