use super::*;

pub(super) fn item_route_variant_contains_eval_ref(
    _db: &dyn RustTranspileDb,
    _item_path: EtherealTerm,
) -> bool {
    // let base_route = db.intern_item_route(EntityRoute {
    //     variant: item_path.variant.clone(),
    //     temporal_arguments: Default::default(),
    //     spatial_arguments: Default::default(),
    // });
    // if item_path.variant
    //     == (EntityRouteVariant::Root {
    //         ident: RootBuiltinIdent::Ref,
    //     })
    // {
    //     return true;
    // }
    // if item_path.variant
    //     == (EntityRouteVariant::Root {
    //         ident: RootBuiltinIdent::Option,
    //     })
    // {
    //     return false;
    // }
    // let item_route_menu = db.item_route_menu();
    // if item_path.variant == item_route_menu.std_slice_cyclic_slice.variant {
    //     return true;
    // }
    // let _husky_item_taxonomy = db.husky_entity_taxonomy(base_route).unwrap();
    todo!()
    // match husky_entity_taxonomy {
    //     EntityKind::Module => return false,
    //     EntityKind::Type(_) => {
    //         let ty_decl = db.ty_decl(base_route).unwrap();
    //         for ty_member in ty_decl.ty_members.iter() {
    //             match ty_member {
    //                 TyMemberDecl::Field(field_decl) => match field_decl.field_kind {
    //                     FieldKind::StructRegular
    //                     | FieldKind::StructDefault
    //                     | FieldKind::StructDerived => {
    //                         if db.item_route_contains_eval_ref(field_decl.ty) {
    //                             return true;
    //                         }
    //                     }
    //                     FieldKind::StructMemo => (),
    //                     FieldKind::RecordRegular => panic!(),
    //                     FieldKind::RecordProperty => panic!(),
    //                 },
    //                 TyMemberDecl::Method(_) => (),
    //                 TyMemberDecl::Call(_) => (),
    //             }
    //         }
    //     }
    //     EntityKind::Trait => todo!(),
    //     EntityKind::Member(_) => {
    //         let call_fugitive_syn_decl = db.item_call_fugitive_syn_decl(base_route).unwrap();
    //         if db.item_route_contains_eval_ref(base_route.parent()) {
    //             return true;
    //         }
    //         for parameter in call_fugitive_syn_decl.primary_parameters.iter() {
    //             if db.item_route_contains_eval_ref(parameter.ty()) {
    //                 return true;
    //             }
    //         }
    //         if db.item_route_contains_eval_ref(call_fugitive_syn_decl.output.ty()) {
    //             return true;
    //         }
    //     }
    //     EntityKind::Function { .. } => {
    //         let call_fugitive_syn_decl = db.item_call_fugitive_syn_decl(base_route).unwrap();
    //         for parameter in call_fugitive_syn_decl.primary_parameters.iter() {
    //             if parameter_contains_eval_ref(db, parameter) {
    //                 return true;
    //             }
    //         }
    //         for parameter in call_fugitive_syn_decl.keyword_parameters.iter() {
    //             if parameter_contains_eval_ref(db, parameter) {
    //                 return true;
    //             }
    //         }
    //         if db.item_route_contains_eval_ref(call_fugitive_syn_decl.output.ty()) {
    //             return true;
    //         }
    //     }
    //     EntityKind::Feature => return false,
    //     EntityKind::EnumVariant => return false,
    //     EntityKind::Main => todo!(),
    // }
    // false
}

// fn parameter_contains_eval_ref(
//     db: &dyn RustTranspileDb,
//     parameter: &infer_decl::ParameterDecl,
// ) -> bool {
//     match parameter.modifier {
//         ParameterModifier::Leash => return true,
//         _ => (),
//     }
//     db.item_route_contains_eval_ref(parameter.ty())
// }

pub(super) fn item_route_contains_eval_ref(
    _db: &dyn RustTranspileDb,
    _item_path: EtherealTerm,
) -> bool {
    todo!()
    // if db.item_route_variant_contains_eval_ref(item_path) {
    //     return true;
    // }
    // for argument in item_path.spatial_arguments.iter() {
    //     match argument {
    //         SpatialArgument::Const(_) => (),
    //         SpatialArgument::EntityRoute(item_path) => {
    //             if db.item_route_contains_eval_ref(*item_path) {
    //                 return true;
    //             }
    //         }
    //     }
    // }
    // false
}
