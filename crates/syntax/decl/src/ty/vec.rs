use vm::InputContract;

use super::*;

pub(crate) fn vec_decl(db: &dyn DeclQueryGroup) -> Arc<TyDecl> {
    let element_ty_ident = db.custom_ident("T");
    let element_ty = db.intern_scope(EntityRoute {
        kind: EntityRouteKind::Generic {
            ident: element_ty_ident,
            raw_entity_kind: RawEntityKind::Type(RawTyKind::Vec),
        },
        generics: vec![],
    });
    let mut members = IdentMap::default();
    members.insert_new(
        db.custom_ident("push"),
        MembDecl {
            variant: MembDeclVariant::Routine(MembCallDecl {
                this_contract: InputContract::BorrowMut,
                inputs: vec![InputDecl {
                    contract: InputContract::Move,
                    ty: element_ty,
                }],
                output: db.entity_route_menu().void_type,
                generic_placeholders: Default::default(),
            }),
        },
    );
    members.insert_new(
        db.custom_ident("len"),
        MembDecl {
            variant: MembDeclVariant::Routine(MembCallDecl {
                this_contract: InputContract::Pure,
                inputs: vec![],
                output: db.entity_route_menu().i32_type,
                generic_placeholders: Default::default(),
            }),
        },
    );
    let mut generic_placeholders = IdentMap::default();
    generic_placeholders.insert_new(
        element_ty_ident,
        GenericPlaceholder::Type { traits: vec![] },
    );
    Arc::new(TyDecl {
        generic_placeholders,
        members,
        kind: TyDeclKind::Vec { element_ty },
        traits: vec![db.entity_route_menu().clone_trait],
    })
}
