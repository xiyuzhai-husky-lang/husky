use crate::*;
use husky_entity_semantics::{Visualizer, VisualizerVariant};
use husky_term::Term;
use husky_vm::__VMResult;
use husky_word::RootBuiltinIdent;

pub(crate) fn visual_feature_lazy_block(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
) -> __VMResult<Arc<FeatureLazyBody>> {
    todo!()
    // let visualizer: Arc<Visualizer> = db.visualizer(this.ty().intrinsic());
    // Ok(FeatureLazyBody::new(
    //     db,
    //     Some(this),
    //     match visualizer.variant {
    //         VisualizerVariant::Custom { ref stmts } => stmts,
    //         _ => panic!(),
    //     },
    //     &[],
    //     None,
    //     db.feature_interner(),
    //     Term {
    //         route: RootBuiltinIdent::VisualType.into(),
    //         range: Default::default(),
    //     },
    // ))
}
