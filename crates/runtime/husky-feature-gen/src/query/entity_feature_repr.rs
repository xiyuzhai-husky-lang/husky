use super::*;

pub(super) fn entity_feature_repr(db: &dyn FeatureGenQueryGroup, entity_path: Term) -> FeatureRepr {
    todo!()
    // let entity_defn = db.entity_defn(entity_path).unwrap();
    // match entity_defn.variant {
    //     EntityDefnVariant::Feature { ref defn_repr, .. } => {
    //         FeatureRepr::from_defn(db, None, defn_repr, db.feature_interner())
    //     }
    //     EntityDefnVariant::TargetInput => FeatureRepr::input(db),
    //     _ => todo!(),
    // }
}

impl FeatureRepr {
    pub fn input(_db: &dyn FeatureGenQueryGroup) -> Self {
        todo!()
        // FeatureRepr::TargetInput {
        //     ty: db.target_parameter_ty().unwrap(),
        //     feature: db.feature_interner().intern(Feature::Input {}),
        //     main_file: db.target_entrance(),
        // }
    }
}
