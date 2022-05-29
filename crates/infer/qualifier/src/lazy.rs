use crate::*;
use infer_error::derived;
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use word::RootIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyQualifiedTy {
    pub qual: LazyQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for LazyQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        write!(result, "{: <12?} {:?}", self.qual, self.ty).unwrap()
    }
}

impl LazyQualifiedTy {
    pub(crate) fn ty_ty() -> Self {
        Self {
            qual: LazyQualifier::GlobalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }

    pub(crate) fn from_input(
        db: &dyn InferQualifiedTyQueryGroup,
        input_liason: InputLiason,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(LazyQualifiedTy::new(
            LazyQualifier::from_input(input_liason, db.is_copyable(ty)?),
            ty,
        ))
    }

    pub fn new(qual: LazyQualifier, ty: EntityRoutePtr) -> Self {
        emsg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn use_for_init(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let | InitKind::Var => Err(derived!(
                "let or var is not allowed in lazy context".to_string()
            ))?,
            InitKind::Decl => match self.qual {
                LazyQualifier::Copyable => LazyQualifier::Copyable,
                LazyQualifier::PureRef => LazyQualifier::PureRef,
                LazyQualifier::GlobalRef | LazyQualifier::Transient => LazyQualifier::GlobalRef,
            },
        };
        Ok(Self { qual, ty: self.ty })
    }

    pub(crate) fn is_implicitly_convertible_to_output(
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
                LazyQualifier::Copyable => true,
                LazyQualifier::PureRef => todo!(),
                LazyQualifier::GlobalRef => todo!(),
                LazyQualifier::Transient => true,
            },
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyQualifier {
    Copyable,
    PureRef,
    GlobalRef,
    Transient,
}

impl LazyQualifier {
    pub fn feature(is_copyable: bool) -> LazyQualifier {
        if is_copyable {
            LazyQualifier::Copyable
        } else {
            LazyQualifier::GlobalRef
        }
    }

    pub fn binding(self, contract: LazyContract) -> Binding {
        match self {
            LazyQualifier::PureRef => match contract {
                LazyContract::Pure => Binding::Ref,
                LazyContract::GlobalRef => todo!(),
                LazyContract::Move => todo!(),
                LazyContract::Init => todo!(),
                LazyContract::UseMemberForInit => Binding::Ref,
                LazyContract::UseMemberForReturn => todo!(),
                LazyContract::Return => todo!(),
            },
            LazyQualifier::Transient => todo!(),
            LazyQualifier::Copyable => Binding::Copy,
            LazyQualifier::GlobalRef => Binding::Ref,
        }
    }

    pub fn variable_use(self, contract: LazyContract) -> InferResult<Self> {
        Ok(match self {
            LazyQualifier::Copyable => match contract {
                LazyContract::Init => todo!(),
                LazyContract::Return => LazyQualifier::Copyable,
                LazyContract::UseMemberForInit => todo!(),
                LazyContract::UseMemberForReturn => todo!(),
                LazyContract::GlobalRef => todo!(),
                LazyContract::Pure => LazyQualifier::Copyable,
                LazyContract::Move => todo!(),
            },
            LazyQualifier::PureRef => todo!(),
            LazyQualifier::GlobalRef => match contract {
                LazyContract::Init => todo!(),
                LazyContract::Return => todo!(),
                LazyContract::UseMemberForInit => LazyQualifier::GlobalRef,
                LazyContract::UseMemberForReturn => LazyQualifier::GlobalRef,
                LazyContract::GlobalRef => todo!(),
                LazyContract::Pure => LazyQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyQualifier::Transient => todo!(),
        })
    }
}

impl LazyQualifier {
    pub fn from_input(input_liason: InputLiason, is_copyable: bool) -> Self {
        match input_liason {
            InputLiason::Pure => {
                if is_copyable {
                    LazyQualifier::Copyable
                } else {
                    LazyQualifier::PureRef
                }
            }
            InputLiason::GlobalRef => LazyQualifier::GlobalRef,
            InputLiason::Move => todo!(),
            InputLiason::LocalRefMut => todo!(),
            InputLiason::MoveMut => todo!(),
            InputLiason::MemberAccess => todo!(),
        }
    }

    pub fn from_field(
        this_qual: LazyQualifier,
        field_liason: MemberLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            LazyQualifier::Copyable
        } else {
            todo!()
        })
    }

    pub fn method_opt_output_binding(
        self,
        output_liason: OutputLiason,
        output_contract: LazyContract,
        is_output_ty_copyable: bool,
    ) -> Option<Binding> {
        match output_liason {
            OutputLiason::Transfer => None,
            OutputLiason::MemberAccess { member_liason } => {
                Some(self.member_binding(member_liason, output_contract, is_output_ty_copyable))
            }
        }
    }

    pub fn member_binding(
        self,
        member_liason: MemberLiason,
        member_contract: LazyContract,
        is_member_ty_copyable: bool,
    ) -> Binding {
        if is_member_ty_copyable {
            match member_contract {
                LazyContract::Init => Binding::Copy,
                LazyContract::Return => todo!(),
                LazyContract::UseMemberForInit => todo!(),
                LazyContract::UseMemberForReturn => todo!(),
                LazyContract::GlobalRef => todo!(),
                LazyContract::Pure => Binding::Copy,
                LazyContract::Move => todo!(),
            }
        } else {
            // non-copyable
            match member_contract {
                LazyContract::Init => match self {
                    LazyQualifier::Copyable => todo!(),
                    LazyQualifier::PureRef => Binding::Ref,
                    LazyQualifier::GlobalRef => Binding::Ref,
                    LazyQualifier::Transient => Binding::Move,
                },
                LazyContract::Return => todo!(),
                LazyContract::UseMemberForInit => todo!(),
                LazyContract::UseMemberForReturn => todo!(),
                LazyContract::GlobalRef => todo!(),
                LazyContract::Pure => todo!(),
                LazyContract::Move => todo!(),
            }
        }
    }
}
