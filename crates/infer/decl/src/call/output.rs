use check_utils::should_eq;
use implement::Implementable;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputDecl {
    pub liason: OutputLiason,
    pub ty: EntityRoutePtr,
}

impl OutputDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
            ty: instantiator
                .instantiate_entity_route(self.ty)
                .as_entity_route(),
            liason: self.liason,
        }
    }
}

impl Implementable for OutputDecl {
    type Target = Self;

    fn implement(&self, implementor: &Implementor) -> Self::Target {
        Self {
            liason: self.liason,
            ty: self.ty.implement(implementor),
        }
    }
}
