use crate::*;
use husky_coword::RootBuiltinIdent;
use husky_ethereal_term::EtherealTerm;
use husky_item_semantics::{Visualizer, VisualizerVariant};
use husky_vm::__VMResult;

pub(crate) fn visual_feature_lazy_block(db: &dyn ValReprDb, this: ValRepr) -> __VMResult<ValBlock> {
    todo!()
    // let visualizer: Arc<Visualizer> = db.visualizer(this.ty().intrinsic());
    // Ok(ValBlock::new(
    //     db,
    //     Some(this),
    //     match visualizer.variant {
    //         VisualizerVariant::Custom { ref stmts } => stmts,
    //         _ => panic!(),
    //     },
    //     &[],
    //     None,
    //     db.feature_interner(),
    //     EtherealTerm {
    //         route: RootBuiltinIdent::VisualType.into(),
    //         range: Default::default(),
    //     },
    // ))
}
