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
        Self {
            this_ty: instantiator
                .instantiate_entity_route(self.this_ty)
                .as_scope(),
            generic_placeholders: Default::default(),
            traits: self
                .traits
                .iter()
                .map(|t| instantiator.instantiate_entity_route(*t).as_scope())
                .collect(),
            methods: self.methods.map(|method| method.instantiate(&instantiator)),
            kind: self.kind,
            fields: self.fields.map(|field| field.instantiate(&instantiator)),
            variants: self
                .variants
                .map(|variant| variant.instantiate(&instantiator)),
        }
    }
}
