use husky_datasets_static_defn::DATASETS_MODULE_DEFN;
use husky_models_static_defn::MODELS_DEFN;

use super::*;

pub static ML_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ml",
    items: &[&DATASETS_MODULE_DEFN, &MODELS_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: __static_dev_src!(),
};

pub mod datasets {
    pub use husky_datasets_static_defn::*;
}

pub mod models {
    pub use husky_models_static_defn::*;
}
