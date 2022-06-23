use super::*;
use dev_utils::static_dev_src;

static_mod! { naive = { naive_i32 } }

pub static NAIVE_I32_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "naive_i32",
    items: &[],
    variant: EntityStaticDefnVariant::Morphism,
    dev_src: static_dev_src!(),
};
