use crate::*;
use husky_atom::*;

impl HuskyCompileTime {
    pub fn parse_entity_route(
        &self,
        opt_package_main: Option<FilePtr>,
        text: &str,
    ) -> EntityRoutePtr {
        let mut context = AtomContextStandalone {
            opt_package_main,
            db: self,
            opt_this_ty: None,
            opt_this_contract: None,
            symbols: (&[] as &[Symbol]).into(),
            kind: AtomContextKind::Normal,
        };
        context.parse_entity_route(text).unwrap()
    }
}
