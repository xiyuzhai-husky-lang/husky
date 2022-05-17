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
            ty: EntityRoutePtr::Root(RootIdentifier::Type),
        }
    }

    pub(crate) fn from_input(
        db: &dyn InferQualifiedTyQueryGroup,
        input_contract: InputContract,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(LazyQualifiedTy::new(
            LazyQualifier::from_input(input_contract, db.is_copyable(ty)?),
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
        output_contract: OutputLiason,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicitly_castable(self.ty, output_ty) {
            return false;
        }
        match output_contract {
            OutputLiason::Transfer => match self.qual {
                LazyQualifier::Copyable => true,
                LazyQualifier::PureRef => todo!(),
                LazyQualifier::GlobalRef => todo!(),
                LazyQualifier::Transient => true,
            },
            OutputLiason::MemberAccess => todo!(),
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
}

impl LazyQualifier {
    pub fn from_input(input_contract: InputContract, is_copyable: bool) -> Self {
        match input_contract {
            InputContract::Pure => {
                if is_copyable {
                    LazyQualifier::Copyable
                } else {
                    LazyQualifier::PureRef
                }
            }
            InputContract::GlobalRef => LazyQualifier::GlobalRef,
            InputContract::Move => todo!(),
            InputContract::BorrowMut => todo!(),
            InputContract::MoveMut => todo!(),
            InputContract::Exec => todo!(),
            InputContract::MemberAccess => todo!(),
        }
    }

    pub fn from_field(
        this_qual: LazyQualifier,
        field_contract: FieldContract,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            LazyQualifier::Copyable
        } else {
            todo!()
        })
    }
}
