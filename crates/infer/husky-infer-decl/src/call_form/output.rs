use husky_check_utils::should_eq;
use husky_implement::Implementable;
use husky_instantiate::Instantiable;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputDecl {
    liason: OutputModifier,
    ty: EntityRoutePtr,
}

impl OutputDecl {
    pub(crate) fn new(
        db: &dyn DeclQueryGroup,
        liason: OutputModifier,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        let ty = implement_target(db, ty)?;
        Ok(Self { liason, ty })
    }

    pub fn liason(&self) -> OutputModifier {
        self.liason
    }

    pub fn ty(&self) -> EntityRoutePtr {
        self.ty
    }
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
