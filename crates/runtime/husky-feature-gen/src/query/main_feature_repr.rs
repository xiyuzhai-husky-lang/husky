use super::*;

pub(super) fn main_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    target_entrance: husky_source_path::SourcePath,
) -> FeatureRepr {
    let package = db.package(target_entrance).unwrap();
    let main = &*package.main_defn;
    FeatureRepr::from_defn(db, None, &main.defn_repr, db.feature_interner())
}
