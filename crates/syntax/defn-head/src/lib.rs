mod generic;

pub use generic::*;
use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use text::RangedCustomIdentifier;
use vm::{InputLiason, MemberLiason, OutputLiason};
use word::{CustomIdentifier, IdentDict, Paradigm};

// function or method
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallableDefnHead {
    pub ident: RangedCustomIdentifier,
    pub paradigm: Paradigm,
    pub generic_parameters: IdentDict<SpatialParameter>,
    pub parameters: Arc<Vec<InputParameter>>,
    pub output_ty: RangedEntityRoute,
    pub output_liason: OutputLiason,
    pub opt_this_contract: Option<InputLiason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputParameter {
    pub ranged_ident: RangedCustomIdentifier,
    pub liason: InputLiason,
    pub ranged_ty: RangedEntityRoute,
}
