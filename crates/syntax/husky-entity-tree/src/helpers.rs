mod available_trai_item;
pub mod ingredient;
pub mod jar;
pub mod paths;
pub mod tokra_region;
mod trai_impl;

pub use self::available_trai_item::*;
pub use self::trai_impl::*;

use crate::*;
use smallvec::{smallvec, SmallVec};
use vec_like::{OrderedSmallVecSet, SmallVecPairMap};

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TraitOrderedSet {
    data: OrderedSmallVecSet<TraitPath, 8>,
}
