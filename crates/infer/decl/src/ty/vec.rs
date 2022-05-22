use vm::InputLiason;

use super::*;

pub(crate) fn vec_decl(db: &dyn DeclQueryGroup) -> Arc<TyDecl> {
    TyDecl::from_static(db, &VEC_TYPE_DEFN)
    // let element_ty_ident = db.custom_ident("T");
    // let element_ty = db.intern_scope(EntityRoute {
    //     kind: EntityRouteKind::Generic {
    //         ident: element_ty_ident,
    //         entity_kind: EntityKind::Type(RawTyKind::Vec),
    //     },
    //     generics: vec![],
    // });
    // let mut generic_placeholders = IdentDict::default();
    // generic_placeholders.insert_new(
    //     element_ty_ident,
    //     GenericPlaceholder::Type { traits: vec![] },
    // );
    // Arc::new(TyDecl::new(
    //     db,
    //     EntityRouteKind::Root {
    //         ident: RootIdentifier::Vec,
    //     },
    //     generic_placeholders,
    //     vec![db.entity_route_menu().clone_trait],
    //     TyKind::Vec { element_ty },
    // ))
}

// pub(crate) fn add_vec_methods(
//     db: &dyn DeclQueryGroup,
//     element_ty: EntityRoutePtr,
//     methods: &mut IdentDict<MembDecl>,
// ) {
//     methods.insert_new(
//         db.custom_ident("push"),
//         MembDecl {
//             variant: FieldDeclVariant::Routine(MethodDecl {
//                 this_contract: InputContract::BorrowMut,
//                 inputs: vec![InputDecl {
//                     contract: InputContract::Move,
//                     ty: element_ty,
//                 }],
//                 output: db.entity_route_menu().void_type,
//                 generic_placeholders: Default::default(),
//             }),
//         },
//     );
//     methods.insert_new(
//         db.custom_ident("len"),
//         MembDecl {
//             variant: FieldDeclVariant::Routine(MethodDecl {
//                 this_contract: InputContract::Pure,
//                 inputs: vec![],
//                 output: db.entity_route_menu().i32_type,
//                 generic_placeholders: Default::default(),
//             }),
//         },
//     );
// }
