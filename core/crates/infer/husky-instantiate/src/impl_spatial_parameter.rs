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
                SpatialParameterVariant::Const => SpatialParameterVariant::Const,
                SpatialParameterVariant::Type { ref traits } => SpatialParameterVariant::Type {
                    traits: traits.map(|trai| todo!()),
                },
            },
            file: self.file,
            range: self.range,
        })
    }
}
