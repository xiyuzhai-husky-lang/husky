use crate::*;
use husky_entity_path::region::RegionPath;
use husky_token::{TokenDb, TokenSheetData};
use husky_vfs::toolchain::Toolchain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum SynNodeRegionPath {
    CrateDecl(CratePath),
    ItemDecl(ItemSynNodePath),
    ItemDefn(ItemSynNodePath),
}

impl SynNodeRegionPath {
    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            SynNodeRegionPath::CrateDecl(crate_path) => crate_path.root_module_path(db),
            SynNodeRegionPath::ItemDecl(item_path) => item_path.module_path(db),
            SynNodeRegionPath::ItemDefn(item_path) => item_path.module_path(db),
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
            SynNodeRegionPath::CrateDecl(crate_path) => RegionPath::CrateDecl(crate_path),
            SynNodeRegionPath::ItemDecl(item_path) => {
                RegionPath::ItemDecl(item_path.unambiguous_item_path(db)?)
            }
            SynNodeRegionPath::ItemDefn(item_path) => {
                RegionPath::ItemDefn(item_path.unambiguous_item_path(db)?)
            }
        })
    }
}
