use crate::*;
use visualizer_gen::VisualizerVariant;

pub(crate) fn visual_feature_repr(db: &dyn FeatureGenQueryGroup, this: FeatureRepr) -> FeatureRepr {
    let visualizer = db.visualizer(this.ty());
    match visualizer.variant {
        VisualizerVariant::Custom { ref stmts } => {
            FeatureLazyBlock::new(db, Some(this), stmts, &[], db.features()).into()
        }
        _ => panic!(),
    }
}
