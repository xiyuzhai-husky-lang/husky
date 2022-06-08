use super::*;
use check_utils::should_eq;

impl TyDecl {
    pub fn instantiate(
        &self,
        db: &dyn DeclQueryGroup,
        dst_generics: &[SpatialArgument],
    ) -> Arc<Self> {
        should_eq!(self.generic_parameters.len(), dst_generics.len());
        let instantiator = Instantiator {
            db: db.upcast(),
            generic_parameters: &self.generic_parameters,
            dst_generics,
        };
        Self::new(
            db,
            instantiator
                .instantiate_entity_route(self.this_ty)
                .take_entity_route(),
            Default::default(), // generic_parameters
            self.ty_members
                .map(|member| member.instantiate(&instantiator)), //   type_methods
            self.variants
                .map(|variant| variant.instantiate(&instantiator)), //   variants
            self.kind,          //      kind
            self.trait_impls.map(|t| t.instantiate(&instantiator)), //   trait_impls
            self.opt_type_call
                .as_ref()
                .map(|type_call| type_call.instantiate(&instantiator)),
        )
    }
}
