use crate::*;
use visualizer_gen::VisualizerVariant;

pub(crate) fn visual_feature_repr<'eval>(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
) -> FeatureRepr {
    let visualizer = db.visualizer(this.ty());
    match visualizer.variant {
        VisualizerVariant::Custom { ref stmts } => {
            FeatureLazyBlock::new(db, Some(this), stmts, &[], db.feature_interner()).into()
        }
        _ => panic!(),
    }
}
