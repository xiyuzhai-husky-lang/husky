mod decl;
mod defn;
mod snippet;

pub use self::decl::*;
pub use self::defn::*;
pub use self::snippet::*;

use crate::*;
use husky_regional_token::*;
use husky_token::{TokenIdx, TokenSheet};

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

pub fn token_region_data(path: RegionPath, db: &dyn EntitySynTreeDb) -> Option<TokraRegionData> {
    todo!()
}

impl RegionPath {
    pub fn regional_token_idx_base(self, db: &dyn EntitySynTreeDb) -> Option<RegionalTokenIdxBase> {
        match self {
            RegionPath::Snippet(slf) => todo!(),
            RegionPath::Decl(slf) => Some(slf.decl_regional_token_idx_base(db)),
            RegionPath::Defn(slf) => slf.defn_regional_token_idx_base(db),
        }
    }
}

pub struct TokraBase {
    // add this to get TokenIdx
    token_idx_base: u32,
    token_group_idx_base: u32,
}

impl TokraBase {
    fn token_idx(self, regional_token_idx: RegionalTokenIdx) -> TokenIdx {
        todo!()
    }
}
