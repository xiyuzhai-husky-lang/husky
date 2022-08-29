use super::FeatureEvaluator;
use husky_check_utils::should_eq;
use husky_entity_semantics::{EntityDefnQueryGroup, VisualTy, Visualizer, VisualizerVariant};
use husky_feature_gen::*;
use husky_trace_protocol::VisualData;
use husky_vm::*;
use std::sync::Arc;

impl<'temp, 'eval> FeatureEvaluator<'temp, 'eval> {
    pub fn visualize_feature(&self, this: FeatureRepr) -> __VMResult<VisualData> {
        self.as_static().visualize_static(this)
    }
}
impl<'temp> FeatureEvaluator<'temp, 'static> {
    pub fn visualize_static(&self, this: FeatureRepr) -> __VMResult<VisualData> {
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
    }

    pub fn visualize_intrinsic(
        &self,
        this: FeatureRepr,
        visualizer: &Visualizer,
    ) -> __VMResult<VisualData> {
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
}
