mod context;
mod impl_arc;
mod impl_entity_route;
mod impl_hash_set;
mod impl_spatial_arguments;
mod impl_spatial_parameter;
mod impl_thin_vec;
mod impl_vec_set;

pub use context::*;

use defn_head::{GenericPlaceholderVariant, SpatialParameter};
use husky_check_utils::should_eq;
use husky_entity_route::*;
use husky_entity_syntax::*;
use husky_print_utils::p;
use husky_word::CustomIdentifier;
use map_collect::MapCollect;
use thin_vec::{thin_vec, ThinVec};

pub trait Instantiable {
    type Target;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target;
}

pub trait FilterInstantiable {
    type Target;

    fn instantiate(&self, ctx: &InstantiationContext) -> Option<Self::Target>;
}
