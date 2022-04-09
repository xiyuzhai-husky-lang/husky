use vm::InputContract;

use super::*;

static STATIC_VEC_DECL: &'static StaticTyDecl = &StaticTyDecl {
    base_ty: "Vec",
    generic_placeholders: &[StaticGenericPlaceholder {
        name: "E",
        variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
    }],
    traits: &["Clone"],
    fields: &[],
    methods: &[
        StaticMethodDecl {
            name: "len",
            this_contract: InputContract::Pure,
            inputs: &[],
            output_ty: "i32",
            generic_placeholders: &[],
        },
        StaticMethodDecl {
            name: "push",
            this_contract: InputContract::BorrowMut,
            inputs: &[StaticInputDecl {
                contract: InputContract::Move,
                ty: "E",
            }],
            output_ty: "void",
            generic_placeholders: &[],
        },
        StaticMethodDecl {
            name: "pop",
            this_contract: InputContract::BorrowMut,
            inputs: &[],
            output_ty: "E",
            generic_placeholders: &[],
        },
    ],
    variants: &[],
    kind: TyKind::Vec,
};

pub(crate) fn vec_decl(db: &dyn DeclQueryGroup) -> Arc<TyDecl> {
    Arc::new(TyDecl::from_static(db, STATIC_VEC_DECL))
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
