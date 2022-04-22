pub mod real1d;
pub mod real2d;
pub const TRIVIAL_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "trivial",
    subscopes: &[
        ("real1d", real1d::REAL_1D_SCOPE_DATA),
        ("real2d", real2d::REAL_2D_MOD_DEFN),
    ],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

use crate::*;
