use husky_entity_route::EntityRoutePtr;
use husky_text::TextRange;

use crate::*;

impl HuskyTraceTime {
    pub fn visualize_temp_value(
        &self,
        value: &__Register<'static>,
        ty: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> __VMResult<VisualData> {
        let eval_time = self.runtime();
        let sample_id = self.restriction.opt_sample_id().unwrap();
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
            None,
            sample_id,
        )
    }

    pub fn visualize_control(&self, control: &ControlSnapshot) -> FigureCanvasData {
        // self.eval_time().visualize_feature(this, sample_id)
        match control {
            ControlSnapshot::None => FigureCanvasData::void(),
            ControlSnapshot::Return(_) => todo!(),
            ControlSnapshot::Break => todo!(),
            ControlSnapshot::Err(e) => {
                todo!()
                // e.clone().into()
            }
        }
    }

    pub(crate) fn update_figure_canvases(
        &mut self,
    ) -> __VMResult<Vec<(FigureCanvasKey, FigureCanvasData)>> {
        let mut new_figure_canvases: Vec<(FigureCanvasKey, FigureCanvasData)> = vec![];
        if let Some(active_trace_id) = self.opt_active_trace_id {
            self.update_figure_canvas(active_trace_id, true, &mut new_figure_canvases)?;
            self.update_figure_canvas(active_trace_id, false, &mut new_figure_canvases)?;
        }
        for pin in self.pins.clone().into_iter() {
            self.update_figure_canvas(*pin, true, &mut new_figure_canvases)?;
            self.update_figure_canvas(*pin, false, &mut new_figure_canvases)?;
        }
        Ok(new_figure_canvases)
    }

    fn update_figure_canvas(
        &mut self,
        trace_id: TraceId,
        is_specific: bool,
        new_figure_canvases: &mut Vec<(FigureCanvasKey, FigureCanvasData)>,
    ) -> __VMResult<()> {
        let key: FigureCanvasKey = self.gen_figure_canvas_key(trace_id, is_specific);
        // todo: clean all this trouble
        let f = |(sample_id, e): (SampleId, __VMError)| -> __VMError {
            match e.variant {
                __VMErrorVariant::Normal => __VMError {
                    message: e.message,
                    variant: __VMErrorVariant::FromBatch {
                        sample_id: sample_id.0,
                    },
                },
                __VMErrorVariant::FromBatch { .. } => e,
            }
        };
        if !self.figure_canvases.contains(&key) {
            self.figure_canvases.insert_move(key.clone());
            new_figure_canvases.push((
                key,
                self.gen_figure_canvas_data(trace_id, is_specific)
                    .map_err(f)?,
            ))
        }
        Ok(())
    }

    fn gen_figure_canvas_key(&self, trace_id: TraceId, is_specific: bool) -> FigureCanvasKey {
        FigureCanvasKey::from_trace_data(
            &self.trace(trace_id).raw_data,
            &self.restriction,
            is_specific,
        )
    }
}
