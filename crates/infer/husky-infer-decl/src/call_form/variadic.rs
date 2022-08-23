pub use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum VariadicTemplate {
    None,
    SingleTyped { variadic_ty: EntityRoutePtr },
}

impl Default for VariadicTemplate {
    fn default() -> Self {
        VariadicTemplate::None
    }
}

impl VariadicTemplate {
    pub(crate) fn from_static(
        ctx: &mut dyn AtomContext,
        static_defn: &StaticVariadicTemplate,
    ) -> Self {
        match static_defn {
            StaticVariadicTemplate::None => VariadicTemplate::None,
            StaticVariadicTemplate::SingleTyped { ty } => VariadicTemplate::SingleTyped {
                variadic_ty: ctx.parse_entity_route(ty).unwrap(),
            },
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            VariadicTemplate::None => false,
            VariadicTemplate::SingleTyped { variadic_ty } => true,
        }
    }
}

impl Instantiable for VariadicTemplate {
    type Target = Self;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        match self {
            VariadicTemplate::None => VariadicTemplate::None,
            VariadicTemplate::SingleTyped { variadic_ty: ty } => VariadicTemplate::SingleTyped {
                variadic_ty: ty.instantiate(ctx).take_entity_route(),
            },
        }
    }
}

impl Implementable for VariadicTemplate {
    type Target = Self;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target {
        match self {
            VariadicTemplate::None => VariadicTemplate::None,
            VariadicTemplate::SingleTyped { variadic_ty: ty } => VariadicTemplate::SingleTyped {
                variadic_ty: ty.implement(ctx).take_entity_route(),
            },
        }
    }
}
