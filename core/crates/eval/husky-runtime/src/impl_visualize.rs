use crate::*;
use feature_gen::FeatureRepr;
use visualizer_gen::VisualizerVariant;
use vm::MemberValue;

impl HuskyRuntime {
    pub fn visualize<'temp, 'eval>(&self, this: FeatureRepr, input_id: usize) -> VisualData {
        let visualizer = self.compile_time.visualizer(this.ty());
        match visualizer.variant {
            VisualizerVariant::Compiled { call } => todo!(),
            VisualizerVariant::Vec { ty } => {
                let value = self.eval_feature_repr_cached(&this, input_id).unwrap();
                let elem_ty = ty.spatial_arguments[0].take_entity_route();
                let elem_visualizer = self.compile_time.visualizer(elem_ty);
                let virtual_vec: &'eval Vec<MemberValue<'eval>> =
                    value.any_eval_ref().downcast_ref();
                VisualData::Group(
                    virtual_vec
                        .iter()
                        .map(|elem: &'eval MemberValue<'eval>| {
                            self.visualize(FeatureRepr::Value {}, input_id),
                            elem_visualizer.visualize(db, elem.any_ref(), self.verbose())
                        })
                        .collect(),
                )
            }
            VisualizerVariant::CyclicSlice { ty } => todo!(),
            VisualizerVariant::Custom { ref stmts } => {
                let visual_feature = self.compile_time.visual_feature_repr(this);
                match self.eval_feature_repr(&visual_feature, input_id) {
                    Ok(value) => todo!(),
                    Err(_) => todo!(),
                }
            }
            VisualizerVariant::Todo => todo!(),
        }
        // let visualizer = self.compile_time().visualizer(ty);
        // visualizer.visualize(self.compile_time(), value, self.verbose())
    }
}
