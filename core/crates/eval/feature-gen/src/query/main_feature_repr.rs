use super::*;

pub(super) fn main_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResult<FeatureRepr> {
    let pack = db.compile_time().package(main_file)?;
    let main = &*pack.main_defn;
    Ok(FeatureRepr::from_defn(
        db,
        None,
        &main.defn_repr,
        db.feature_interner(),
    ))
}
