use crate::*;

impl Instantiable for SpatialArgument {
    type Target = SpatialArgument;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        match self {
            SpatialArgument::Const(_) => *self,
            SpatialArgument::EntityRoute(route) => route.instantiate(ctx),
        }
    }
}
