pub mod crate_decl;
pub mod item_decl;
pub mod item_defn;

pub use self::item_decl::*;
pub use self::item_defn::*;

use self::crate_decl::{CrateDeclTokraRegionDataRef, HasCrateDeclTokraRegion};
use super::*;
use husky_entity_path::region::RegionPath;
use husky_regional_token::*;
use husky_token::TokenIdx;
use node::HasSynNodePath;
use region_path::SynNodeRegionPath;

/// Tokra region is for representing tokens and asts in a relative way.
/// Previously, the source code is lexed into tokens and then divided into crude asts.
/// Then we divide the source code into minimal code analysis units, called regions.
/// Token and ast data and the relative indexes against the starting token of the region,
/// is collected as Tokra Region
///
#[derive(Debug, Clone, Copy)]
#[enum_class::from_variants]
pub enum TokraRegionDataRef<'a> {
    CrateDecl(CrateDeclTokraRegionDataRef<'a>),
    ItemDecl(ItemDeclTokraRegionDataRef<'a>),
    ItemDefn(ItemDefnTokraRegionDataRef<'a>),
}

impl<'a> std::ops::Index<RegionalTokenIdx> for TokraRegionDataRef<'a> {
    type Output = TokenData;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        match self {
            TokraRegionDataRef::CrateDecl(slf) => &slf[index],
            TokraRegionDataRef::ItemDecl(slf) => &slf[index],
            TokraRegionDataRef::ItemDefn(slf) => &slf[index],
        }
    }
}

impl SynNodeRegionPath {
    pub fn regional_token_idx_base(self, db: &::salsa::Db) -> Option<RegionalTokenIdxBase> {
        match self {
            SynNodeRegionPath::CrateDecl(_) => todo!(),
            SynNodeRegionPath::ItemDecl(slf) => Some(slf.decl_regional_token_idx_base(db)),
            SynNodeRegionPath::ItemDefn(slf) => slf.defn_regional_token_idx_base(db),
        }
    }

    pub fn tokra_region_data_ref<'a>(self, db: &'a ::salsa::Db) -> Option<TokraRegionDataRef<'a>> {
        Some(match self {
            SynNodeRegionPath::CrateDecl(slf) => slf.decl_tokra_region(db)?.data(db).into(),
            SynNodeRegionPath::ItemDecl(slf) => slf.decl_tokra_region(db).data(db).into(),
            SynNodeRegionPath::ItemDefn(slf) => slf.defn_tokra_region(db)?.data(db).into(),
        })
    }
}

pub trait HasRegionalTokenIdxBase {
    fn regional_token_idx_base(self, db: &::salsa::Db) -> Option<RegionalTokenIdxBase>;
}

impl HasRegionalTokenIdxBase for RegionPath {
    fn regional_token_idx_base(self, db: &salsa::Db) -> Option<RegionalTokenIdxBase> {
        match self {
            RegionPath::CrateDecl(slf) => slf.decl_regional_token_idx_base(db),
            RegionPath::ItemDecl(slf) => {
                Some(slf.syn_node_path(db).decl_regional_token_idx_base(db))
            }
            RegionPath::ItemDefn(slf) => slf.syn_node_path(db).defn_regional_token_idx_base(db),
            RegionPath::Script(slf) => Some(RegionalTokenIdxBase::new_chunk(slf, db)),
        }
    }
}
