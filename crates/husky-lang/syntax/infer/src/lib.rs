mod call;
mod ty;

pub use call::*;
use file::FilePtr;
pub use ty::*;

use ast::{Ast, AstKind, AstResult, RawExprArena, RawExprIdx, RawExprKind, RawStmt, RawStmtKind};
use common::*;
use fold::FoldStorage;
use scope::{InputSignature, ScopeKind, ScopePtr, ScopeRoute, ScopeSource, StaticFuncSignature};
use scope_query::ScopeQueryGroup;
use std::sync::Arc;
use syntax_error::*;
use syntax_types::{ListOpr, Opr, RawTyKind};
use vm::{Compiled, EnumLiteralValue};
use word::{BuiltinIdentifier, CustomIdentifier, ImplicitIdentifier};

pub trait InferQueryGroup: InferSalsaQueryGroup {
    fn expr_ty(&self, file: FilePtr, expr_idx: RawExprIdx) -> SyntaxResult<ScopePtr> {
        self.ty_sheet(file)?.ty(expr_idx)
    }

    fn expr_ty_signature(
        &self,
        file: FilePtr,
        expr_idx: RawExprIdx,
    ) -> SyntaxResultArc<TySignature> {
        self.ty_signature(self.expr_ty(file, expr_idx)?)
    }
}

#[salsa::query_group(InferSalsaQueryGroupStorage)]
pub trait InferSalsaQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn call_signature(&self, scope: ScopePtr) -> SyntaxResultArc<CallSignature>;
    fn ty_signature(&self, scope: ScopePtr) -> SyntaxResultArc<TySignature>;
    fn scope_ty(&self, scope: ScopePtr) -> SyntaxResult<ScopePtr>;
    fn input_ty(&self, main_file: FilePtr) -> SyntaxResult<ScopePtr>;
    fn enum_literal_value(&self, scope: ScopePtr) -> EnumLiteralValue;
    fn ty_sheet(&self, file: FilePtr) -> SyntaxResultArc<TySheet>;
}

fn scope_ty(db: &dyn InferSalsaQueryGroup, scope: ScopePtr) -> SyntaxResult<ScopePtr> {
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

fn input_ty(db: &dyn InferSalsaQueryGroup, main_file: FilePtr) -> SyntaxResult<ScopePtr> {
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
    db: &dyn InferSalsaQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> SyntaxResult<ScopePtr> {
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
                    let signature = db.call_signature(scope)?;
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

fn enum_literal_value(db: &dyn InferSalsaQueryGroup, scope: ScopePtr) -> EnumLiteralValue {
    msg_once!("todo: enum_literal_value");
    EnumLiteralValue::interpreted(scope)
}
