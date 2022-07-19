pub use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum VariadicTemplateDecl {
    None,
    SingleTyped { ty: EntityRoutePtr },
}

impl VariadicTemplateDecl {
    pub(crate) fn from_static(
        ctx: &mut dyn AtomContext,
        static_defn: &StaticVariadicTemplateDefn,
    ) -> Self {
        match static_defn {
            StaticVariadicTemplateDefn::None => VariadicTemplateDecl::None,
            StaticVariadicTemplateDefn::SingleTyped { ty } => VariadicTemplateDecl::SingleTyped {
                ty: ctx.parse_entity_route(ty).unwrap(),
            },
        }
    }
}

impl Instantiable for VariadicTemplateDecl {
    type Target = Self;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        match self {
            VariadicTemplateDecl::None => VariadicTemplateDecl::None,
            VariadicTemplateDecl::SingleTyped { ty } => VariadicTemplateDecl::SingleTyped {
                ty: ty.instantiate(ctx).take_entity_route(),
            },
        }
    }
}
