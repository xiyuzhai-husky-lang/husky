use husky_token::TokenSheetData;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum RegionPath {
    Snippet(Toolchain),
    Decr(DecrId),
    Decl(EntityNodePath),
    Defn(EntityNodePath),
}

impl RegionPath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> Option<ModulePath> {
        match self {
            RegionPath::Snippet(_) => None,
            RegionPath::Decr(id) => Some(id.module_path(db)),
            RegionPath::Decl(path) => Some(path.module_path(db)),
            RegionPath::Defn(path) => Some(path.module_path(db)),
        }
    }

    pub fn toolchain(self, db: &dyn EntityTreeDb) -> Toolchain {
        match self {
            RegionPath::Snippet(_) => todo!(),
            _ => self.module_path(db).unwrap().toolchain(db),
        }
    }

    pub fn token_sheet_data<'a>(self, db: &'a dyn EntityTreeDb) -> VfsResult<&'a TokenSheetData> {
        match self.module_path(db) {
            Some(module_path) => db.token_sheet_data(module_path),
            None => todo!(),
        }
    }
}

impl From<DecrId> for RegionPath {
    fn from(v: DecrId) -> Self {
        Self::Decr(v)
    }
}
