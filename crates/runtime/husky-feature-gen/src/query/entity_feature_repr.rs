use super::*;

pub(super) fn item_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    item_path: EtherealTerm,
) -> FeatureRepr {
    todo!()
    // let item_defn = db.item_defn(item_path).unwrap();
    // match item_defn.variant {
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
