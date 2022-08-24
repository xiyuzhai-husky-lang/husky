mod spatial;

use husky_check_utils::should;
use husky_entity_syntax::EntitySyntaxQueryGroup;
use husky_liason_semantics::{
    MemberLiason, OutputLiason, ParameterModifier, RangedParameterLiason,
};
use husky_print_utils::p;
pub use spatial::*;
use std::sync::Arc;
use thin_vec::thin_vec;

use husky_entity_route::{
    EntityRoutePtr, EntityRouteVariant, InternEntityRoute, RangedEntityRoute,
};
use husky_text::{RangedCustomIdentifier, TextRange};
use husky_word::{CustomIdentifier, IdentDict, Paradigm, RootIdentifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    ranged_ident: RangedCustomIdentifier,
    ranged_liason: RangedParameterLiason,
    ranged_raw_ty: RangedEntityRoute,
    ty: EntityRoutePtr,
}

impl Parameter {
    pub fn new(
        db: &dyn EntitySyntaxQueryGroup,
        ranged_ident: RangedCustomIdentifier,
        ranged_liason: RangedParameterLiason,
        ranged_raw_ty: RangedEntityRoute,
    ) -> Self {
        let ty = Self::synthesize_ty(db, ranged_liason.liason, ranged_raw_ty.route);
        assert!(ranged_liason.liason.is_compatible(ty));
        Self {
            ty,
            ranged_ident,
            ranged_liason,
            ranged_raw_ty,
        }
    }

    fn synthesize_ty(
        db: &dyn EntitySyntaxQueryGroup,
        liason: ParameterModifier,
        raw_ty: EntityRoutePtr,
    ) -> EntityRoutePtr {
        match liason {
            ParameterModifier::None => raw_ty,
            ParameterModifier::Move => raw_ty,
            ParameterModifier::MoveMut => raw_ty,
            ParameterModifier::MemberAccess => todo!(),
            ParameterModifier::EvalRef => todo!(),
            ParameterModifier::TempRef => todo!(),
            ParameterModifier::TempRefMut => {
                db.route_call(RootIdentifier::RefMut.into(), thin_vec![raw_ty.into()])
            }
        }
    }

    pub fn liason(&self) -> ParameterModifier {
        self.ranged_liason.liason
    }
    pub fn liason_range(&self) -> TextRange {
        self.ranged_liason.opt_range.unwrap()
    }

    pub fn ranged_ident(&self) -> RangedCustomIdentifier {
        self.ranged_ident
    }

    pub fn ident(&self) -> CustomIdentifier {
        self.ranged_ident.ident
    }

    pub fn raw_ty(&self) -> EntityRoutePtr {
        self.ranged_raw_ty.route
    }

    pub fn raw_ty_range(&self) -> TextRange {
        self.ranged_raw_ty.range
    }

    pub fn ty(&self) -> EntityRoutePtr {
        self.ty
    }

    pub fn from_member(
        db: &dyn EntitySyntaxQueryGroup,
        ranged_ident: RangedCustomIdentifier,
        liason: MemberLiason,
        member_ty: EntityRoutePtr,
        is_member_ty_copyable: bool,
    ) -> Self {
        should!(ranged_ident
            .ident
            .as_str()
            .chars()
            .next()
            .unwrap()
            .is_lowercase());
        Parameter::new(
            db,
            ranged_ident,
            RangedParameterLiason {
                liason: ParameterModifier::from_member(liason, member_ty, is_member_ty_copyable),
                opt_range: None,
            },
            RangedEntityRoute {
                route: member_ty,
                range: Default::default(),
            },
        )
    }
}
