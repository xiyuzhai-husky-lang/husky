use super::*;

pub(super) fn item_feature_repr(db: &dyn ValReprDb, item_path: EtherealTerm) -> ValRepr {
    todo!()
    // let item_defn = db.item_defn(item_path).unwrap();
    // match item_defn.variant {
    //     EntityDefnVariant::Feature { ref defn_repr, .. } => {
    //         ValRepr::from_defn(db, None, defn_repr, db.feature_interner())
    //     }
    //     EntityDefnVariant::TargetInput => ValRepr::input(db),
    //     _ => todo!(),
    // }
}

impl ValRepr {
    pub fn input(_db: &dyn ValReprDb) -> Self {
        todo!()
        // ValRepr::TargetInput {
        //     ty: db.target_parameter_ty().unwrap(),
        //     feature: db.feature_interner().intern(Feature::Input {}),
        //     main_file: db.target_entrance(),
        // }
    }
}
