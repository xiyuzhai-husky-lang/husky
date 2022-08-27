use super::FeatureEvaluator;
use crate::*;
use cyclic_slice::CyclicSlice;
use husky_check_utils::should_eq;
use husky_entity_semantics::{EntityDefnQueryGroup, VisualTy, Visualizer, VisualizerVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_print_utils::{epin, msg_once, p};
use husky_trace_protocol::VisualData;
use husky_vm::*;
use husky_word::IdentPairDict;
use std::{iter::zip, sync::Arc};

impl<'temp, 'eval> FeatureEvaluator<'temp, 'eval> {
    pub fn visualize_feature(&self, this: FeatureRepr) -> __VMResult<VisualData> {
        self.as_static().visualize_static(this)
    }
    pub fn visualize_static(&self, this: FeatureRepr) -> __VMResult<VisualData>
    where
        'eval: 'static,
    {
        let ty = this.ty();
        let visualizer: Arc<Visualizer> = self.db.comptime().visualizer(ty);
        match visualizer.variant {
            VisualizerVariant::Option {
                ref intrinsic_visualizer,
            } => {
                let value = self.eval_feature_repr_cached(&this)?;
                match value.data_kind() {
                    __RegisterDataKind::PrimitiveValue => todo!(),
                    __RegisterDataKind::Box => todo!(),

                    __RegisterDataKind::EvalRef => {
                        self.visualize_intrinsic(this, intrinsic_visualizer)
                    }
                    __RegisterDataKind::TempRef => todo!(),
                    __RegisterDataKind::TempMut => todo!(),
                    __RegisterDataKind::Moved => todo!(),
                    __RegisterDataKind::None => todo!(),
                    __RegisterDataKind::Unreturned => todo!(),
                }
            }
            _ => self.visualize_intrinsic(this, &visualizer),
        }
        // let visualizer = self.db.comptime().visualizer(this.ty());
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

    pub fn visualize_intrinsic(
        &self,
        this: FeatureRepr,
        visualizer: &Visualizer,
    ) -> __VMResult<VisualData>
    where
        'eval: 'static,
    {
        let ty = this.ty().intrinsic();
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
            VisualizerVariant::Custom { .. } => {
                let visual_feature_lazy_block = self.db.visual_feature_lazy_block(this)?;
                Ok(self
                    .eval_lazy_block(&visual_feature_lazy_block)?
                    .downcast_eval_ref::<VisualData>(&__VISUAL_DATA_VTABLE)
                    .clone())
            }
            VisualizerVariant::Void => {
                should_eq!(
                    visualizer.visual_ty,
                    VisualTy::Void,
                    "expect `{}` to be of visual type void",
                    ty
                );
                Ok(VisualData::Primitive { value: ().into() })
            }
            VisualizerVariant::Static { fp } => {
                let value = self.eval_feature_repr_cached(&this)?;
                (fp.0)(&value)
            }
            VisualizerVariant::Any => todo!(),
            VisualizerVariant::Option { .. } => panic!(),
        }
    }
    fn visualize_cyclic_slice(&self) -> __VMResult<VisualData> {
        todo!()
    }
}

// VisualizerVariant::Vec { ty } => {
//     let elem_ty = ty.entity_route_argument(0);
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
//     let elem_ty = ty.entity_route_argument(0);
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
//                         Feature::CyclicIndexConstIndex {
//                             this: this.feature(),
//                             index,
//                         },
//                     ),
//                 })
//             })
//             .collect::<__VMResult<_>>()?,
//     )
// }
