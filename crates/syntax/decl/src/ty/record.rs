use super::*;

pub(crate) fn record_decl(
    generic_placeholders: IdentMap<GenericPlaceholder>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut memb_vars = VecDict::default();
    let mut memb_features = VecDict::default();
    let mut traits = Vec::new();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVarDefn(ref memb_var_defn) => memb_vars.insert_new(
                memb_var_defn.ident,
                MembAccessDecl {
                    contract: memb_var_defn.contract,
                    ty: memb_var_defn.ty,
                },
            ),
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
