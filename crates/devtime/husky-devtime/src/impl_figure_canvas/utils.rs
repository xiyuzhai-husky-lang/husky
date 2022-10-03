use husky_entity_route::EntityRoutePtr;
use husky_text::TextRange;

use crate::*;

impl HuskyDevtime {
    pub fn visualize_temp_value(
        &self,
        value: &__Register<'static>,
        ty: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> __VMResult<VisualData> {
        let eval_time = self.runtime();
        let sample_id = self.state.presentation.opt_sample_id().unwrap();
        let feature = self
            .runtime()
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
            ControlSnapshot::Err(e) => FigureCanvasData::void(),
        }
    }

    pub(crate) fn update_figure_canvases(&mut self) -> HuskyDevtimeUpdateM<()> {
        if let Some(active_trace_id) = *self.state.opt_active_trace_id {
            self.update_figure_canvas(active_trace_id, true)?;
            self.update_figure_canvas(active_trace_id, false)?;
        }
        for pin in self.state.pins.clone().into_iter() {
            self.update_figure_canvas(*pin, true)?;
            self.update_figure_canvas(*pin, false)?;
        }
        HuskyDevtimeUpdateM::Ok(())
    }

    fn update_figure_canvas(
        &mut self,
        trace_id: TraceId,
        is_specific: bool,
    ) -> HuskyDevtimeUpdateM<()> {
        let key: FigureCanvasKey = self.gen_figure_canvas_key(trace_id, is_specific);
        // todo: clean all this trouble
        let f = |(sample_id, e): (SampleId, __VMError)| -> __VMError { (sample_id.0, e).into() };
        if !self.state.figure_canvases.contains(&key) {
            self.state.figure_canvases.insert_new(
                key.clone(),
                self.gen_figure_canvas_data(trace_id, is_specific)
                    .map_err(f)?,
            )?
        }
        HuskyDevtimeUpdateM::Ok(())
    }

    fn gen_figure_canvas_key(&self, trace_id: TraceId, is_specific: bool) -> FigureCanvasKey {
        FigureCanvasKey::from_trace_data(
            &self.trace(trace_id).raw_data,
            &self.state.presentation,
            is_specific,
        )
    }
}
