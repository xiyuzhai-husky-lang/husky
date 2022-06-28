pub mod trivial;

pub const SYNTHETIC_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "synthetic",
    items: &[trivial::TRIVIAL_MODULE_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};
use crate::*;
