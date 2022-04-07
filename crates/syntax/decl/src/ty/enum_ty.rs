use super::*;

pub(crate) fn enum_decl(
    generic_placeholders: IdentMap<GenericPlaceholder>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut variants = VecDict::default();
    for subitem in children {
        match subitem.value.as_ref()?.kind {
            AstKind::EnumVariantDefnHead {
                ident,
                variant_class: ref raw_variant_kind,
            } => {
                let variant_sig = match raw_variant_kind {
                    EnumVariantKind::Constant => EnumVariantDecl::Constant,
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
        traits: Default::default(),
    }))
}
