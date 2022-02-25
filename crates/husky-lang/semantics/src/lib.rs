#![allow(dead_code, warnings)]
mod config;
mod error;
mod expr;
mod instruction_sheet;
mod kind;
mod package;
mod query;
mod stmt;

pub use config::Config;
pub use error::{SemanticError, SemanticResult, SemanticResultArc};
pub use expr::{BinaryOpnKind, Expr, ExprKind, Opn};
use file::FilePtr;
pub use instruction_sheet::InstructionSheet;
pub use kind::EntityKind;
pub use package::Package;
pub use query::*;
use scope::ScopePtr;
pub use stmt::{
    DeclBranchKind, DeclBranchesKind, DeclStmt, DeclStmtKind, StrictStmt, StrictStmtKind,
};

use kind::*;
use std::sync::Arc;
use unique_vector::UniqVec;
use vc::{Uid, VersionControl};

use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub ident: CustomIdentifier,
    pub kind: Arc<EntityKind>,
    pub subentities: Arc<Vec<Arc<Entity>>>,
    pub scope: ScopePtr,
    pub file: FilePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityInnerId {
    raw: usize,
}

pub type EntityVersionControl = VersionControl<ScopePtr, EntityKind>;
pub type EntityUid = Uid;

pub trait ControlEntityVersion {
    fn entity_vc(&self) -> &EntityVersionControl;
    fn entity_uid(&self, scope: ScopePtr) -> Uid {
        self.entity_vc().uid(scope)
    }
}

impl Entity {
    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }

    fn dependees(kind: &EntityKind) -> UniqVec<ScopePtr> {
        return match kind {
            EntityKind::Module(_) => Default::default(),
            EntityKind::Feature(_) => todo!(),
            EntityKind::Pattern(_) => todo!(),
            EntityKind::Func {
                input_contracts: inputs,
                stmts,
            } => {
                let mut v = UniqVec::new();
                for input in inputs.iter() {
                    v.push(input.1.ty)
                }
                extract_decl_stmts_dependees(stmts, &mut v);
                v
            }
            EntityKind::Proc(_) => todo!(),
            EntityKind::Ty(ty) => match ty.kind {
                ty::TyKind::Enum => todo!(),
                ty::TyKind::Struct { ref memb_vars } => {
                    memb_vars.iter().map(|memb_var| memb_var.ty.scope).into()
                }
            },
        };

        fn extract_decl_stmts_dependees(stmts: &[Arc<DeclStmt>], v: &mut UniqVec<ScopePtr>) {
            for stmt in stmts {
                match stmt.kind {
                    DeclStmtKind::Init { varname, ref value } => extract_expr_dependees(value, v),
                    DeclStmtKind::Assert { ref condition } => extract_expr_dependees(condition, v),
                    DeclStmtKind::Return { ref result } => extract_expr_dependees(result, v),
                    DeclStmtKind::Branches { kind, ref branches } => {
                        for branch in branches {
                            extract_decl_stmts_dependees(&branch.stmts, v)
                        }
                    }
                }
            }
        }

        fn extract_expr_dependees(expr: &Expr, v: &mut UniqVec<ScopePtr>) {
            match expr.kind {
                ExprKind::Variable(_) => (),
                ExprKind::Scope { scope, compiled } => v.push(scope),
                ExprKind::Literal(_) => (),
                ExprKind::Bracketed(ref expr) => extract_expr_dependees(expr, v),
                ExprKind::Opn {
                    ref opn,
                    compiled,
                    ref opds,
                } => match opn {
                    Opn::Binary { opr, this, kind } => v.push(*this),
                    Opn::Prefix(_) => todo!(),
                    Opn::Suffix(_) => todo!(),
                    Opn::FuncCall { func, .. } => v.push(*func),
                    Opn::PattCall => todo!(),
                    Opn::MembVarAccess => todo!(),
                    Opn::MembFuncCall(_) => todo!(),
                    Opn::ElementAccess => todo!(),
                },
                ExprKind::Lambda(_, _) => todo!(),
            }
        }
    }

    pub(crate) fn new(
        ident: CustomIdentifier,
        kind: EntityKind,
        subentities: Arc<Vec<Arc<Entity>>>,
        scope: ScopePtr,
        file: FilePtr,
        vc: &EntityVersionControl,
    ) -> Entity {
        use std::borrow::Borrow;
        let kind = Arc::new(kind);
        vc.update(scope, kind.clone(), &Self::dependees(&kind));
        Self {
            ident,
            kind,
            subentities,
            scope,
            file,
        }
    }
}
