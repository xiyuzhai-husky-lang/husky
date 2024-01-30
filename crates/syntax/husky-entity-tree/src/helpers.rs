pub mod ingredient;
pub mod jar;
pub mod paths;
pub mod tokra_region;
mod trai_available;
mod trai_impl;
mod trai_in_use;

pub use self::trai_impl::*;
pub use self::trai_in_use::*;

use crate::*;
use smallvec::{smallvec, SmallVec};
use vec_like::{OrderedSmallVecSet, SmallVecPairMap};

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct TraitOrderedSet {
    data: OrderedSmallVecSet<TraitPath, 8>,
}
