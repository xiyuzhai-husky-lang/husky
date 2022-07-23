use super::FeatureEvaluator;
use crate::*;
use cyclic_slice::CyclicSlice;
use husky_check_utils::should_eq;
use husky_entity_semantics::EntityDefnQueryGroup;
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_print_utils::{epin, msg_once, p};
use husky_trace_protocol::VisualData;
use std::{iter::zip, sync::Arc};
use vm::*;
use word::IdentPairDict;

impl<'temp, 'eval> FeatureEvaluator<'temp, 'eval> {
    pub fn visualize_feature(&mut self, this: FeatureRepr) -> __EvalResult<VisualData> {
        self.as_static().visualize_static(this)
    }
    pub fn visualize_static(&mut self, this: FeatureRepr) -> __EvalResult<VisualData>
    where
        'eval: 'static,
    {
        let visualizer = self.db.compile_time().visualizer(this.ty());
        let this_value = self.eval_feature_repr_cached(&this).unwrap();
        should_eq!(this_value.any_ref().__ty_dyn(), this.ty());
        if let Some(visual_data) =
            this_value
                .eval_ref()
                .0
                .__opt_visualize_dyn(&mut |index, elem| {
                    self.visualize_feature(FeatureRepr::Value {
                        value: __EvalRef(elem),
                        file: this.file(),
                        range: this.text_range(),
                        ty: elem.__ty_dyn(),
                        feature: self.db.feature_interner().intern(Feature::IndexFixed {
                            this: this.feature(),
                            index,
                        }),
                    })
                })?
        {
            return Ok(visual_data);
        }
        let visual_feature = self.db.visual_feature_repr(this)?;
        Ok(self
            .eval_feature_repr(&visual_feature)?
            .any_ref()
            .__downcast_ref::<VisualData>()
            .clone())
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
//             .collect::<__EvalResult<_>>()?,
//     )
// }
