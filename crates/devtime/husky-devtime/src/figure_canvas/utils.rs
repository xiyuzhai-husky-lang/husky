use husky_ethereal_term::EtherealTerm;
use husky_text::TextRange;

use crate::*;

impl Debugtime {
    pub fn visualize_temp_value(
        &self,
        value: &__Register<'static>,
        ty: EtherealTerm,
        file: DiffPath,
        range: TextRange,
    ) -> __VMResult<VisualData> {
        let eval_time = self.runtime();
        let sample_id = self.state.presentation().opt_sample_id().unwrap();
        let feature = self
            .runtime()
            .feature_interner()
            .intern(Feature::new_temp());
        let value = eval_time
            .session()
            .dev()
            .cache_temp_value(feature, sample_id, value);
        eval_time.visualize_feature(
            ValRepr::Value {
                value,
                ty,
                file,
                range,
                feature,
            },
            sample_id,
        )
    }

    pub fn visualize_control(&self, control: &ControlSnapshot) -> SpecificFigureCanvasData {
        // self.eval_time().visualize_feature(this, sample_id)
        match control {
            ControlSnapshot::None => Default::default(),
            ControlSnapshot::Return(_) => todo!(),
            ControlSnapshot::Break => todo!(),
            ControlSnapshot::Err(_e) => Default::default(),
        }
    }

    pub(crate) fn update_figure_canvases(&mut self) -> DebugtimeUpdateM<()> {
        if let Some(active_trace_id) = self.opt_active_trace_id() {
            self.update_figure_canvas(active_trace_id)?;
        }
        for pin in self.state.presentation().pins().to_vec().into_iter() {
            self.update_figure_canvas(pin)?;
        }
        DebugtimeUpdateM::Ok(())
    }
    fn update_figure_canvas(&mut self, trace_id: TraceId) -> DebugtimeUpdateM<()> {
        self.update_trace_generic_figure_canvas(trace_id)?;
        self.update_trace_specific_figure_canvas(trace_id)
    }

    fn update_trace_generic_figure_canvas(&mut self, trace_id: TraceId) -> DebugtimeUpdateM<()> {
        if let Some(key) = self.gen_generic_figure_canvas_key(trace_id) {
            // todo: clean all this trouble
            let f =
                |(sample_id, e): (SampleId, __VMError)| -> __VMError { (sample_id.0, e).into() };
            if !self.state.generic_figure_canvases.contains(&key) {
                self.state
                    .generic_figure_canvases
                    .insert_new(key, self.gen_trace_generic_figure(trace_id).map_err(f)?)?
            }
        }
        DebugtimeUpdateM::Ok(())
    }

    fn update_trace_specific_figure_canvas(&mut self, trace_id: TraceId) -> DebugtimeUpdateM<()> {
        if let Some(key) = self.gen_specific_figure_canvas_key(trace_id) {
            // todo: clean all this trouble
            let f =
                |(sample_id, e): (SampleId, __VMError)| -> __VMError { (sample_id.0, e).into() };
            if !self.state.specific_figure_canvases.contains(&key) {
                self.state
                    .specific_figure_canvases
                    .insert_new(key, self.gen_trace_specific_figure(trace_id).map_err(f)?)?
            }
        }
        DebugtimeUpdateM::Ok(())
    }

    fn gen_generic_figure_canvas_key(&self, trace_id: TraceId) -> Option<GenericFigureCanvasKey> {
        GenericFigureCanvasKey::from_trace_data(
            &self.trace(trace_id).raw_data,
            &self.state.presentation(),
        )
    }

    fn gen_specific_figure_canvas_key(&self, trace_id: TraceId) -> Option<SpecificFigureCanvasKey> {
        SpecificFigureCanvasKey::from_trace_data(
            &self.trace(trace_id).raw_data,
            self.state.presentation(),
        )
    }
}
