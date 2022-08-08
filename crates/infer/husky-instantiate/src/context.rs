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
}
