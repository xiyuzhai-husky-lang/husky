use vm::EvalResult;

use super::*;

pub(super) fn entity_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> FeatureRepr {
    let entity = db.compile_time().entity_defn(entity_route).unwrap();
    match entity.variant {
        EntityDefnVariant::Feature { ref defn_repr, .. } => {
            FeatureRepr::from_defn(db, None, defn_repr, db.feature_interner())
        }
        _ => todo!(),
    }
}
