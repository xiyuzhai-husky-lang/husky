use crate::*;
use infer_error::derived;
use std::fmt::Write;
use test_utils::TestComparable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyQualifiedType {
    pub qual: LazyQualifier,
    pub ty: EntityRoutePtr,
}

impl TestComparable for LazyQualifiedType {
    fn write_inherent(&self, result: &mut String) {
        write!(result, "{: <12?} {:?}", self.qual, self.ty).unwrap()
    }
}

impl LazyQualifiedType {
    pub fn new(qual: LazyQualifier, ty: EntityRoutePtr) -> Self {
        msg_once!("handle ref");
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
        output_contract: OutputContract,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicit_convertible(self.ty, output_ty) {
            return false;
        }
        match output_contract {
            OutputContract::Transfer => match self.qual {
                LazyQualifier::Copyable => true,
                LazyQualifier::PureRef => todo!(),
                LazyQualifier::GlobalRef => todo!(),
                LazyQualifier::Transient => todo!(),
            },
            OutputContract::MemberAccess => todo!(),
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
        todo!()
    }
}
