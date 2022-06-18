use crate::*;
use feature_gen::*;
use husky_compile_time::VisualizerQueryGroup;
use husky_tracer_protocol::VisualData;
use print_utils::{epin, msg_once, p};
use semantics_lazy::LazyStmt;
use std::{iter::zip, sync::Arc};
use visualizer_gen::VisualizerVariant;
use vm::*;
use word::IdentPairDict;

use super::FeatureEvaluator;

impl<'temp> FeatureEvaluator<'temp, 'static> {
    pub fn visualize(&mut self, this: FeatureRepr) -> VisualData {
        let visualizer = self.db.compile_time().visualizer(this.ty());
        match visualizer.variant {
            VisualizerVariant::Compiled { call } => todo!(),
            VisualizerVariant::Vec { ty } => {
                let value = self.eval_feature_repr_cached(&this).unwrap();
                let elem_ty = ty.spatial_arguments[0].take_entity_route();
                let elem_visualizer = self.db.compile_time().visualizer(elem_ty);
                let any_value_dyn: &'static dyn AnyValueDyn<'static> = value.eval_ref().0;
                let virtual_vec: &Vec<MemberValue> =
                    any_value_dyn.downcast_ref::<Vec<MemberValue<'static>>>();
                VisualData::Group(
                    virtual_vec
                        .iter()
                        .enumerate()
                        .map(|(index, elem)| {
                            self.visualize(FeatureRepr::Value {
                                value: EvalRef(elem.any_ref()),
                                file: this.file(),
                                range: this.text_range(),
                                ty: elem_ty,
                                feature: self.db.compile_time().feature_interner().intern(
                                    Feature::ElementAccessConstIndex {
                                        this: this.feature(),
                                        index,
                                    },
                                ),
                            })
                        })
                        .collect(),
                )
            }
            VisualizerVariant::CyclicSlice { ty } => todo!(),
            VisualizerVariant::Custom { ref stmts } => {
                let visual_feature = self.db.compile_time().visual_feature_repr(this);
                match self.eval_feature_repr(&visual_feature) {
                    Ok(value) => value.any_ref().downcast_ref::<VisualData>().clone(),
                    Err(_) => todo!(),
                }
            }
            VisualizerVariant::Todo => todo!(),
        }
        // let visualizer = self.compile_time().visualizer(ty);
        // visualizer.visualize(self.compile_time(), value, self.verbose())
    }
}
