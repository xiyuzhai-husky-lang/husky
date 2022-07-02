mod naive;

pub use naive::*;

use dev_utils::*;
use husky_liason_semantics::*;
use husky_visual_syntax::{primitive_visualizer, StaticVisualTy};
use static_defn::{static_mod, EntityStaticDefn, EntityStaticDefnVariant};

static_mod! { models = { naive } }
