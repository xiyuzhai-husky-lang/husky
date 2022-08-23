use husky_file::FilePtr;
use husky_text::{BindTextRangeInto, TextRange};
use husky_word::RootIdentifier;
use thin_vec::{thin_vec, ThinVec};
use vm::*;

use crate::*;

fn target_input_ty_from_ast(
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
                        let dataset_type = signature_result?.output.ty();
                        match dataset_type.variant {
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

fn target_output_ty_from_ast(
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
                        let dataset_type = call_decl_result?.output.ty();
                        match dataset_type.variant {
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

pub(crate) fn target_input_ty(db: &dyn DeclQueryGroup) -> InferResult<EntityRoutePtr> {
    let ast_text = db.ast_text(db.opt_target_entrance().unwrap())?;
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref()?.variant {
            AstVariant::DatasetConfigDefnHead => {
                return target_input_ty_from_ast(
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

pub(crate) fn target_output_ty(db: &dyn DeclQueryGroup) -> InferResult<EntityRoutePtr> {
    let ast_text = db.ast_text(db.opt_target_entrance().unwrap())?;
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref()?.variant {
            AstVariant::DatasetConfigDefnHead => {
                return target_output_ty_from_ast(
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

pub(crate) fn implement_target(
    db: &dyn DeclQueryGroup,
    ty: EntityRoutePtr,
) -> InferResult<EntityRoutePtr> {
    let mut spatial_arguments: ThinVec<_> = Default::default();
    let variant = match ty.variant {
        EntityRouteVariant::Root { ident } => EntityRouteVariant::Root { ident },
        EntityRouteVariant::Package { main, ident } => EntityRouteVariant::Package { main, ident },
        EntityRouteVariant::Child { parent, ident } => EntityRouteVariant::Child {
            parent: db.implement_target(parent)?,
            ident,
        },
        EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => todo!(),
        EntityRouteVariant::TargetInputValue => todo!(),
        EntityRouteVariant::TargetOutputType => {
            let target_output_ty = db.target_output_ty()?;
            spatial_arguments = target_output_ty.spatial_arguments.clone();
            target_output_ty.variant.clone()
        }
        EntityRouteVariant::Any {
            ident,
            husky_entity_kind,
            file,
            range,
        } => EntityRouteVariant::Any {
            ident,
            husky_entity_kind,
            file,
            range,
        },
        EntityRouteVariant::ThisType => EntityRouteVariant::ThisType,
    };
    for arg in ty.spatial_arguments.iter() {
        spatial_arguments.push(match arg {
            SpatialArgument::Const(value) => SpatialArgument::Const(*value),
            SpatialArgument::EntityRoute(route) => {
                SpatialArgument::EntityRoute(db.implement_target(*route)?)
            }
        })
    }
    Ok(db.intern_entity_route(EntityRoute {
        variant,
        temporal_arguments: Default::default(),
        spatial_arguments,
    }))
}
