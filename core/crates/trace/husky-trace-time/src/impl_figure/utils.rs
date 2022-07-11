use husky_entity_route::EntityRoutePtr;
use husky_text::TextRange;

use crate::*;

impl HuskyTraceTime {
    pub fn visualize_temp_value(
        &self,
        value: &__EvalValue<'static>,
        ty: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> __EvalResult<VisualData> {
        let eval_time = self.eval_time();
        let sample_id = self.restriction.opt_sample_id().unwrap();
        let feature = self
            .eval_time()
            .feature_interner()
            .intern(Feature::new_temp());
        let value = eval_time
            .session()
            .dev()
            .cache_temp_value(feature, sample_id, value);
        eval_time.visualize_feature(
            FeatureRepr::Value {
                value,
                ty,
                file,
                range,
                feature,
            },
            sample_id,
        )
    }

    pub fn visualize_control(&self, control: &ControlSnapshot) -> FigureCanvasData {
        // self.eval_time().visualize_feature(this, sample_id)
        match control {
            ControlSnapshot::None => FigureCanvasData::void(),
            ControlSnapshot::Return(_) => todo!(),
            ControlSnapshot::Break => todo!(),
            ControlSnapshot::Err(e) => e.clone().into(),
        }
    }
}
