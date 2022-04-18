use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDecl {
    pub contract: InputContract,
    pub ty: EntityRoutePtr,
    pub ident: CustomIdentifier,
}

impl InputDecl {
    pub fn from_static(
        db: &dyn DeclQueryGroup,
        input: &StaticInputDecl,
        opt_this_ty: Option<EntityRoutePtr>,
        symbols: &[Symbol],
    ) -> Self {
        Self {
            ty: db.parse_entity(input.ty, opt_this_ty, symbols).unwrap(),
            contract: input.contract,
            ident: db.custom_ident(input.name),
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
            ty: instantiator.instantiate_entity_route(self.ty).as_scope(),
            contract: self.contract,
            ident: self.ident,
        }
    }

    pub fn implement(&self, implementor: &Implementor) -> Self {
        todo!()
    }
}

impl Into<InputDecl> for &InputPlaceholder {
    fn into(self) -> InputDecl {
        InputDecl {
            contract: self.contract,
            ty: self.ranged_ty.route,
            ident: self.ident,
        }
    }
}
