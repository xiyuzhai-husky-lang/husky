use super::*;

pub(super) fn main_feature_repr(db: &dyn ValReprDb, target_entrance: EntityPath) -> ValRepr {
    let package = db.package(target_entrance).unwrap();
    let main = &*package.main_defn;
    ValRepr::from_defn(db, None, &main.defn_repr, db.feature_interner())
}
