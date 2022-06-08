mod spatial;

use liason::{MemberLiason, OutputLiason, ParameterLiason, RangedParameterLiason};
pub use spatial::*;
use std::sync::Arc;

use entity_route::{EntityRouteKind, EntityRoutePtr, RangedEntityRoute};
use text::RangedCustomIdentifier;
use word::{CustomIdentifier, IdentDict, Paradigm, RootIdentifier};

// function or method
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallableDefnHead {
    pub ident: RangedCustomIdentifier,
    pub paradigm: Paradigm,
    pub generic_parameters: IdentDict<SpatialParameter>,
    pub parameters: Arc<Vec<Parameter>>,
    pub output_ty: RangedEntityRoute,
    pub output_liason: OutputLiason,
    pub opt_this_liason: Option<ParameterLiason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub ranged_ident: RangedCustomIdentifier,
    pub ranged_liason: RangedParameterLiason,
    pub ranged_ty: RangedEntityRoute,
}

impl Parameter {
    pub fn new(
        ranged_ident: RangedCustomIdentifier,
        ranged_liason: RangedParameterLiason,
        ranged_ty: RangedEntityRoute,
    ) -> Self {
        match ranged_ty.route.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => panic!(),
            _ => (),
        }
        Self {
            ranged_ident,
            ranged_liason,
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
            ranged_liason: RangedParameterLiason {
                liason: ParameterLiason::from_member(liason, member_ty, is_member_ty_copyable),
                opt_range: None,
            },
            ranged_ty: RangedEntityRoute {
                route: member_ty.deref_route(),
                range: Default::default(),
            },
        }
    }
}
