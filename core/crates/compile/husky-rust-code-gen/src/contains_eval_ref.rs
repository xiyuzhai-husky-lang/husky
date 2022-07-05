use entity_kind::{EntityKind, FieldKind};
use husky_entity_route::entity_route_menu;
use husky_entity_route::{EntityRoute, EntityRouteKind, SpatialArgument};
use infer_decl::TyMemberDecl;
use word::RootIdentifier;

use super::*;

pub(super) fn entity_route_kind_contains_eval_ref(
    db: &dyn RustCodeGenQueryGroup,
    entity_route_kind: EntityRouteKind,
) -> bool {
    let base_route = db.intern_entity_route(EntityRoute {
        kind: entity_route_kind,
        temporal_arguments: Default::default(),
        spatial_arguments: Default::default(),
    });
    if entity_route_kind
        == (EntityRouteKind::Root {
            ident: RootIdentifier::Ref,
        })
    {
        return true;
    }
    let entity_route_menu = entity_route_menu();
    if entity_route_kind == entity_route_menu.std_slice_cyclic_slice.kind {
        return true;
    }
    let entity_kind = db.entity_kind(base_route).unwrap();
    match entity_kind {
        EntityKind::Module => return false,
        EntityKind::Type(_) => {
            let ty_decl = db.ty_decl(base_route).unwrap();
            for ty_member in ty_decl.ty_members.iter() {
                match ty_member {
                    TyMemberDecl::Field(field_decl) => match field_decl.field_kind {
                        FieldKind::StructOriginal
                        | FieldKind::StructDefault
                        | FieldKind::StructDerivedEager => {
                            if db.entity_route_contains_eval_ref(field_decl.ty) {
                                return true;
                            }
                        }
                        FieldKind::StructDerivedLazy => (),
                        FieldKind::RecordOriginal => panic!(),
                        FieldKind::RecordDerived => panic!(),
                    },
                    TyMemberDecl::Method(_) => (),
                    TyMemberDecl::Call(_) => (),
                }
            }
        }
        EntityKind::Trait => todo!(),
        EntityKind::Member(_) => {
            let method_decl = db.method_decl(base_route).unwrap();
            if db.entity_route_contains_eval_ref(base_route.parent()) {
                return true;
            }
            for parameter in method_decl.parameters.iter() {
                if db.entity_route_contains_eval_ref(parameter.ty) {
                    return true;
                }
            }
            if db.entity_route_contains_eval_ref(method_decl.output.ty) {
                return true;
            }
        }
        EntityKind::Function { requires_lazy } => {
            let function_decl = db.function_decl(base_route).unwrap();
            for parameter in function_decl.primary_parameters.iter() {
                if db.entity_route_contains_eval_ref(parameter.ty) {
                    return true;
                }
            }
            for parameter in function_decl.keyword_parameters.iter() {
                match parameter.liason {
                    ParameterLiason::EvalRef => return true,
                    ParameterLiason::TempRef => todo!(),
                    ParameterLiason::TempRefMut => todo!(),
                    _ => (),
                }
                if db.entity_route_contains_eval_ref(parameter.ty) {
                    return true;
                }
            }
            if db.entity_route_contains_eval_ref(function_decl.output.ty) {
                return true;
            }
        }
        EntityKind::Feature => todo!(),
        EntityKind::EnumLiteral => return false,
        EntityKind::Main => todo!(),
    }
    false
}

pub(super) fn entity_route_contains_eval_ref(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> bool {
    if db.entity_route_kind_contains_eval_ref(entity_route.kind) {
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
