use atom::symbol_proxy::Symbol;
use fold::LocalStack;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDecl {
    pub route: EntityRoutePtr,
    // associated types
}

impl TraitImplDecl {
    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        trait_impl: &StaticTraitImplDecl,
        this_ty: EntityRoutePtr,
        symbols: &LocalStack<Symbol>,
    ) -> Arc<Self> {
        Arc::new(Self {
            route: db
                .parse_entity(trait_impl.route, Some(this_ty), symbols)
                .unwrap(),
        })
    }

    pub(crate) fn clone_trait_impl(db: &dyn DeclQueryGroup, this_ty: EntityRoutePtr) -> Arc<Self> {
        todo!()
    }

    pub(crate) fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            route: instantiator.instantiate_entity_route(self.route).as_scope(),
        })
    }
}
