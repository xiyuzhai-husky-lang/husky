pub mod ops;
pub mod slice;

use ops::*;
use slice::*;

use crate::*;

pub static STD_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "std",
    items: &[&STD_OPS_DEFN, &STD_SLICE_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::__static_dev_src!(),
};
