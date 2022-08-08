use entity_kind::EnumVariantKind;
use vec_like::VecMapEntry;

use super::*;

// pub(crate) fn enum_decl(
//     db: &dyn DeclQueryGroup,
//     this_ty: EntityRoutePtr,
//     generic_parameters: IdentDict<GenericPlaceholder>,
//     children: AstIter,
// ) -> InferResultArc<TyDecl> {
//     let mut variants = VecMap::default();
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
//         generic_parameters,
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
        static_defn: &EntityStaticDefn,
        symbol_context: &mut dyn AtomContext,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::EnumVariant => Self {
                ident: db.custom_ident(static_defn.name),
                variant: EnumVariantDeclVariant::Constant,
            },
            _ => panic!(),
        }
    }

    pub fn instantiate(&self, instantiator: &InstantiationContext) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDeclVariant {
    Constant,
}

impl VecMapEntry<CustomIdentifier> for EnumVariantDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
