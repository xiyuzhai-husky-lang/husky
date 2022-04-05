use super::*;

pub(crate) fn record_decl(
    generic_placeholders: IdentMap<GenericPlaceholderKind>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut memb_vars = VecMap::default();
    let mut memb_features = VecMap::default();
    let mut traits = Vec::new();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVarDefn {
                ident,
                signature: MembAccessDecl { contract, ty },
            } => memb_vars.insert_new(ident, MembAccessDecl { contract, ty }),
            AstKind::MembFeatureDefnHead { ident, ty } => memb_features.insert_new(ident, ty),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TyDecl {
        generic_placeholders,
        members: Default::default(),
        kind: TyDeclKind::Record {
            memb_vars,
            memb_features,
        },
        traits,
    }))
}
