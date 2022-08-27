use husky_entity_route::{CanonicalTyKind, EntityRoutePtr, EntityRouteVariant, TemporalArgument};
use husky_text::TextRange;
use husky_word::{LiasonKeyword, RootIdentifier};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParameterModifier {
    None,
    Owned,
    OwnedMut,
    MemberAccess,
    EvalRef,
    TempRef,
    TempRefMut,
}

impl ParameterModifier {
    pub fn is_compatible(self, ty: EntityRoutePtr) -> bool {
        match self {
            ParameterModifier::None => true,
            ParameterModifier::Owned => true,
            ParameterModifier::OwnedMut => {
                let canonical_ty = ty.canonicalize();
                match canonical_ty.kind() {
                    CanonicalTyKind::Intrinsic => true,
                    CanonicalTyKind::Optional => todo!(),
                    CanonicalTyKind::EvalRef => todo!(),
                    CanonicalTyKind::OptionalEvalRef => todo!(),
                    CanonicalTyKind::TempRefMut => todo!(),
                }
            }
            ParameterModifier::MemberAccess => todo!(),
            ParameterModifier::EvalRef => {
                let canonical_ty = ty.canonicalize();
                match canonical_ty.kind() {
                    CanonicalTyKind::Intrinsic => false,
                    CanonicalTyKind::Optional => todo!(),
                    CanonicalTyKind::EvalRef => todo!(),
                    CanonicalTyKind::OptionalEvalRef => todo!(),
                    CanonicalTyKind::TempRefMut => todo!(),
                }
            }
            ParameterModifier::TempRef => todo!(),
            ParameterModifier::TempRefMut => {
                let canonical_ty = ty.canonicalize();
                match canonical_ty.kind() {
                    CanonicalTyKind::Intrinsic => false,
                    CanonicalTyKind::Optional => todo!(),
                    CanonicalTyKind::EvalRef => todo!(),
                    CanonicalTyKind::OptionalEvalRef => todo!(),
                    CanonicalTyKind::TempRefMut => true,
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
        member_liason: MemberModifier,
        member_ty: EntityRoutePtr,
        is_copyable: bool,
    ) -> ParameterModifier {
        match member_liason {
            MemberModifier::Immutable => {
                if is_copyable {
                    ParameterModifier::None
                } else {
                    ParameterModifier::Owned
                }
            }
            MemberModifier::Mutable => {
                if is_copyable {
                    ParameterModifier::None
                } else {
                    ParameterModifier::OwnedMut
                }
            }
            MemberModifier::Property => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OutputModifier {
    Transfer,
    MemberAccess { member_liason: MemberModifier },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MemberModifier {
    Immutable,
    Mutable,
    Property,
}

impl std::fmt::Display for MemberModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemberModifier::Immutable => "immutable",
            MemberModifier::Mutable => "mutable",
            MemberModifier::Property => "property",
        }
        .fmt(f)
    }
}

impl MemberModifier {
    pub fn from_opt_keyword(opt_keyword: Option<LiasonKeyword>) -> MemberModifier {
        match opt_keyword {
            Some(liason_keyword) => match liason_keyword {
                LiasonKeyword::Mut => MemberModifier::Mutable,
            },
            None => MemberModifier::Immutable,
        }
    }

    pub fn allow_mutable(self) -> bool {
        match self {
            MemberModifier::Immutable | MemberModifier::Property => false,
            MemberModifier::Mutable => true,
        }
    }
}
