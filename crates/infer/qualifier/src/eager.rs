use crate::*;
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use word::RootIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerQualifiedTy {
    pub qual: EagerQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for EagerQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <12?}{} {}{:?}{}",
                print_utils::PINK,
                self.qual,
                print_utils::RESET,
                print_utils::GREEN,
                self.ty,
                print_utils::RESET,
            )
            .unwrap()
        } else {
            write!(result, "{: <12?} {:?}", self.qual, self.ty,).unwrap()
        }
    }
}

impl EagerQualifiedTy {
    pub(crate) fn ty_qualified_ty() -> Self {
        Self {
            qual: EagerQualifier::GlobalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }
    pub(crate) fn from_input(
        db: &dyn InferQualifiedTyQueryGroup,
        input_liason: InputLiason,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(EagerQualifiedTy::new(
            EagerQualifier::from_input(input_liason, db.is_copyable(ty)?),
            ty,
        ))
    }

    pub(crate) fn new(qual: EagerQualifier, ty: EntityRoutePtr) -> Self {
        emsg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn init_variable_qual(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let => match self.qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => EagerQualifier::Copyable,
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::LocalRef => EagerQualifier::LocalRef,
                EagerQualifier::Transient | EagerQualifier::OwnedMut => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
            InitKind::Var => match self.qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => {
                    EagerQualifier::CopyableMut
                }
                EagerQualifier::PureRef => todo!(),
                EagerQualifier::LocalRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::OwnedMut,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => todo!(),
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
            InitKind::Decl => match self.qual {
                EagerQualifier::Copyable => EagerQualifier::Copyable,
                EagerQualifier::CopyableMut => panic!(),
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => panic!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
        };
        Ok(Self { qual, ty: self.ty })
    }

    pub fn is_implicitly_castable_to_output(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        output_liason: OutputLiason,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicitly_castable(self.ty, output_ty) {
            return false;
        }
        match output_liason {
            OutputLiason::Transfer => match self.qual {
                EagerQualifier::PureRef | EagerQualifier::LocalRef => false,
                EagerQualifier::Transient
                | EagerQualifier::Copyable
                | EagerQualifier::CopyableMut
                | EagerQualifier::Owned
                | EagerQualifier::OwnedMut => true,
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
            OutputLiason::MemberAccess => todo!(),
        }
    }

    pub fn as_ty(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        if !db.is_explicitly_castable(self.ty, ty)? {
            todo!()
        }
        Ok(Self {
            qual: self.qual,
            ty,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerQualifier {
    Copyable,
    CopyableMut,
    Owned,
    OwnedMut,
    PureRef,
    GlobalRef,
    LocalRef,
    LocalRefMut,
    Transient,
}

impl std::fmt::Debug for EagerQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            EagerQualifier::Copyable => "Copyable",
            EagerQualifier::CopyableMut => "CopyableMut",
            EagerQualifier::Owned => "Owned",
            EagerQualifier::OwnedMut => "OwnedMut",
            EagerQualifier::PureRef => "PureRef",
            EagerQualifier::GlobalRef => "GlobalRef",
            EagerQualifier::LocalRef => "LocalRef",
            EagerQualifier::LocalRefMut => "RefMut",
            EagerQualifier::Transient => "Transient",
        })
    }
}

impl EagerQualifier {
    pub fn mutable(&self) -> bool {
        match self {
            EagerQualifier::Copyable
            | EagerQualifier::PureRef
            | EagerQualifier::GlobalRef
            | EagerQualifier::LocalRef
            | EagerQualifier::Owned
            | EagerQualifier::Transient => false,
            EagerQualifier::CopyableMut
            | EagerQualifier::OwnedMut
            | EagerQualifier::LocalRefMut => true,
        }
    }

    pub fn binding(self, contract: EagerContract) -> Binding {
        match self {
            EagerQualifier::PureRef => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => Binding::Ref,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssign => todo!(),
            },
            EagerQualifier::LocalRef => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssign => todo!(),
            },
            EagerQualifier::Transient => todo!(),
            EagerQualifier::Copyable => Binding::Copy,
            EagerQualifier::CopyableMut => match contract {
                EagerContract::Pure => Binding::Copy,
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => Binding::Copy,
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => panic!(),
                EagerContract::UseMemberForVarInit => panic!(),
                EagerContract::Return => Binding::Copy,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => Binding::Copy,
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssign => todo!(),
            },
            EagerQualifier::Owned => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::GlobalRef => panic!(),
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => Binding::Move,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssign => todo!(),
            },
            EagerQualifier::OwnedMut => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => Binding::Move,
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => Binding::Ref,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => Binding::Move,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssign => todo!(),
            },
            EagerQualifier::GlobalRef => todo!(),
            EagerQualifier::LocalRefMut => todo!(),
        }
    }

    pub fn from_input(input_liason: InputLiason, is_copyable: bool) -> Self {
        match input_liason {
            InputLiason::Pure => {
                if is_copyable {
                    EagerQualifier::Copyable
                } else {
                    EagerQualifier::PureRef
                }
            }
            InputLiason::GlobalRef => todo!(),
            InputLiason::Move => todo!(),
            InputLiason::BorrowMut => todo!(),
            InputLiason::MoveMut => todo!(),
            InputLiason::Exec => todo!(),
            InputLiason::MemberAccess => todo!(),
        }
    }

    pub fn from_field(
        this_qual: EagerQualifier,
        field_liason: FieldLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            if this_qual.mutable() && field_liason.mutable() {
                EagerQualifier::CopyableMut
            } else {
                EagerQualifier::Copyable
            }
        } else {
            // non-copyable
            match this_qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => panic!(),
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::GlobalRef => EagerQualifier::GlobalRef,
                EagerQualifier::LocalRef => EagerQualifier::LocalRef,
                EagerQualifier::Transient => match field_liason {
                    FieldLiason::Own => todo!(),
                    FieldLiason::GlobalRef => todo!(),
                    FieldLiason::LazyOwn => todo!(),
                },
                EagerQualifier::Owned => match field_liason {
                    FieldLiason::Own => todo!(),
                    FieldLiason::GlobalRef => todo!(),
                    FieldLiason::LazyOwn => todo!(),
                },
                EagerQualifier::OwnedMut => match field_liason {
                    FieldLiason::Own => EagerQualifier::LocalRefMut,
                    FieldLiason::GlobalRef => todo!(),
                    FieldLiason::LazyOwn => todo!(),
                },
                EagerQualifier::LocalRefMut => match field_liason {
                    FieldLiason::Own => EagerQualifier::LocalRefMut,
                    FieldLiason::GlobalRef => todo!(),
                    FieldLiason::LazyOwn => todo!(),
                },
            }
        })
    }

    pub fn from_output(output_liason: OutputLiason, is_copyable: bool) -> Self {
        match output_liason {
            OutputLiason::Transfer => Self::transitive(is_copyable),
            OutputLiason::MemberAccess => todo!(),
        }
    }

    pub fn transitive(is_copyable: bool) -> Self {
        if is_copyable {
            EagerQualifier::Copyable
        } else {
            EagerQualifier::Transient
        }
    }

    pub fn element_access_qual(
        this_qual: Self,
        this_contract: EagerContract,
        is_element_copyable: bool,
    ) -> Self {
        if is_element_copyable {
            match this_contract {
                EagerContract::Pure => EagerQualifier::Copyable,
                EagerContract::GlobalRef => panic!(),
                EagerContract::Move => panic!(),
                EagerContract::UseForLetInit => EagerQualifier::Copyable,
                EagerContract::UseForVarInit => EagerQualifier::CopyableMut,
                EagerContract::UseMemberForLetInit => EagerQualifier::Copyable,
                EagerContract::UseMemberForVarInit => EagerQualifier::CopyableMut,
                EagerContract::Return => EagerQualifier::Copyable,
                EagerContract::RefMut => EagerQualifier::CopyableMut,
                EagerContract::UseForAssign => todo!(),
                EagerContract::MoveMut => panic!(),
                EagerContract::Exec => panic!(),
            }
        } else {
            match this_qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => panic!(),
                EagerQualifier::PureRef => match this_contract {
                    EagerContract::Pure => EagerQualifier::PureRef,
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => EagerQualifier::PureRef,
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => EagerQualifier::PureRef,
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssign => todo!(),
                },
                EagerQualifier::LocalRef => match this_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssign => todo!(),
                },
                EagerQualifier::Transient => match this_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssign => todo!(),
                },
                EagerQualifier::Owned => match this_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssign => todo!(),
                },
                EagerQualifier::OwnedMut => match this_contract {
                    EagerContract::Pure => EagerQualifier::PureRef,
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    // let stmt doesn't move, but create a ref instead
                    EagerContract::UseMemberForLetInit => EagerQualifier::LocalRef,
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => EagerQualifier::LocalRefMut,
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssign => todo!(),
                },
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => todo!(),
            }
        }
    }
}
