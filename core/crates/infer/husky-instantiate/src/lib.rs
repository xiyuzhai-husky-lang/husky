mod context;
mod impl_arc;
mod impl_entity_route;
mod impl_hash_set;
mod impl_spatial_arguments;
mod impl_spatial_parameter;
mod impl_thin_vec;
mod impl_vec_set;

pub use context::*;

use check_utils::should_eq;
use defn_head::{GenericPlaceholderVariant, SpatialParameter};
use husky_entity_route::*;
use husky_entity_syntax::*;
use map_collect::MapCollect;
use print_utils::p;
use thin_vec::{thin_vec, ThinVec};
use word::CustomIdentifier;

pub trait Instantiable {
    type Target;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target;
}

pub trait FilterInstantiable {
    type Target;

    fn instantiate(&self, ctx: &InstantiationContext) -> Option<Self::Target>;
}
