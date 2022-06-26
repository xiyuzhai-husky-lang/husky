use entity_route::EntityRoutePtr;
use text::TextRange;

use crate::*;

impl HuskyTraceTime {
    pub fn visualize_temp_value(
        &self,
        value: &EvalValue<'static>,
        ty: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> VisualData {
        let eval_time = self.eval_time();
        let sample_id = self.attention.opt_sample_id().unwrap();
        let feature = self
            .eval_time()
            .feature_interner()
            .intern(Feature::new_temp());
        let value = eval_time
            .session()
            .dev()
            .cache_temp_value(feature, sample_id, value);
        eval_time
            .visualize(
                FeatureRepr::Value {
                    value,
                    ty,
                    file,
                    range,
                    feature,
                },
                sample_id,
            )
            .unwrap()
    }
}
