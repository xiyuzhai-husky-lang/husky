use super::*;
use vm::__VMResult;

pub(super) fn main_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    crate_entrance: husky_file::FilePtr,
) -> FeatureRepr {
    let package = db.compile_time().package(crate_entrance).unwrap();
    let main = &*package.main_defn;
    FeatureRepr::from_defn(db, None, &main.defn_repr, db.feature_interner())
}
