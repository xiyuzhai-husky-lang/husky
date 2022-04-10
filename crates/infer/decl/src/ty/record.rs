use super::*;

pub(crate) fn record_decl(
    db: &dyn DeclQueryGroup,
    entity_route_kind: EntityRouteKind,
    generic_placeholders: IdentDict<GenericPlaceholder>,
    mut children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut type_members = IdentDict::default();
    let mut trait_impls = vec![];
    // add fields
    while let Some(subitem) = children.next() {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_var_defn) => {
                type_members.insert_new(TypeMemberDecl::Field(Arc::new(FieldDecl {
                    ident: field_var_defn.ident,
                    contract: field_var_defn.contract,
                    ty: field_var_defn.ty,
                })))
            }
            _ => break,
        }
    }
    // add other members
    while let Some(subitem) = children.next() {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_var_defn) => panic!("expect fields to be defined first"),
            AstKind::MembFeatureDefnHead { ident, ty } => type_members.insert_new(todo!()),
            _ => panic!(),
        }
    }
    let generics = db.generic_arguments_from_generic_placeholders(&generic_placeholders);
    let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
    let this_ty = db.intern_scope(EntityRoute {
        kind: entity_route_kind,
        generic_arguments: generics,
    });
    Ok(Arc::new(TyDecl::new(
        db,
        this_ty,
        generic_placeholders,
        type_members,
        Default::default(), // variants
        TyKind::Record,
        trait_impls,
    )))
    // db,
    // entity_route_kind,
    // generic_placeholders,
    // traits,
    // TyKind::Record,
}
