use husky_entity_path::region::RegionPath;
use husky_token::{TokenDb, TokenSheetData};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum SynNodeRegionPath {
    Decl(ItemSynNodePath),
    Defn(ItemSynNodePath),
}

impl SynNodeRegionPath {
    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            SynNodeRegionPath::Decl(path) => path.module_path(db),
            SynNodeRegionPath::Defn(path) => path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub fn token_sheet_data<'a>(self, db: &'a ::salsa::Db) -> &'a TokenSheetData {
        db.token_sheet_data(self.module_path(db))
    }

    pub fn region_path(self, db: &::salsa::Db) -> Option<RegionPath> {
        Some(match self {
            SynNodeRegionPath::Decl(path) => RegionPath::Decl(path.unambiguous_item_path(db)?),
            SynNodeRegionPath::Defn(path) => RegionPath::Defn(path.unambiguous_item_path(db)?),
        })
    }
}
