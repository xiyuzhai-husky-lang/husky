pub mod decl;
pub mod defn;

pub use self::decl::*;
pub use self::defn::*;

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
    Decl(DeclTokraRegionDataRef<'a>),
    Defn(DefnTokraRegionDataRef<'a>),
}

impl<'a> std::ops::Index<RegionalTokenIdx> for TokraRegionDataRef<'a> {
    type Output = TokenData;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        match self {
            TokraRegionDataRef::Decl(token_region) => &token_region[index],
            TokraRegionDataRef::Defn(token_region) => &token_region[index],
        }
    }
}

impl SynNodeRegionPath {
    pub fn regional_token_idx_base(self, db: &::salsa::Db) -> Option<RegionalTokenIdxBase> {
        match self {
            SynNodeRegionPath::Decl(slf) => Some(slf.decl_regional_token_idx_base(db)),
            SynNodeRegionPath::Defn(slf) => slf.defn_regional_token_idx_base(db),
        }
    }

    pub fn tokra_region_data_ref<'a>(self, db: &'a ::salsa::Db) -> Option<TokraRegionDataRef<'a>> {
        Some(match self {
            SynNodeRegionPath::Decl(slf) => slf.decl_tokra_region(db).data(db).into(),
            SynNodeRegionPath::Defn(slf) => slf.defn_tokra_region(db)?.data(db).into(),
        })
    }
}

pub trait HasRegionalTokenIdxBase {
    fn regional_token_idx_base(self, db: &::salsa::Db) -> Option<RegionalTokenIdxBase>;
}

impl HasRegionalTokenIdxBase for RegionPath {
    fn regional_token_idx_base(self, db: &salsa::Db) -> Option<RegionalTokenIdxBase> {
        match self {
            RegionPath::Snippet(_) => Some(RegionalTokenIdxBase::new_snippet()),
            RegionPath::Decl(slf) => Some(slf.syn_node_path(db).decl_regional_token_idx_base(db)),
            RegionPath::Defn(slf) => slf.syn_node_path(db).defn_regional_token_idx_base(db),
        }
    }
}
