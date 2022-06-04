use entity_route::EntityRoutePtr;
use word::LiasonKeyword;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParameterLiason {
    Pure,
    Move,
    TempMut,
    MoveMut,
    MemberAccess,
    EvalRef,
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
    Derived,
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
            MemberLiason::Immutable | MemberLiason::Derived => false,
            MemberLiason::Mutable => true,
        }
    }

    pub fn constructor_input_liason(self, is_copyable: bool) -> ParameterLiason {
        match self {
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
            MemberLiason::Derived => panic!(),
        }
    }
}
