mod __rust_code_gen__;
mod naive;

pub use naive::*;

use husky_dev_utils::*;
use husky_liason_semantics::*;
use husky_static_visualizer::StaticVisualTy;
use static_defn::{static_mod, EntityStaticDefn, EntityStaticDefnVariant};

static_mod! { models = { naive } }
