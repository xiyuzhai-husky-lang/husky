mod decl;
mod defn;
mod snippet;

pub use self::decl::*;
pub use self::defn::*;
pub use self::snippet::*;

use crate::*;
use husky_token::{Token, TokenSheet};
use std::num::NonZeroU32;

#[enum_class::from_variants]
pub enum TokenRegionData<'a> {
    Snippet(SnippetTokenRegionData<'a>),
    Decl(DeclTokenRegionData<'a>),
    Defn(DefnTokenRegionData<'a>),
}

// base 1
pub struct RegionalTokenIdx(NonZeroU32);

impl<'a> std::ops::Index<RegionalTokenIdx> for TokenRegionData<'a> {
    type Output = Token;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        match self {
            TokenRegionData::Snippet(token_region) => &token_region[index],
            TokenRegionData::Decl(token_region) => &token_region[index],
            TokenRegionData::Defn(token_region) => &token_region[index],
        }
    }
}

pub fn token_region_data(path: RegionPath, db: &dyn EntitySynTreeDb) -> TokenRegionData {
    match path {
        RegionPath::Snippet(_) => todo!(),
        RegionPath::Decl(syn_node_path) => decl_token_region(syn_node_path, db).data(db).into(),
        RegionPath::Defn(syn_node_path) => defn_token_region(syn_node_path, db).data(db).into(),
    }
}
