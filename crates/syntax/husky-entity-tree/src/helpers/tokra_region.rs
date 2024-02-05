pub mod decl;
pub mod defn;
pub mod snippet;

pub use self::decl::*;
pub use self::defn::*;
pub use self::snippet::*;

use crate::*;
use husky_entity_path::region::RegionPath;
use husky_regional_token::*;
use husky_token::TokenIdx;

/// Tokra region is for representing tokens and asts in a relative way.
/// Previously, the source code is lexed into tokens and then divided into crude asts.
/// Then we divide the source code into minimal code analysis units, called regions.
/// Token and ast data and the relative indexes against the starting token of the region,
/// is collected as Tokra Region
///
///
#[enum_class::from_variants]
pub enum TokraRegionData<'a> {
    Snippet(SnippetTokraRegionData<'a>),
    Decl(DeclTokraRegionData<'a>),
    Defn(DefnTokraRegionData<'a>),
}

impl<'a> std::ops::Index<RegionalTokenIdx> for TokraRegionData<'a> {
    type Output = TokenData;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        match self {
            TokraRegionData::Snippet(token_region) => &token_region[index],
            TokraRegionData::Decl(token_region) => &token_region[index],
            TokraRegionData::Defn(token_region) => &token_region[index],
        }
    }
}

impl SynNodeRegionPath {
    pub fn regional_token_idx_base(self, db: &::salsa::Db) -> Option<RegionalTokenIdxBase> {
        match self {
            SynNodeRegionPath::Snippet(_) => Some(RegionalTokenIdxBase::new_snippet()),
            SynNodeRegionPath::Decl(slf) => Some(slf.decl_regional_token_idx_base(db)),
            SynNodeRegionPath::Defn(slf) => slf.defn_regional_token_idx_base(db),
        }
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
