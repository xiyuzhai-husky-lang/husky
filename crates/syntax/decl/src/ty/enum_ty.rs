use super::*;

pub(crate) fn enum_decl(
    generic_placeholders: IdentMap<GenericPlaceholderKind>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut variants = VecMap::default();
    for subitem in children {
        match subitem.value.as_ref()?.kind {
            AstKind::EnumVariantDefnHead {
                ident,
                ref raw_variant_kind,
            } => {
                let variant_sig = match raw_variant_kind {
                    RawEnumVariantKind::Constant => EnumVariantDecl::Constant,
                };
                variants.insert_new(ident, variant_sig)
            }
            _ => panic!(),
        }
    }
    Ok(Arc::new(TyDecl {
        generic_placeholders,
        members: IdentMap::default(),
        kind: TyDeclKind::Enum { variants },
        traits: todo!(),
    }))
}
