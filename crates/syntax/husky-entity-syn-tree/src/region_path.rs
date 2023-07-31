use husky_token::TokenSheetData;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum RegionPath {
    Snippet(ModulePath),
    Decr(DecrId),
    Decl(ItemSynNodePath),
    Defn(ItemSynNodePath),
}

impl RegionPath {
    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        match self {
            RegionPath::Snippet(module_path) => module_path,
            RegionPath::Decr(id) => id.module_path(db),
            RegionPath::Decl(path) => path.module_path(db),
            RegionPath::Defn(path) => path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntitySynTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub fn token_sheet_data<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> VfsResult<&'a TokenSheetData> {
        db.token_sheet_data(self.module_path(db))
    }
}

impl From<DecrId> for RegionPath {
    fn from(v: DecrId) -> Self {
        Self::Decr(v)
    }
}
