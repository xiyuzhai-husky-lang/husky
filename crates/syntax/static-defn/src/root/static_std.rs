mod ops;

pub use ops::*;

use crate::*;

pub static STD_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "std",
    subscopes: &[("ops", &STD_OPS_MODULE_DEFN)],
    variant: StaticEntityDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};
