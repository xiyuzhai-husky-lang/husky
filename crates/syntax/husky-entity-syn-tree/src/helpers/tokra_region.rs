mod decl;
mod defn;
mod snippet;

pub use self::decl::*;
pub use self::defn::*;
pub use self::snippet::*;

use crate::*;
use husky_token::{Token, TokenIdx, TokenSheet};
use std::num::NonZeroU32;

#[enum_class::from_variants]
pub enum TokraRegionData<'a> {
    Snippet(SnippetTokraRegionData<'a>),
    Decl(DeclTokraRegionData<'a>),
    Defn(DefnTokraRegionData<'a>),
}

// base 1
pub struct RegionalTokenIdx(NonZeroU32);

pub struct RegionalAstIdx(NonZeroU32);

impl<'a> std::ops::Index<RegionalTokenIdx> for TokraRegionData<'a> {
    type Output = Token;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        match self {
            TokraRegionData::Snippet(token_region) => &token_region[index],
            TokraRegionData::Decl(token_region) => &token_region[index],
            TokraRegionData::Defn(token_region) => &token_region[index],
        }
    }
}

pub fn token_region_data(path: RegionPath, db: &dyn EntitySynTreeDb) -> TokraRegionData {
    match path {
        RegionPath::Snippet(_) => todo!(),
        RegionPath::Decl(syn_node_path) => decl_tokra_region(syn_node_path, db).data(db).into(),
        RegionPath::Defn(syn_node_path) => defn_token_region(syn_node_path, db).data(db).into(),
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
