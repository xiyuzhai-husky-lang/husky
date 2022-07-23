use husky_check_utils::should_eq;
use husky_implement::Implementable;
use husky_instantiate::Instantiable;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputDecl {
    pub liason: OutputLiason,
    pub ty: EntityRoutePtr,
}

impl Instantiable for OutputDecl {
    type Target = Self;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self {
        Self {
            ty: self.ty.instantiate(ctx).take_entity_route(),
            liason: self.liason,
        }
    }
}

impl Implementable for OutputDecl {
    type Target = Self;

    fn implement(&self, implementor: &ImplementationContext) -> Self::Target {
        Self {
            liason: self.liason,
            ty: self.ty.implement(implementor).take_entity_route(),
        }
    }
}
