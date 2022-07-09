use crate::*;
use husky_entity_route::RangedEntityRoute;
use vm::__EvalResult;
use word::RootIdentifier;

pub(crate) fn visual_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
) -> __EvalResult<FeatureRepr> {
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
