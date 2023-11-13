use husky_token::TokenSheetData;
use husky_vfs::error::VfsResult;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum RegionPath {
    Snippet(ModulePath),
    Decl(ItemSynNodePath),
    Defn(ItemSynNodePath),
}

impl RegionPath {
    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        match self {
            RegionPath::Snippet(module_path) => module_path,
            RegionPath::Decl(path) => path.module_path(db),
            RegionPath::Defn(path) => path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntitySynTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub fn token_sheet_data<'a>(self, db: &'a dyn EntitySynTreeDb) -> &'a TokenSheetData {
        db.token_sheet_data(self.module_path(db))
    }
}
