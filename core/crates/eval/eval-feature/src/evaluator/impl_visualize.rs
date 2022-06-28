use crate::*;
use check_utils::should_eq;
use cyclic_slice::CyclicSlice;
use feature_gen::*;
use husky_tracer_protocol::VisualData;
use print_utils::{epin, msg_once, p};
use semantics_lazy::LazyStmt;
use std::{iter::zip, sync::Arc};
use visualizer_gen::VisualizerVariant;
use vm::*;
use word::IdentPairDict;

use super::FeatureEvaluator;

impl<'temp, 'eval> FeatureEvaluator<'temp, 'eval> {
    pub fn visualize_feature(&mut self, this: FeatureRepr) -> EvalResult<VisualData> {
        self.as_static().visualize_static(this)
    }
    pub fn visualize_static(&mut self, this: FeatureRepr) -> EvalResult<VisualData>
    where
        'eval: 'static,
    {
        let visualizer = self.db.visualizer(this.ty());
        let this_value = self.eval_feature_repr_cached(&this).unwrap();
        should_eq!(this_value.any_ref().ty_dyn(), this.ty());
        Ok(match visualizer.variant {
            VisualizerVariant::Compiled { call } => call(this_value.any_ref()),
            VisualizerVariant::Vec { ty } => {
                let elem_ty = ty.spatial_arguments[0].take_entity_route();
                let elem_visualizer = self.db.visualizer(elem_ty);
                let any_value_dyn: &'static dyn AnyValueDyn<'static> = this_value.eval_ref().0;
                let virtual_vec: &VirtualVec<'static> = any_value_dyn.downcast_ref();
                VisualData::Group(
                    virtual_vec
                        .iter()
                        .enumerate()
                        .map(|(index, elem)| {
                            self.visualize_feature(FeatureRepr::Value {
                                value: EvalRef(elem.any_ref()),
                                file: this.file(),
                                range: this.text_range(),
                                ty: elem_ty,
                                feature: self.db.feature_interner().intern(
                                    Feature::ElementAccessConstIndex {
                                        this: this.feature(),
                                        index,
                                    },
                                ),
                            })
                        })
                        .collect::<EvalResult<_>>()?,
                )
            }
            VisualizerVariant::CyclicSlice { ty } => {
                let elem_ty = ty.spatial_arguments[0].take_entity_route();
                let elem_visualizer = self.db.visualizer(elem_ty);
                let any_value_dyn: &'static dyn AnyValueDyn<'static> = this_value.eval_ref().0;
                let virtual_cyclic_slice: &VirtualCyclicSlice<'eval> = any_value_dyn.downcast_ref();
                VisualData::Group(
                    virtual_cyclic_slice
                        .enum_iter()
                        .map(|(index, elem)| {
                            self.visualize_feature(FeatureRepr::Value {
                                value: EvalRef(elem.any_ref()),
                                file: this.file(),
                                range: this.text_range(),
                                ty: elem_ty,
                                feature: self.db.feature_interner().intern(
                                    Feature::CyclicElementAccessConstIndex {
                                        this: this.feature(),
                                        index,
                                    },
                                ),
                            })
                        })
                        .collect::<EvalResult<_>>()?,
                )
            }
            VisualizerVariant::Custom { ref stmts } => {
                let visual_feature = self.db.visual_feature_repr(this)?;
                self.eval_feature_repr(&visual_feature)?
                    .any_ref()
                    .downcast_ref::<VisualData>()
                    .clone()
            }
            VisualizerVariant::Todo => todo!(),
        })
        // let visualizer = self.compile_time().visualizer(ty);
        // visualizer.visualize(self.compile_time(), value, self.verbose())
    }
}
