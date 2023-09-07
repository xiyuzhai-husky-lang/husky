pub mod paths;
pub mod token_region;
mod trai_available;
mod trai_impl;
mod trai_in_use;

pub use self::trai_available::*;
pub use self::trai_impl::*;
pub use self::trai_in_use::*;

use crate::*;
use smallvec::{smallvec, SmallVec};
use vec_like::{OrderedSmallVecSet, SmallVecMap, SmallVecPairMap, SmallVecSet};

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct TraitOrderedSet {
    data: OrderedSmallVecSet<TraitPath, 8>,
}
