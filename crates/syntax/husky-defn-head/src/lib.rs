mod spatial;

use husky_check_utils::should;
use husky_entity_syntax::EntitySyntaxQueryGroup;
use husky_liason_semantics::{MemberModifier, ParameterModifier, RangedParameterLiason};
pub use spatial::*;
use thin_vec::thin_vec;

use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_text::{RangedCustomIdentifier, TextRange};
use husky_word::{CustomIdentifier, RootBuiltinIdentifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    ranged_ident: RangedCustomIdentifier,
    ranged_liason: RangedParameterLiason,
    ranged_book_ty: RangedEntityRoute,
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
            ranged_book_ty: ranged_raw_ty,
        }
    }

    fn synthesize_ty(
        db: &dyn EntitySyntaxQueryGroup,
        liason: ParameterModifier,
        raw_ty: EntityRoutePtr,
    ) -> EntityRoutePtr {
        match liason {
            ParameterModifier::None => raw_ty,
            ParameterModifier::Owned => raw_ty,
            ParameterModifier::OwnedMut => raw_ty,
            ParameterModifier::MemberAccess => todo!(),
            ParameterModifier::EvalRef => todo!(),
            ParameterModifier::TempRef => todo!(),
            ParameterModifier::TempRefMut => db.route_call(
                RootBuiltinIdentifier::RefMut.into(),
                thin_vec![raw_ty.into()],
            ),
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
        self.ranged_book_ty.route
    }

    pub fn raw_ty_range(&self) -> TextRange {
        self.ranged_book_ty.range
    }

    pub fn ty(&self) -> EntityRoutePtr {
        self.ty
    }

    pub fn from_field(
        db: &dyn EntitySyntaxQueryGroup,
        ranged_ident: RangedCustomIdentifier,
        modifier: MemberModifier,
        member_ty: EntityRoutePtr,
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
                liason: ParameterModifier::from_field(modifier),
                opt_range: None,
            },
            RangedEntityRoute {
                route: member_ty,
                range: Default::default(),
            },
        )
    }
}
