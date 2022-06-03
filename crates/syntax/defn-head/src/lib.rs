mod generic;

pub use generic::*;
use liason::{OutputLiason, ParameterLiason};
use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use text::RangedCustomIdentifier;
use word::{CustomIdentifier, IdentDict, Paradigm};

// function or method
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallableDefnHead {
    pub ident: RangedCustomIdentifier,
    pub paradigm: Paradigm,
    pub generic_parameters: IdentDict<SpatialParameter>,
    pub parameters: Arc<Vec<Parameter>>,
    pub output_ty: RangedEntityRoute,
    pub output_liason: OutputLiason,
    pub opt_this_contract: Option<ParameterLiason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub ranged_ident: RangedCustomIdentifier,
    pub liason: ParameterLiason,
    pub ranged_ty: RangedEntityRoute,
}
