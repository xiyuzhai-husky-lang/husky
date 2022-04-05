use super::*;

pub(crate) fn enum_decl(
    generic_placeholders: IdentMap<GenericPlaceholderKind>,
    children: AstIter,
) -> InferResultArc<TySignature> {
    let mut variants = VecMap::default();
    for subitem in children {
        match subitem.value.as_ref()?.kind {
            AstKind::EnumVariantDefnHead {
                ident,
                ref raw_variant_kind,
            } => {
                let variant_sig = match raw_variant_kind {
                    RawEnumVariantKind::Constant => EnumVariantSignature::Constant,
                };
                variants.insert_new(ident, variant_sig)
            }
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature {
        generic_placeholders,
        members: IdentMap::default(),
        kind: TySignatureKind::Enum { variants },
        traits: todo!(),
    }))
}
