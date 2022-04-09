use super::*;

pub(crate) fn record_decl(
    db: &dyn DeclQueryGroup,
    entity_route_kind: EntityRouteKind,
    generic_placeholders: IdentDict<GenericPlaceholder>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut fields = VecDict::default();
    let mut traits = Vec::new();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_var_defn) => fields.insert_new(FieldDecl {
                ident: field_var_defn.ident,
                contract: field_var_defn.contract,
                ty: field_var_defn.ty,
            }),
            AstKind::MembFeatureDefnHead { ident, ty } => fields.insert_new(todo!()),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TyDecl {
        this_type: todo!(),
        generic_placeholders,
        traits,
        fields,
        methods: todo!(),
        variants: todo!(),
        kind: todo!(),
    }))
    // db,
    // entity_route_kind,
    // generic_placeholders,
    // traits,
    // TyKind::Record,
}
