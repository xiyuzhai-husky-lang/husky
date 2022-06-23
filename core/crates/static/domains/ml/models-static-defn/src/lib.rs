use liason::*;
use static_defn::{static_mod, EntityStaticDefn, EntityStaticDefnVariant};
use visual_syntax::{primitive_visualizer, StaticVisualTy};

// static_mod! { models = {} }
// Recursive expansion of static_mod! macro
// =========================================

pub static MODELS_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "models",
    items: &[],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::StaticDevSource { file: "", line: 0 },
};
