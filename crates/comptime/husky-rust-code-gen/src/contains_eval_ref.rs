use husky_entity_kind::{EntityKind, FieldKind};
use husky_entity_route::{EntityRoute, EntityRouteVariant, SpatialArgument};
use husky_word::RootIdentifier;
use infer_decl::TyMemberDecl;

use super::*;

pub(super) fn entity_route_variant_contains_eval_ref(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> bool {
    let base_route = db.intern_entity_route(EntityRoute {
        variant: entity_route.variant.clone(),
        temporal_arguments: Default::default(),
        spatial_arguments: Default::default(),
    });
    if entity_route.variant
        == (EntityRouteVariant::Root {
            ident: RootIdentifier::Ref,
        })
    {
        return true;
    }
    if entity_route.variant
        == (EntityRouteVariant::Root {
            ident: RootIdentifier::Option,
        })
    {
        return false;
    }
    let entity_route_menu = db.entity_route_menu();
    if entity_route.variant == entity_route_menu.std_slice_cyclic_slice.variant {
        return true;
    }
    let husky_entity_kind = db.husky_entity_kind(base_route).unwrap();
    match husky_entity_kind {
        EntityKind::Module => return false,
        EntityKind::Type(_) => {
            let ty_decl = db.ty_decl(base_route).unwrap();
            for ty_member in ty_decl.ty_members.iter() {
                match ty_member {
                    TyMemberDecl::Field(field_decl) => match field_decl.field_kind {
                        FieldKind::StructRegular
                        | FieldKind::StructDefault
                        | FieldKind::StructDerived => {
                            if db.entity_route_contains_eval_ref(field_decl.ty) {
                                return true;
                            }
                        }
                        FieldKind::StructProperty => (),
                        FieldKind::RecordRegular => panic!(),
                        FieldKind::RecordProperty => panic!(),
                    },
                    TyMemberDecl::Method(_) => (),
                    TyMemberDecl::Call(_) => (),
                }
            }
        }
        EntityKind::Trait => todo!(),
        EntityKind::Member(_) => {
            let call_form_decl = db.entity_call_form_decl(base_route).unwrap();
            if db.entity_route_contains_eval_ref(base_route.parent()) {
                return true;
            }
            for parameter in call_form_decl.primary_parameters.iter() {
                if db.entity_route_contains_eval_ref(parameter.ty()) {
                    return true;
                }
            }
            if db.entity_route_contains_eval_ref(call_form_decl.output.ty()) {
                return true;
            }
        }
        EntityKind::Function { requires_lazy } => {
            let call_form_decl = db.entity_call_form_decl(base_route).unwrap();
            for parameter in call_form_decl.primary_parameters.iter() {
                if parameter_contains_eval_ref(db, parameter) {
                    return true;
                }
            }
            for parameter in call_form_decl.keyword_parameters.iter() {
                if parameter_contains_eval_ref(db, parameter) {
                    return true;
                }
            }
            if db.entity_route_contains_eval_ref(call_form_decl.output.ty()) {
                return true;
            }
        }
        EntityKind::Feature => return false,
        EntityKind::EnumVariant => return false,
        EntityKind::Main => todo!(),
    }
    false
}

fn parameter_contains_eval_ref(
    db: &dyn RustCodeGenQueryGroup,
    parameter: &infer_decl::ParameterDecl,
) -> bool {
    match parameter.liason {
        ParameterModifier::EvalRef => return true,
        _ => (),
    }
    db.entity_route_contains_eval_ref(parameter.ty())
}

pub(super) fn entity_route_contains_eval_ref(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> bool {
    if db.entity_route_variant_contains_eval_ref(entity_route) {
        return true;
    }
    for argument in entity_route.spatial_arguments.iter() {
        match argument {
            SpatialArgument::Const(_) => (),
            SpatialArgument::EntityRoute(entity_route) => {
                if db.entity_route_contains_eval_ref(*entity_route) {
                    return true;
                }
            }
        }
    }
    false
}
