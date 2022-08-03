use crate::*;

impl FilterInstantiable for SpatialParameter {
    type Target = SpatialParameter;

    fn instantiate(&self, ctx: &InstantiationContext) -> Option<Self::Target> {
        for spatial_argument in ctx.spatial_parameters.iter() {
            if spatial_argument.ident == self.ident {
                return None;
            }
        }
        Some(SpatialParameter {
            ident: self.ident,
            variant: match self.variant {
                SpatialPlaceholderVariant::Const => SpatialPlaceholderVariant::Const,
                SpatialPlaceholderVariant::Type { ref traits } => SpatialPlaceholderVariant::Type {
                    traits: traits.map(|trai| todo!()),
                },
            },
        })
    }
}
