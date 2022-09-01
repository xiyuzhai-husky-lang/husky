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
    pub fn new(
        db: &dyn DeclQueryGroup,
        liason: ParameterModifier,
        ty: EntityRoutePtr,
        ident: CustomIdentifier,
    ) -> InferResult<Self> {
        assert!(liason.is_compatible(ty));
        let ty = implement_target(db, ty)?;
        Ok(Self { liason, ty, ident })
    }

    pub fn ty(&self) -> EntityRoutePtr {
        self.ty
    }

    pub fn from_static(
        db: &dyn DeclQueryGroup,
        symbol_context: &mut dyn AtomContext,
        parameter: &StaticParameter,
    ) -> InferResult<Self> {
        // opt_this_ty,
        Self::new(
            db,
            parameter.modifier,
            symbol_context.parse_entity_route(parameter.ty).unwrap(),
            symbol_context
                .entity_syntax_db()
                .custom_ident(parameter.name),
        )
    }

    pub fn from_field(db: &dyn DeclQueryGroup, field_decl: &FieldDecl) -> InferResult<Self> {
        ParameterDecl::new(
            db,
            ParameterModifier::from_field(field_decl.modifier),
            field_decl.ty,
            field_decl.ident,
        )
    }

    pub fn from_parameter(db: &dyn DeclQueryGroup, parameter: &Parameter) -> InferResult<Self> {
        ParameterDecl::new(db, parameter.liason(), parameter.ty(), parameter.ident())
    }

    pub fn instantiate(&self, ctx: &InstantiationContext) -> Self {
        Self {
            ty: self.ty.instantiate(ctx).take_entity_route(),
            liason: self.liason,
            ident: self.ident,
        }
    }
}

impl Implementable for ParameterDecl {
    type Target = Self;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target {
        Self {
            liason: self.liason,
            ty: self.ty.implement(ctx).take_entity_route(),
            ident: self.ident,
        }
    }
}
