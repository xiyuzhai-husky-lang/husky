mod ops;

pub use ops::*;

use crate::*;

pub static STD_MODULE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[("ops", &STD_OPS_MODULE_DEFN)],
    decl: StaticEntityDecl::Module,
};
