mod cyclic_slice;

pub use cyclic_slice::*;

use crate::*;
use dev_utils::static_dev_src;
use vm::{InputLiason, MemberLiason, OutputLiason};

pub static STD_SLICE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "slice",
    subscopes: &[("CyclicSlice", &STD_SLICE_CYCLIC_SLICE_DEFN)],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};
