use super::*;
use husky_check_utils::should_eq;
use husky_print_utils::msg_once;

impl TyDecl {
    pub fn instantiate(
        &self,
        db: &dyn DeclQueryGroup,
        spatial_arguments: &[SpatialArgument],
    ) -> Arc<Self> {
        // should_eq!(self.spatial_parameters.len(), spatial_arguments.len());
        let ctx = InstantiationContext {
            db: db.upcast(),
            spatial_parameters: &self.spatial_parameters,
            spatial_arguments,
        };
        Self::new(
            db,
            self.this_ty.instantiate(&ctx).take_entity_route(),
            Default::default(), // generic_parameters
            self.ty_members.map(|member| member.instantiate(&ctx)), //   type_methods
            self.variants.map(|variant| variant.instantiate(&ctx)), //   variants
            self.ty_kind,       //      kind
            self.trait_impls.map(|t| t.instantiate(&ctx)), //   trait_impls
            self.opt_type_call
                .as_ref()
                .map(|type_call| type_call.instantiate(&ctx)),
        )
    }
}
