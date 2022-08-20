use crate::*;

pub(crate) fn module_contains_features(
    db: &dyn EntityDefnQueryGroup,
    module_route: EntityRoutePtr,
) -> bool {
    let subentity_defns = db.subentity_defns(module_route).unwrap();
    for subentity_defn in &*subentity_defns {
        match subentity_defn.variant {
            EntityDefnVariant::Module { .. } => {
                if db.module_contains_features(subentity_defn.base_route) {
                    return true;
                }
            }
            EntityDefnVariant::Feature { ref defn_repr } => return true,
            _ => (),
        }
    }
    false
}
