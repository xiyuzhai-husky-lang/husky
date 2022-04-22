use entity_kind::EnumVariantKind;
use vec_dict::HasKey;

use super::*;

// pub(crate) fn enum_decl(
//     db: &dyn DeclQueryGroup,
//     this_ty: EntityRoutePtr,
//     generic_placeholders: IdentDict<GenericPlaceholder>,
//     children: AstIter,
// ) -> InferResultArc<TyDecl> {
//     let mut variants = VecDict::default();
//     let mut trait_impls = Vec::new();
//     for subitem in children {
//         match subitem.value.as_ref()?.kind {
//             AstKind::EnumVariantDefnHead {
//                 ident,
//                 variant_class: ref raw_variant_kind,
//             } => variants.insert_new(EnumVariantDecl {
//                 ident,
//                 variant: match raw_variant_kind {
//                     EnumVariantKind::Constant => EnumVariantDeclVariant::Constant,
//                 },
//             }),
//             _ => panic!(),
//         }
//     }
//     Ok(TyDecl::new(
//         db,
//         this_ty,
//         generic_placeholders,
//         Default::default(), // type_methods
//         variants,
//         TyKind::Enum,
//         trait_impls,
//     ))
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumVariantDecl {
    pub ident: CustomIdentifier,
    pub variant: EnumVariantDeclVariant,
}

impl EnumVariantDecl {
    pub fn from_static(
        db: &dyn DeclQueryGroup,
        static_decl: &EntityStaticDefn,
        symbol_context: &SymbolContext,
    ) -> Self {
        todo!()
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDeclVariant {
    Constant,
}

impl HasKey<CustomIdentifier> for EnumVariantDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
