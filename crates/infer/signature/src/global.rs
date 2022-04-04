use file::FilePtr;
use syntax_types::{ListOpr, Opr};
use word::BuiltinIdentifier;

use crate::*;

fn global_input_ty_from_ast(
    db: &dyn InferSignatureQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> InferResult<ScopePtr> {
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
                    kind: RawEntityKind::Routine,
                    ..
                } => {
                    let signature = db.call_signature(scope)?;
                    let dataset_type = signature.output;
                    match dataset_type.kind {
                        ScopeKind::Builtin {
                            ident: BuiltinIdentifier::DatasetType,
                        } => match dataset_type.generics[0] {
                            GenericArgument::Const(_) => todo!(),
                            GenericArgument::Scope(input_ty) => Ok(input_ty),
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

fn global_output_ty_from_ast(
    db: &dyn InferSignatureQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> InferResult<ScopePtr> {
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
                    kind: RawEntityKind::Routine,
                    ..
                } => {
                    let signature = db.call_signature(scope)?;
                    let dataset_type = signature.output;
                    match dataset_type.kind {
                        ScopeKind::Builtin {
                            ident: BuiltinIdentifier::DatasetType,
                        } => match dataset_type.generics[1] {
                            GenericArgument::Const(_) => todo!(),
                            GenericArgument::Scope(output_ty) => Ok(output_ty),
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

pub(crate) fn global_input_ty(
    db: &dyn InferSignatureQueryGroup,
    main_file: FilePtr,
) -> InferResult<ScopePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfig => {
                return global_input_ty_from_ast(
                    db,
                    &ast_text.arena,
                    derived_not_none!(item.children)?
                        .last()
                        .unwrap()
                        .value
                        .as_ref()?,
                )
            }
            _ => (),
        }
    }
    err!("dataset config not found, so input type can't be inferred")
}

pub(crate) fn global_output_ty(
    db: &dyn InferSignatureQueryGroup,
    main_file: FilePtr,
) -> InferResult<ScopePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfig => {
                return global_output_ty_from_ast(
                    db,
                    &ast_text.arena,
                    derived_not_none!(item.children)?
                        .last()
                        .unwrap()
                        .value
                        .as_ref()?,
                )
            }
            _ => (),
        }
    }
    err!("dataset config not found, so input type can't be inferred")
}
