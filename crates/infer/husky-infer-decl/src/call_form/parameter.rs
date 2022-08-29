use husky_atom::AtomContext;
use vec_like::VecMapEntry;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDecl {
    pub liason: ParameterModifier,
    ty: EntityRoutePtr,
    pub ident: CustomIdentifier,
}

impl VecMapEntry<CustomIdentifier> for ParameterDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

impl ParameterDecl {
    pub fn new(liason: ParameterModifier, ty: EntityRoutePtr, ident: CustomIdentifier) -> Self {
        assert!(liason.is_compatible(ty));
        Self { liason, ty, ident }
    }

    pub fn ty(&self) -> EntityRoutePtr {
        self.ty
    }

    pub fn from_static(symbol_context: &mut dyn AtomContext, input: &StaticParameter) -> Self {
        // opt_this_ty,
        Self::new(
            input.modifier,
            symbol_context.parse_entity_route(input.ty).unwrap(),
            symbol_context.entity_syntax_db().custom_ident(input.name),
        )
    }

    pub fn from_field(db: &dyn DeclQueryGroup, field_decl: &FieldDecl) -> InferResult<Self> {
        Ok(ParameterDecl::new(
            ParameterModifier::from_field(field_decl.modifier),
            field_decl.ty,
            field_decl.ident,
        ))
    }

    pub fn from_parameter(db: &dyn DeclQueryGroup, parameter: &Parameter) -> InferResult<Self> {
        Ok(ParameterDecl::new(
            parameter.liason(),
            db.implement_target(parameter.ty())?,
            parameter.ident(),
        ))
    }

    pub fn instantiate(&self, ctx: &InstantiationContext) -> Self {
        Self {
            ty: self.ty.instantiate(ctx).take_entity_route(),
            liason: self.liason,
            ident: self.ident,
        }
    }

    pub fn implement(&self, implementor: &ImplementationContext) -> Self {
        todo!()
    }
}
