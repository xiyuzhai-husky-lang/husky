use crate::*;
use husky_entity_route_syntax::RangedEntityRoute;
use husky_visualizer_gen::VisualizerVariant;
use vm::__EvalResult;
use word::RootIdentifier;

pub(crate) fn visual_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
) -> __EvalResult<FeatureRepr> {
    let visualizer = db.visualizer(this.ty());
    Ok(match visualizer.variant {
        VisualizerVariant::Custom { ref stmts } => FeatureLazyBlock::new(
            db,
            Some(this),
            stmts,
            &[],
            db.feature_interner(),
            RangedEntityRoute {
                route: RootIdentifier::VisualType.into(),
                range: Default::default(),
            },
        )
        .into(),
        _ => panic!(),
    })
}
