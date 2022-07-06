use crate::*;

pub struct InstantiationContext<'a> {
    pub db: &'a dyn EntitySyntaxSalsaQueryGroup,
    pub spatial_parameters: &'a [SpatialParameter],
    pub spatial_arguments: &'a [SpatialArgument],
}

impl<'a> InstantiationContext<'a> {
    pub(crate) fn find_generic(&self, ident: CustomIdentifier) -> Option<usize> {
        self.spatial_parameters
            .iter()
            .position(|p| p.ident.ident == ident)
    }

    pub fn instantiate_generic_arguments(
        &self,
        src_generic_arguments: &[SpatialArgument],
    ) -> ThinVec<SpatialArgument> {
        src_generic_arguments
            .iter()
            .map(|src_generic| match src_generic {
                SpatialArgument::Const(_) => *src_generic,
                SpatialArgument::EntityRoute(route) => route.instantiate(self),
            })
            .collect()
    }

    pub fn instantiate_generic_placeholder(
        &self,
        placeholder: &SpatialParameter,
    ) -> Option<SpatialParameter> {
        for targeted_placeholder in self.spatial_parameters.iter() {
            if targeted_placeholder.ident == placeholder.ident {
                todo!()
            }
        }
        Some(SpatialParameter {
            ident: placeholder.ident,
            variant: match placeholder.variant {
                GenericPlaceholderVariant::Const => GenericPlaceholderVariant::Const,
                GenericPlaceholderVariant::Type { ref traits } => GenericPlaceholderVariant::Type {
                    traits: traits.map(|trai| todo!()),
                },
            },
        })
    }
}
