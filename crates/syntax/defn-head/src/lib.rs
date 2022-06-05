mod generic;

pub use generic::*;
use liason::{MemberLiason, OutputLiason, ParameterLiason};
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

impl Parameter {
    pub fn new(ranged_ident: RangedCustomIdentifier, ranged_ty: RangedEntityRoute) -> Self {
        let liason = ParameterLiason::new(ranged_ty.route);
        Self {
            ranged_ident,
            liason,
            ranged_ty,
        }
    }

    pub fn from_member(
        ranged_ident: RangedCustomIdentifier,
        liason: MemberLiason,
        member_ty: EntityRoutePtr,
        is_member_ty_copyable: bool,
    ) -> Self {
        Parameter {
            ranged_ident,
            liason: ParameterLiason::from_member(liason, member_ty, is_member_ty_copyable),
            ranged_ty: RangedEntityRoute {
                route: member_ty.deref_route(),
                range: Default::default(),
            },
        }
    }
}
