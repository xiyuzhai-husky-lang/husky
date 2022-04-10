use super::*;
use check_utils::should_eq;

impl TyDecl {
    pub fn instantiate(&self, db: &dyn DeclQueryGroup, dst_generics: &[GenericArgument]) -> Self {
        should_eq!(self.generic_placeholders.len(), dst_generics.len());
        let instantiator = Instantiator {
            db: db.upcast(),
            generic_placeholders: &self.generic_placeholders,
            dst_generics,
        };
        Self::new(
            db,
            instantiator
                .instantiate_entity_route(self.this_ty)
                .as_scope(),
            Default::default(), // generic_placeholders
            self.type_members
                .map(|member| member.instantiate(&instantiator)), //   type_methods
            self.variants
                .map(|variant| variant.instantiate(&instantiator)), //   variants
            self.kind,          //      kind
            self.trait_impls.map(|t| t.instantiate(&instantiator)), //   trait_impls
        )
    }
}
