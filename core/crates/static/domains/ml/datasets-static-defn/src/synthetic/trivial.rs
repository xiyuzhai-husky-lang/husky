pub mod real1d;
pub mod real2d;
pub const TRIVIAL_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "trivial",
    items: &[real1d::REAL_1D_MODULE_DEFN, real2d::REAL_2D_MOD_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

use crate::*;
