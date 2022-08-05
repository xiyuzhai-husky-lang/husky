use husky_file::FilePtr;
use husky_text::{BindTextRangeInto, TextRange};
use husky_word::RootIdentifier;
use thin_vec::thin_vec;
use vm::*;

use crate::*;

fn eval_input_ty_from_ast(
    db: &dyn DeclQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> InferResult<EntityRoutePtr> {
    match ast.variant {
        AstVariant::Stmt(RawStmt {
            variant: RawStmtVariant::Return { result, .. },
            ..
        }) => match arena[result].variant {
            RawExprVariant::Opn {
                opn_variant: RawOpnVariant::List(ListOpr::FunctionCall),
                ref opds,
            } => {
                let caller = &arena[opds][0];
                match caller.variant {
                    RawExprVariant::Entity {
                        route,
                        kind: EntityKind::Function { .. },
                        ..
                    } => {
                        let signature_result: InferResult<_> =
                            db.entity_call_form_decl(route).bind_into(caller);
                        let dataset_type = signature_result?.output.ty;
                        match dataset_type.kind {
                            EntityRouteVariant::Root {
                                ident: RootIdentifier::DatasetType,
                            } => match dataset_type.spatial_arguments[0] {
                                SpatialArgument::Const(_) => todo!(),
                                SpatialArgument::EntityRoute(input_ty) => Ok(input_ty),
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

fn eval_output_ty_from_ast(
    db: &dyn DeclQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> InferResult<EntityRoutePtr> {
    match ast.variant {
        AstVariant::Stmt(RawStmt {
            variant: RawStmtVariant::Return { result, .. },
            ..
        }) => match arena[result].variant {
            RawExprVariant::Opn {
                opn_variant: RawOpnVariant::List(ListOpr::FunctionCall),
                ref opds,
            } => {
                let caller = &arena[opds][0];
                match caller.variant {
                    RawExprVariant::Entity {
                        route,
                        kind: EntityKind::Function { .. },
                        ..
                    } => {
                        let call_decl_result: InferResult<_> =
                            db.entity_call_form_decl(route).bind_into(caller);
                        let dataset_type = call_decl_result?.output.ty;
                        match dataset_type.kind {
                            EntityRouteVariant::Root {
                                ident: RootIdentifier::DatasetType,
                            } => match dataset_type.spatial_arguments[1] {
                                SpatialArgument::Const(_) => todo!(),
                                SpatialArgument::EntityRoute(label_ty) => {
                                    Ok(db.option(label_ty.into()))
                                }
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

pub(crate) fn eval_input_ty(
    db: &dyn DeclQueryGroup,
    main_file: FilePtr,
) -> InferResult<EntityRoutePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref()?.variant {
            AstVariant::DatasetConfigDefnHead => {
                return eval_input_ty_from_ast(
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

pub(crate) fn eval_output_ty(
    db: &dyn DeclQueryGroup,
    main_file: FilePtr,
) -> InferResult<EntityRoutePtr> {
    let ast_text = db.ast_text(main_file)?;
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref()?.variant {
            AstVariant::DatasetConfigDefnHead => {
                return eval_output_ty_from_ast(
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
