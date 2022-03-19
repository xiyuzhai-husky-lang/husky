mod call;
mod ty;

pub use call::*;
pub use ty::*;

use ast::{Ast, AstKind, AstResult, RawExprArena, RawExprKind, RawStmt, RawStmtKind};
use common::*;
use fold::FoldStorage;
use scope::{InputSignature, ScopeKind, ScopePtr, ScopeRoute, ScopeSource, StaticFuncSignature};
use scope_query::ScopeQueryGroup;
use semantics_error::*;
use std::sync::Arc;
use syntax_types::{ListOpr, Opr, RawTyKind};
use vm::{Compiled, EnumLiteralValue};
use word::{BuiltinIdentifier, CustomIdentifier, ImplicitIdentifier};

#[salsa::query_group(InferQueryGroupStorage)]
pub trait InferQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn call_signature(&self, scope: ScopePtr) -> SemanticResultArc<CallSignature>;
    fn ty_signature(&self, scope: ScopePtr) -> SemanticResultArc<TySignature>;
    fn scope_ty(&self, scope: ScopePtr) -> SemanticResult<ScopePtr>;
    fn input_ty(&self, main_file: file::FilePtr) -> SemanticResult<ScopePtr>;
    fn enum_literal_value(&self, scope: ScopePtr) -> EnumLiteralValue;
}

fn scope_ty(db: &dyn InferQueryGroup, scope: ScopePtr) -> SemanticResult<ScopePtr> {
    match scope {
        ScopePtr::Builtin(ident) => match ident {
            BuiltinIdentifier::Void => todo!(),
            BuiltinIdentifier::I32 => todo!(),
            BuiltinIdentifier::F32 => todo!(),
            BuiltinIdentifier::B32 => todo!(),
            BuiltinIdentifier::B64 => todo!(),
            BuiltinIdentifier::Bool => todo!(),
            BuiltinIdentifier::True | BuiltinIdentifier::False => {
                Ok(ScopePtr::Builtin(BuiltinIdentifier::Bool))
            }
            BuiltinIdentifier::Vector => todo!(),
            BuiltinIdentifier::Tuple => todo!(),
            BuiltinIdentifier::Debug => todo!(),
            BuiltinIdentifier::Std => todo!(),
            BuiltinIdentifier::Core => todo!(),
            BuiltinIdentifier::Fp => todo!(),
            BuiltinIdentifier::Fn => todo!(),
            BuiltinIdentifier::FnMut => todo!(),
            BuiltinIdentifier::FnOnce => todo!(),
            BuiltinIdentifier::Array => todo!(),
            BuiltinIdentifier::DatasetType => todo!(),
        },
        ScopePtr::Custom(scope) => match scope.route {
            ScopeRoute::Implicit { main, ident } => match ident {
                ImplicitIdentifier::Input => input_ty(db, main),
            },
            _ => todo!(),
        },
    }
}

fn input_ty(db: &dyn InferQueryGroup, main_file: file::FilePtr) -> SemanticResult<ScopePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfig => {
                return input_ty_from_ast(
                    db,
                    &ast_text.arena,
                    not_none!(item.children).last().unwrap().value.as_ref()?,
                )
            }
            _ => (),
        }
    }
    err!("dataset config not found, so input type can't be inferred")
}

fn input_ty_from_ast(
    this: &dyn InferQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> SemanticResult<ScopePtr> {
    match ast.kind {
        AstKind::Stmt(RawStmt {
            kind: RawStmtKind::Return(idx),
            ..
        }) => match arena[idx].kind {
            RawExprKind::Opn {
                opr: Opr::List(ListOpr::Call),
                ref opds,
            } => match arena[opds][0].kind {
                RawExprKind::Scope {
                    scope,
                    kind: ScopeKind::Routine,
                    ..
                } => {
                    let signature = this.call_signature(scope)?;
                    let dataset_type = signature.output;
                    match dataset_type.route {
                        ScopeRoute::Builtin {
                            ident: BuiltinIdentifier::DatasetType,
                        } => match dataset_type.generics[0] {
                            scope::GenericArgument::Const(_) => todo!(),
                            scope::GenericArgument::Scope(input_ty) => Ok(input_ty),
                        },
                        _ => panic!(),
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn enum_literal_value(db: &dyn InferQueryGroup, scope: ScopePtr) -> EnumLiteralValue {
    msg_once!("todo: enum_literal_value");
    EnumLiteralValue::interpreted(scope)
}
