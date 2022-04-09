use crate::*;
use atom::*;
use entity_route::*;
use fold::LocalStack;

impl<'a> dyn DeclQueryGroup + 'a {
    pub(crate) fn parse_entity(&self, text: &str) -> AtomResult<EntityRoutePtr> {
        parse_entity(
            SymbolProxy {
                opt_package_main: None,
                db: self.upcast(),
                this_ty: None,
                symbols: &LocalStack::new(),
            },
            &self.tokenize(text),
        )
    }
}
