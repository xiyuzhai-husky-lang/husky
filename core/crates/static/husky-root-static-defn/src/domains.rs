pub mod gui;
pub mod ml;
pub mod prover;
pub mod rl;

use ml::*;

use crate::*;

pub static DOMAINS_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "domains",
    items: &[&ML_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: static_dev_src!(),
};
