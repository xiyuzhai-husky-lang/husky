use file::FilePtr;
use text::{BindTextRange, TextRange};
use vm::*;
use word::RootIdentifier;

use crate::*;

fn global_input_ty_from_ast(
    db: &dyn DeclQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> InferResult<EntityRoutePtr> {
    match ast.kind {
        AstKind::Stmt(RawStmt {
            variant: RawStmtVariant::Return(idx),
            ..
        }) => match arena[idx].variant {
            RawExprVariant::Opn {
                opr: Opr::List(ListOpr::Call),
                ref opds,
            } => {
                let caller = &arena[opds][0];
                match caller.variant {
                    RawExprVariant::Entity {
                        route,
                        kind: EntityKind::Routine,
                        ..
                    } => {
                        let signature = db.call_decl(route).bind(caller)?;
                        let dataset_type = signature.output.ty;
                        match dataset_type.kind {
                            EntityRouteKind::Root {
                                ident: RootIdentifier::DatasetType,
                            } => match dataset_type.generic_arguments[0] {
                                GenericArgument::Const(_) => todo!(),
                                GenericArgument::EntityRoute(input_ty) => Ok(input_ty),
                            },
                            _ => panic!(),
                        }
                    }
                    _ => todo!(),
                }
            }
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
            variant: RawStmtVariant::Return(idx),
            ..
        }) => match arena[idx].variant {
            RawExprVariant::Opn {
                opr: Opr::List(ListOpr::Call),
                ref opds,
            } => {
                let caller = &arena[opds][0];
                match caller.variant {
                    RawExprVariant::Entity {
                        route: scope,
                        kind: EntityKind::Routine,
                        ..
                    } => {
                        let call_decl = db.call_decl(scope).bind_text_range(caller.range)?;
                        let dataset_type = call_decl.output.ty;
                        match dataset_type.kind {
                            EntityRouteKind::Root {
                                ident: RootIdentifier::DatasetType,
                            } => match dataset_type.generic_arguments[1] {
                                GenericArgument::Const(_) => todo!(),
                                GenericArgument::EntityRoute(output_ty) => Ok(output_ty),
                            },
                            _ => panic!(),
                        }
                    }
                    _ => todo!(),
                }
            }
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
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfigDefnHead => {
                return global_input_ty_from_ast(
                    db,
                    &ast_text.arena,
                    derived_not_none!(item.opt_children)?
                        .last()
                        .unwrap()
                        .value
                        .as_ref()?,
                )
            }
            _ => (),
        }
    }
    throw!(
        "dataset config not found in main, so input type can't be inferred",
        TextRange::whole()
    )
}

pub(crate) fn global_output_ty(
    db: &dyn DeclQueryGroup,
    main_file: FilePtr,
) -> InferResult<EntityRoutePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfigDefnHead => {
                return global_output_ty_from_ast(
                    db,
                    &ast_text.arena,
                    derived_not_none!(item.opt_children)?
                        .last()
                        .unwrap()
                        .value
                        .as_ref()?,
                )
            }
            _ => (),
        }
    }
    throw_derived!("dataset config not found in main, so output type can't be inferred")
}
