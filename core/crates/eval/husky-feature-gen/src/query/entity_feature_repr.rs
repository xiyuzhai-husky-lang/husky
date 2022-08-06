use vm::__VMResult;

use super::*;

pub(super) fn entity_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> FeatureRepr {
    let entity_defn = db.comptime().entity_defn(entity_route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Feature { ref defn_repr, .. } => {
            FeatureRepr::from_defn(db, None, defn_repr, db.feature_interner())
        }
        EntityDefnVariant::Input { target_entrance } => FeatureRepr::EvalInput {
            ty: db.comptime().target_input_ty(target_entrance).unwrap(),
            feature: db.feature_interner().intern(Feature::Input {}),
        },
        _ => todo!(),
    }
}
