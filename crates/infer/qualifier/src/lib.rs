mod builder;
mod query;
mod sheet;

use entity_route::EntityRoutePtr;
pub use query::*;
pub use sheet::*;

use ast::RawExprIdx;
use infer_error::InferResult;

use vm::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QualifiedTy {
    qual: Qualifier,
    ty: EntityRoutePtr,
}

impl QualifiedTy {
    pub(crate) fn use_for_init(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let => match self.qual {
                Qualifier::Copyable | Qualifier::CopyableMut => Qualifier::Copyable,
                Qualifier::PureRef => Qualifier::PureRef,
                Qualifier::LocalRef => todo!(),
                Qualifier::Transient => Qualifier::StackOwned,
                Qualifier::StackOwned => todo!(),
                Qualifier::StackOwnedMut => todo!(),
            },
            InitKind::Var => match self.qual {
                Qualifier::Copyable | Qualifier::CopyableMut => Qualifier::CopyableMut,
                Qualifier::PureRef => todo!(),
                Qualifier::LocalRef => todo!(),
                Qualifier::Transient => Qualifier::StackOwnedMut,
                Qualifier::StackOwned => todo!(),
                Qualifier::StackOwnedMut => todo!(),
            },
            InitKind::Decl => todo!(),
        };
        Ok(Self { qual, ty: self.ty })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Qualifier {
    Copyable,
    CopyableMut,
    PureRef,
    LocalRef,
    Transient,
    StackOwned,
    StackOwnedMut,
}

impl Qualifier {
    pub fn eager_binding(self, contract: EagerContract) -> Binding {
        match self {
            Qualifier::PureRef => match contract {
                EagerContract::Pure => todo!(),
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => todo!(),
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::BorrowMut => todo!(),
                EagerContract::TakeMut => todo!(),
                EagerContract::Exec => todo!(),
            },
            Qualifier::LocalRef => todo!(),
            Qualifier::Transient => todo!(),
            Qualifier::Copyable => todo!(),
            Qualifier::CopyableMut => todo!(),
            Qualifier::StackOwned => todo!(),
            Qualifier::StackOwnedMut => todo!(),
        }
    }

    pub fn from_input(input_contract: InputContract, is_copyable: bool) -> Self {
        match input_contract {
            InputContract::Pure => {
                if is_copyable {
                    Qualifier::Copyable
                } else {
                    Qualifier::PureRef
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
}

pub trait InferQualifier {
    fn qualified_ty_sheet(&self) -> &QualifiedTySheet;

    fn lazy_expr_qualifier_result(&self, raw_expr_idx: RawExprIdx) -> InferResult<Qualifier> {
        self.qualified_ty_sheet()
            .lazy_expr_qualifier_result(raw_expr_idx)
    }

    fn eager_expr_qualifier_result(&self, raw_expr_idx: RawExprIdx) -> InferResult<Qualifier> {
        self.qualified_ty_sheet()
            .eager_expr_qualifier_result(raw_expr_idx)
    }
}
