use super::*;

pub(crate) fn record_decl(
    generic_placeholders: IdentMap<GenericPlaceholderKind>,
    children: AstIter,
) -> InferResultArc<TySignature> {
    let mut memb_vars = VecMap::default();
    let mut memb_features = VecMap::default();
    let mut traits = Vec::new();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVarDefn {
                ident,
                signature: MembAccessSignature { contract, ty },
            } => memb_vars.insert_new(ident, MembAccessSignature { contract, ty }),
            AstKind::MembFeatureDefnHead { ident, ty } => memb_features.insert_new(ident, ty),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature {
        generic_placeholders,
        members: Default::default(),
        kind: TySignatureKind::Record {
            memb_vars,
            memb_features,
        },
        traits,
    }))
}
