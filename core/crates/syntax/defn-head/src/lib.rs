mod spatial;

use husky_liason_semantics::{MemberLiason, OutputLiason, ParameterLiason, RangedParameterLiason};
pub use spatial::*;
use std::sync::Arc;

use husky_entity_route::{EntityRouteKind, EntityRoutePtr, RangedEntityRoute};
use husky_text::RangedCustomIdentifier;
use word::{CustomIdentifier, IdentDict, Paradigm, RootIdentifier};

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
