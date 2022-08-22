use husky_entity_route::{EntityRoutePtr, EntityRouteVariant, TemporalArgument};
use husky_text::TextRange;
use husky_word::{LiasonKeyword, RootIdentifier};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParameterLiason {
    Pure,
    Move,
    MoveMut,
    MemberAccess,
    EvalRef,
    TempRef,
    TempRefMut,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RangedParameterLiason {
    pub liason: ParameterLiason,
    pub opt_range: Option<TextRange>,
}

impl From<ParameterLiason> for RangedParameterLiason {
    fn from(liason: ParameterLiason) -> Self {
        Self {
            liason,
            opt_range: None,
        }
    }
}

impl ParameterLiason {
    pub fn new(ty: EntityRoutePtr) -> Self {
        match ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => {
                if ty.temporal_arguments.len() == 0
                    || ty.temporal_arguments[0] == TemporalArgument::Eval
                {
                    ParameterLiason::EvalRef
                } else {
                    ParameterLiason::TempRef
                }
            }
            _ => ParameterLiason::Pure,
        }
    }

    pub fn from_member(
        member_liason: MemberLiason,
        member_ty: EntityRoutePtr,
        is_copyable: bool,
    ) -> ParameterLiason {
        match member_ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => {
                if member_ty.temporal_arguments.len() == 0
                    || member_ty.temporal_arguments[0] == TemporalArgument::Eval
                {
                    ParameterLiason::EvalRef
                } else {
                    ParameterLiason::TempRef
                }
            }
            _ => match member_liason {
                MemberLiason::Immutable => {
                    if is_copyable {
                        ParameterLiason::Pure
                    } else {
                        ParameterLiason::Move
                    }
                }
                MemberLiason::Mutable => {
                    if is_copyable {
                        ParameterLiason::Pure
                    } else {
                        ParameterLiason::MoveMut
                    }
                }
                MemberLiason::DerivedLazy => panic!(),
            },
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
