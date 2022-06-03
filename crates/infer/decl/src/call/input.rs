use atom::AtomContext;
use vec_map::HasKey;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDecl {
    pub liason: ParameterLiason,
    pub ty: EntityRoutePtr,
    pub ident: CustomIdentifier,
}

impl HasKey<CustomIdentifier> for InputDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

impl InputDecl {
    pub fn from_static(symbol_context: &mut dyn AtomContext, input: &StaticParameter) -> Self {
        // opt_this_ty,
        Self {
            ty: symbol_context.parse_entity_route(input.ty).unwrap(),
            liason: input.contract,
            ident: symbol_context.entity_syntax_db().custom_ident(input.name),
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
            ty: instantiator
                .instantiate_entity_route(self.ty)
                .take_entity_route(),
            liason: self.liason,
            ident: self.ident,
        }
    }

    pub fn implement(&self, implementor: &Implementor) -> Self {
        todo!()
    }
}

impl Into<InputDecl> for &Parameter {
    fn into(self) -> InputDecl {
        InputDecl {
            liason: self.liason,
            ty: self.ranged_ty.route,
            ident: self.ranged_ident.ident,
        }
    }
}
