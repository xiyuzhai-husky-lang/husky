mod calculus;
mod context;
mod intern;
mod query;
mod universe;

pub use context::*;
pub use intern::*;
pub use query::*;
pub use universe::*;

use husky_entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Entity(EntityRoutePtr),
    TemplateInstantiation {
        template: EntityRoutePtr,
        temporal_arguments: Vec<TemporalArgument>,
        spatial_arguments: Vec<SpatialArgument>,
    },
    ThickFp {
        regular_parameters: Vec<RegularParameter>,
        keyword_parameters: Vec<KeywordParameter>,
        variadic_parameter: VariadicParameter,
    },
    Template {
        temporal_parameters: Vec<TemporalParameter>,
        spatial_parameters: Vec<SpatialParameter>,
        ty: TyPtr,
    },
    Prop, // reserved
    Type(Universe),
}

pub struct RegularParameter {}

pub struct KeywordParameter {}
pub enum VariadicParameter {
    None,
}

pub struct TemporalParameter {
    route: EntityRoutePtr,
}

pub struct SpatialParameter {
    route: EntityRoutePtr,
}

pub enum TemporalArgument {
    Eval,
    Custom(/* todo */),
}

pub enum SpatialArgument {
    PhysicalTy(TyPtr),
    ConstUsize, //ad hoc
}
