use super::FeatureEvaluator;
use crate::*;
use cyclic_slice::CyclicSlice;
use husky_check_utils::should_eq;
use husky_entity_semantics::{EntityDefnQueryGroup, VisualTy, Visualizer, VisualizerVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_print_utils::{epin, msg_once, p};
use husky_trace_protocol::VisualData;
use husky_word::IdentPairDict;
use std::{iter::zip, sync::Arc};
use vm::*;

impl<'temp, 'eval> FeatureEvaluator<'temp, 'eval> {
    pub fn visualize_feature(&self, this: FeatureRepr) -> __VMResult<VisualData> {
        self.as_static().visualize_static(this)
    }
    pub fn visualize_static(&self, this: FeatureRepr) -> __VMResult<VisualData>
    where
        'eval: 'static,
    {
        let ty = this.ty();
        let visualizer: Arc<Visualizer> = self.db.compile_time().visualizer(ty);
        match visualizer.variant {
            VisualizerVariant::Group { element_ty } => {
                let ty_data_viewer = self.db.ty_data_viewer(ty);
                let value = self.eval_feature_repr_cached(&this)?;
                let mut elements = vec![];
                for (index, element) in ty_data_viewer.member_eval_indexed_iter(&value) {
                    elements.push(self.visualize_feature(FeatureRepr::Value {
                        value: element,
                        file: this.file(),
                        range: this.text_range(),
                        ty: element_ty,
                        feature: self.db.feature_interner().intern(Feature::IndexFixed {
                            this: this.feature(),
                            index: index as usize,
                        }),
                    })?)
                }
                Ok(VisualData::Group(elements))
            }
            VisualizerVariant::Custom { ref opt_stmts } => {
                if opt_stmts.is_none() {
                    should_eq!(
                        visualizer.visual_ty,
                        VisualTy::Void,
                        "expect `{}` to be of visual type void",
                        ty
                    );
                    Ok(VisualData::Primitive { value: ().into() })
                } else {
                    let visual_feature_lazy_block = self.db.visual_feature_lazy_block(this)?;
                    Ok(self
                        .eval_feature_lazy_block(&visual_feature_lazy_block)?
                        .downcast_eval_ref::<VisualData>()
                        .clone())
                }
            }
        }
        // let visualizer = self.db.compile_time().visualizer(this.ty());
        // let this_value = self.eval_feature_repr_cached(&this).unwrap();
        // todo!()
        // if let Some(visual_data) =
        //     this_value
        //         .eval_ref()
        //         .0
        //         .__opt_visualize_dyn(&mut |index, elem| {

        //         })?
        // {
        //     return Ok(visual_data);
        // }
    }

    fn visualize_cyclic_slice(&self) -> __VMResult<VisualData> {
        todo!()
    }
}

// VisualizerVariant::Vec { ty } => {
//     let elem_ty = ty.spatial_arguments[0].take_entity_route();
//     let elem_visualizer = self.db.visualizer(elem_ty);
//     let any_value_dyn: &'static dyn AnyValueDyn<'static> = this_value.eval_ref().0;
//     let virtual_vec: &VirtualVec<'static> = any_value_dyn.downcast_ref();
//     VisualData::Group(
//         virtual_vec
//             .iter()
//             .enumerate()
//             .map(

//     )
// }
// VisualizerVariant::CyclicSlice { ty } => {
//     let elem_ty = ty.spatial_arguments[0].take_entity_route();
//     let elem_visualizer = self.db.visualizer(elem_ty);
//     let any_value_dyn: &'static dyn AnyValueDyn<'static> = this_value.eval_ref().0;
//     let virtual_cyclic_slice: &VirtualCyclicSlice<'eval> = any_value_dyn.downcast_ref();
//     VisualData::Group(
//         virtual_cyclic_slice
//             .enum_iter()
//             .map(|(index, elem)| {
//                 self.visualize_feature(FeatureRepr::Value {
//                     value: __EvalRef(elem.any_ref()),
//                     file: this.file(),
//                     range: this.text_range(),
//                     ty: elem_ty,
//                     feature: self.db.feature_interner().intern(
//                         Feature::CyclicElementAccessConstIndex {
//                             this: this.feature(),
//                             index,
//                         },
//                     ),
//                 })
//             })
//             .collect::<__VMResult<_>>()?,
//     )
// }
