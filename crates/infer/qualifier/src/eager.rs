use crate::*;
use std::fmt::Write;
use test_utils::{TestCompareConfig, TestDisplay};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerQualifiedTy {
    pub qual: EagerQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for EagerQualifiedTy {
    fn write_inherent(&self, config: TestCompareConfig, result: &mut String) {
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
    pub(crate) fn from_input(
        db: &dyn InferQualifiedTyQueryGroup,
        input_contract: InputContract,
        ty: EntityRoutePtr,
    ) -> Self {
        EagerQualifiedTy::new(
            EagerQualifier::from_input(input_contract, db.is_copyable(ty)),
            ty,
        )
    }

    pub(crate) fn new(qual: EagerQualifier, ty: EntityRoutePtr) -> Self {
        msg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn use_for_init(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let => match self.qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => EagerQualifier::Copyable,
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::LocalRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => todo!(),
                EagerQualifier::GlobalRef => todo!(),
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
            },
            InitKind::Decl => match self.qual {
                EagerQualifier::Copyable => todo!(),
                EagerQualifier::CopyableMut => todo!(),
                EagerQualifier::PureRef => todo!(),
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => panic!(),
            },
        };
        Ok(Self { qual, ty: self.ty })
    }

    pub(crate) fn is_implicitly_convertible_to_output(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        output_contract: OutputContract,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicit_convertible(self.ty, output_ty) {
            return false;
        }
        match output_contract {
            OutputContract::Transfer => match self.qual {
                EagerQualifier::PureRef | EagerQualifier::LocalRef => false,
                EagerQualifier::Transient
                | EagerQualifier::Copyable
                | EagerQualifier::CopyableMut
                | EagerQualifier::Owned
                | EagerQualifier::OwnedMut => true,
                EagerQualifier::GlobalRef => todo!(),
            },
            OutputContract::MemberAccess => todo!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerQualifier {
    Copyable,
    CopyableMut,
    PureRef,
    GlobalRef,
    LocalRef,
    Transient,
    Owned,
    OwnedMut,
}

impl std::fmt::Debug for EagerQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            Self::Copyable => "Copyable",
            Self::CopyableMut => "CopyableMut",
            Self::PureRef => "PureRef",
            Self::GlobalRef => "GlobalRef",
            Self::LocalRef => "LocalRef",
            Self::Transient => "Transient",
            Self::Owned => "Owned",
            Self::OwnedMut => "OwnedMut",
        })
    }
}

impl EagerQualifier {
    pub fn binding(self, contract: EagerContract) -> Binding {
        match self {
            EagerQualifier::PureRef => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => todo!(),
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
            },
            EagerQualifier::LocalRef => todo!(),
            EagerQualifier::Transient => todo!(),
            EagerQualifier::Copyable => Binding::Copy,
            EagerQualifier::CopyableMut => match contract {
                EagerContract::Pure => Binding::Copy,
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => todo!(),
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::UseMemberForLetInit => panic!(),
                EagerContract::UseMemberForVarInit => panic!(),
                EagerContract::Return => Binding::Copy,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => Binding::Copy,
                EagerContract::Exec => todo!(),
            },
            EagerQualifier::Owned => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::GlobalRef => panic!(),
                EagerContract::Move => todo!(),
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => Binding::Move,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
            },
            EagerQualifier::OwnedMut => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => Binding::Move,
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::UseMemberForLetInit => Binding::Ref,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => Binding::Move,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
            },
            EagerQualifier::GlobalRef => todo!(),
        }
    }

    pub fn from_input(input_contract: InputContract, is_copyable: bool) -> Self {
        match input_contract {
            InputContract::Pure => {
                if is_copyable {
                    EagerQualifier::Copyable
                } else {
                    EagerQualifier::PureRef
                }
            }
            InputContract::GlobalRef => todo!(),
            InputContract::Move => todo!(),
            InputContract::BorrowMut => todo!(),
            InputContract::MoveMut => todo!(),
            InputContract::Exec => todo!(),
            InputContract::MemberAccess => todo!(),
        }
    }

    pub fn from_output(output_contract: OutputContract, is_copyable: bool) -> Self {
        match output_contract {
            OutputContract::Transfer => Self::transitive(is_copyable),
            OutputContract::MemberAccess => todo!(),
        }
    }

    pub fn transitive(is_copyable: bool) -> Self {
        if is_copyable {
            EagerQualifier::Copyable
        } else {
            EagerQualifier::Transient
        }
    }
}
