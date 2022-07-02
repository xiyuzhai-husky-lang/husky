use super::*;
use vm::__EvalResult;

pub(super) fn main_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    main_file: husky_file::FilePtr,
) -> FeatureRepr {
    let package = db.compile_time().package(main_file).unwrap();
    let main = &*package.main_defn;
    FeatureRepr::from_defn(db, None, &main.defn_repr, db.feature_interner())
}
