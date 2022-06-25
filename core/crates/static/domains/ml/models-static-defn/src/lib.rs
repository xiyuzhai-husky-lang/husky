mod naive;

pub use naive::*;

use liason::*;
use static_defn::{static_mod, EntityStaticDefn, EntityStaticDefnVariant};
use visual_syntax::{primitive_visualizer, StaticVisualTy};

static_mod! { models = { naive } }
