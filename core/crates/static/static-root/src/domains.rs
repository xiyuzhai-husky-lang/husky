mod gui;
mod ml;
mod prover;
mod rl;

pub use ml::*;

use crate::*;

pub static DOMAINS_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "domains",
    items: &[&ML_MODULE_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: static_dev_src!(),
};
