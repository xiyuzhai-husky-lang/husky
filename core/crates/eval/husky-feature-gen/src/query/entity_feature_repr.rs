use vm::__VMResult;

use super::*;

pub(super) fn entity_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> FeatureRepr {
    let entity_defn = db.compile_time().entity_defn(entity_route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Feature { ref defn_repr, .. } => {
            FeatureRepr::from_defn(db, None, defn_repr, db.feature_interner())
        }
        EntityDefnVariant::Input { main_file } => FeatureRepr::EvalInput {
            ty: db.compile_time().crate_input_ty(main_file).unwrap(),
            feature: db.feature_interner().intern(Feature::Input {}),
        },
        _ => todo!(),
    }
}
