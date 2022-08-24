use husky_entity_route::{
    CanonicalEntityRoutePtrKind, EntityRoutePtr, EntityRouteVariant, TemporalArgument,
};
use husky_text::TextRange;
use husky_word::{LiasonKeyword, RootIdentifier};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParameterModifier {
    None,
    Move,
    MoveMut,
    MemberAccess,
    EvalRef,
    TempRef,
    TempRefMut,
}

impl ParameterModifier {
    pub fn is_compatible(self, ty: EntityRoutePtr) -> bool {
        match self {
            ParameterModifier::None => true,
            ParameterModifier::Move => true,
            ParameterModifier::MoveMut => {
                let canonical_ty = ty.canonicalize();
                match canonical_ty.kind() {
                    CanonicalEntityRoutePtrKind::Intrinsic => true,
                    CanonicalEntityRoutePtrKind::Optional => todo!(),
                    CanonicalEntityRoutePtrKind::EvalRef => todo!(),
                    CanonicalEntityRoutePtrKind::OptionalEvalRef => todo!(),
                    CanonicalEntityRoutePtrKind::TempRefMut => todo!(),
                }
            }
            ParameterModifier::MemberAccess => todo!(),
            ParameterModifier::EvalRef => {
                let canonical_ty = ty.canonicalize();
                match canonical_ty.kind() {
                    CanonicalEntityRoutePtrKind::Intrinsic => false,
                    CanonicalEntityRoutePtrKind::Optional => todo!(),
                    CanonicalEntityRoutePtrKind::EvalRef => todo!(),
                    CanonicalEntityRoutePtrKind::OptionalEvalRef => todo!(),
                    CanonicalEntityRoutePtrKind::TempRefMut => todo!(),
                }
            }
            ParameterModifier::TempRef => todo!(),
            ParameterModifier::TempRefMut => {
                let canonical_ty = ty.canonicalize();
                match canonical_ty.kind() {
                    CanonicalEntityRoutePtrKind::Intrinsic => false,
                    CanonicalEntityRoutePtrKind::Optional => todo!(),
                    CanonicalEntityRoutePtrKind::EvalRef => todo!(),
                    CanonicalEntityRoutePtrKind::OptionalEvalRef => todo!(),
                    CanonicalEntityRoutePtrKind::TempRefMut => true,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RangedParameterLiason {
    pub liason: ParameterModifier,
    pub opt_range: Option<TextRange>,
}

impl From<ParameterModifier> for RangedParameterLiason {
    fn from(liason: ParameterModifier) -> Self {
        Self {
            liason,
            opt_range: None,
        }
    }
}

impl ParameterModifier {
    pub fn from_member(
        member_liason: MemberLiason,
        member_ty: EntityRoutePtr,
        is_copyable: bool,
    ) -> ParameterModifier {
        match member_liason {
            MemberLiason::Immutable => {
                if is_copyable {
                    ParameterModifier::None
                } else {
                    ParameterModifier::Move
                }
            }
            MemberLiason::Mutable => {
                if is_copyable {
                    ParameterModifier::None
                } else {
                    ParameterModifier::MoveMut
                }
            }
            MemberLiason::DerivedLazy => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OutputLiason {
    Transfer,
    MemberAccess { member_liason: MemberLiason },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MemberLiason {
    Immutable,
    Mutable,
    DerivedLazy,
}

impl std::fmt::Display for MemberLiason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemberLiason::Immutable => "immutable",
            MemberLiason::Mutable => "mutable",
            MemberLiason::DerivedLazy => "derived",
        }
        .fmt(f)
    }
}

impl MemberLiason {
    pub fn from_opt_keyword(opt_keyword: Option<LiasonKeyword>) -> MemberLiason {
        match opt_keyword {
            Some(liason_keyword) => match liason_keyword {
                LiasonKeyword::Mut => MemberLiason::Mutable,
            },
            None => MemberLiason::Immutable,
        }
    }

    pub fn mutable(self) -> bool {
        match self {
            MemberLiason::Immutable | MemberLiason::DerivedLazy => false,
            MemberLiason::Mutable => true,
        }
    }
}
