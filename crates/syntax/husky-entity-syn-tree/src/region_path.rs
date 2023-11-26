use husky_entity_path::region::RegionPath;
use husky_token::{TokenDb, TokenSheetData};
use husky_vfs::error::VfsResult;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum SynNodeRegionPath {
    Snippet(ModulePath),
    Decl(ItemSynNodePath),
    Defn(ItemSynNodePath),
}

impl SynNodeRegionPath {
    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        todo!()
        // match self {
        //     SynNodeRegionPath::Snippet(module_path) => module_path,
        //     SynNodeRegionPath::Decl(path) => path.module_path(db),
        //     SynNodeRegionPath::Defn(path) => path.module_path(db),
        // }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub fn token_sheet_data<'a>(self, db: &'a ::salsa::Db) -> &'a TokenSheetData {
        db.token_sheet_data(self.module_path(db))
    }

    pub fn region_path(self, db: &::salsa::Db) -> Option<RegionPath> {
        Some(match self {
            SynNodeRegionPath::Snippet(path) => RegionPath::Snippet(path),
            SynNodeRegionPath::Decl(path) => RegionPath::Decl(path.path(db)?),
            SynNodeRegionPath::Defn(path) => RegionPath::Defn(path.path(db)?),
        })
    }
}
