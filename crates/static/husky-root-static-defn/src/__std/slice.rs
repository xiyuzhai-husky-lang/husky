mod cyclic_slice_;
mod full_slice;

pub use cyclic_slice::CyclicSlice;
pub use cyclic_slice_::*;

use crate::*;
use husky_dev_utils::static_dev_src;

pub static STD_SLICE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "slice",
    items: &[&STD_SLICE_CYCLIC_SLICE_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::static_dev_src!(),
};
