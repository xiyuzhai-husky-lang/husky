use crate::*;
use visualizer_gen::VisualizerVariant;
use vm::EvalResult;

pub(crate) fn visual_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
) -> EvalResult<FeatureRepr> {
    let visualizer = db.visualizer(this.ty());
    Ok(match visualizer.variant {
        VisualizerVariant::Custom { ref stmts } => {
            FeatureLazyBlock::new(db, Some(this), stmts, &[], db.feature_interner()).into()
        }
        _ => panic!(),
    })
}
