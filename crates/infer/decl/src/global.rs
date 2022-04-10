use file::FilePtr;
use syntax_types::{ListOpr, Opr};
use text::TextRange;
use word::RootIdentifier;

use crate::*;

fn global_input_ty_from_ast(
    db: &dyn DeclQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> InferResult<EntityRoutePtr> {
    match ast.kind {
        AstKind::Stmt(RawStmt {
            kind: RawStmtKind::Return(idx),
            ..
        }) => match arena[idx].kind {
            RawExprVariant::Opn {
                opr: Opr::List(ListOpr::Call),
                ref opds,
            } => match arena[opds][0].kind {
                RawExprVariant::Scope {
                    scope,
                    kind: EntityKind::Routine,
                    ..
                } => {
                    let signature = db.call_decl(scope)?;
                    let dataset_type = signature.output;
                    match dataset_type.kind {
                        EntityRouteKind::Root {
                            ident: RootIdentifier::DatasetType,
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
    db: &dyn DeclQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> InferResult<EntityRoutePtr> {
    match ast.kind {
        AstKind::Stmt(RawStmt {
            kind: RawStmtKind::Return(idx),
            ..
        }) => match arena[idx].kind {
            RawExprVariant::Opn {
                opr: Opr::List(ListOpr::Call),
                ref opds,
            } => match arena[opds][0].kind {
                RawExprVariant::Scope {
                    scope,
                    kind: EntityKind::Routine,
                    ..
                } => {
                    let call_decl = db.call_decl(scope)?;
                    let dataset_type = call_decl.output;
                    match dataset_type.kind {
                        EntityRouteKind::Root {
                            ident: RootIdentifier::DatasetType,
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
    db: &dyn DeclQueryGroup,
    main_file: FilePtr,
) -> InferResult<EntityRoutePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfigDefnHead => {
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
    err!(
        "dataset config not found, so input type can't be inferred",
        todo!("whole main file range")
    )
}

pub(crate) fn global_output_ty(
    db: &dyn DeclQueryGroup,
    main_file: FilePtr,
) -> InferResult<EntityRoutePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfigDefnHead => {
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
    err!(
        "dataset config not found in main, so input type can't be inferred",
        TextRange {
            start: (0u32, 0).into(),
            end: (0u32, 4).into(),
        }
    )
}
