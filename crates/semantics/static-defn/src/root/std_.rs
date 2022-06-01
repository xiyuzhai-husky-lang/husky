mod ops;
mod slice;

pub use ops::*;
pub use slice::*;

use crate::*;

pub static STD_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "std",
    subscopes: &[("ops", &STD_OPS_DEFN), ("slice", &STD_SLICE_DEFN)],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};
