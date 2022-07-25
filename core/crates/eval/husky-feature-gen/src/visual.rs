use crate::*;
use husky_entity_route::RangedEntityRoute;
use husky_word::RootIdentifier;
use vm::__VMResult;

pub(crate) fn visual_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
) -> __VMResult<FeatureRepr> {
    let visualizer = db.compile_time().visualizer(this.ty());
    Ok(FeatureLazyBlock::new(
        db,
        Some(this),
        visualizer.opt_stmts.as_ref().unwrap(),
        &[],
        None,
        db.feature_interner(),
        RangedEntityRoute {
            route: RootIdentifier::VisualType.into(),
            range: Default::default(),
        },
    )
    .into())
}
