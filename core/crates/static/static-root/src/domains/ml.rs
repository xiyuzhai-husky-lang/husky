use super::*;

pub static ML_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ml",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Module,
    dev_src: static_dev_src!(),
};
