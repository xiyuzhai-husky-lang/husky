use husky_vm_primitive_value::PrimitiveValueData;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_repr_figure(
        &self,
        repr: &FeatureRepr,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        if let Some(sample_id) = self.restriction.opt_sample_id() {
            let value = self
                .runtime_singleton
                .eval_feature_repr(repr, sample_id)
                .map_err(|e| (sample_id, e))?;
            Ok(FigureCanvasData::new_specific(
                self.runtime()
                    .visualize_feature(repr.clone(), sample_id)
                    .unwrap(),
            ))
        } else {
            self.feature_repr_generic_figure(repr)
        }
    }

    fn feature_repr_generic_figure(
        &self,
        repr: &FeatureRepr,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        assert_eq!(
            self.restriction.partitions().last().unwrap().variant,
            PartitionDefnDataVariant::Other
        );
        const COLUMN_NUMBER: u32 = 7;
        const COLUMN_HEIGHT: u32 = 5;
        let others_size: u32 = COLUMN_HEIGHT
            * (COLUMN_NUMBER
                - self
                    .restriction
                    .partitions()
                    .iter()
                    .map(|partition| partition.ncol)
                    .sum::<u32>());
        let ty = repr.ty();
        let visualizer = self.runtime().comptime().visualizer(ty);
        match visualizer.visual_ty {
            VisualTy::Void => {
                p!(ty);
                todo!()
            }
            VisualTy::Bool => todo!(),
            VisualTy::B32 => todo!(),
            VisualTy::B64 => todo!(),
            VisualTy::Integer => {
                let ref this = self;
                Ok(FigureCanvasData::GenericI32 {
                    partitioned_samples: this.feature_repr_partitioned_samples(
                        repr,
                        |visual_data| match visual_data {
                            VisualData::Primitive {
                                value: PrimitiveValueData::I32(i),
                            } => i,
                            _ => {
                                p!(visual_data);
                                panic!()
                            }
                        },
                    )?,
                })
            }
            VisualTy::Float => Ok(FigureCanvasData::GenericF32 {
                partitioned_samples: self.feature_repr_partitioned_samples(
                    repr,
                    |visual_data| match visual_data {
                        VisualData::Primitive {
                            value: PrimitiveValueData::F32(f),
                        } => f,
                        _ => panic!(),
                    },
                )?,
            }),
            VisualTy::Point2d => todo!(),
            VisualTy::Shape2d | VisualTy::Region2d | VisualTy::Image2d | VisualTy::Graphics2d => {
                Ok(FigureCanvasData::GenericGraphics2d {
                    partitioned_samples: self
                        .feature_repr_partitioned_samples(repr, |visual_data| {
                            Graphics2dCanvasData::from_visual_data(visual_data)
                        })?,
                })
            }
            VisualTy::Dataset => todo!(),
            VisualTy::Plot2d => todo!(),
            VisualTy::Any => todo!(),
            VisualTy::AnyGroup => todo!(),
        }
    }

    fn feature_repr_partitioned_samples<T>(
        &self,
        repr: &FeatureRepr,
        map: impl Fn(VisualData) -> T,
    ) -> Result<Vec<(PartitionDefnData, Vec<(SampleId, T)>)>, (SampleId, __VMError)> {
        let session = self.runtime().session();
        let dev_division = session.dev();
        let restriction = &self.restriction;
        let mut sampler = PartitionedSampler::<T>::new(restriction);
        for labeled_data in dev_division.each_labeled_data() {
            let label = labeled_data.label;
            let sample_id = labeled_data.sample_id;
            let f = |e| {
                todo!()
                //  (sample_id, e)
            };
            if !self
                .all_arrived(restriction.arrivals(), sample_id)
                .map_err(f)?
            {
                continue;
            }
            for trace_id in restriction.enters().iter() {
                todo!()
            }
            if sampler
                .process(label, || {
                    let visual_data = self.runtime().visualize_feature(repr.clone(), sample_id)?;
                    Ok((labeled_data.sample_id, map(visual_data)))
                })
                .map_err(f)?
            {
                break;
            }
        }
        Ok(sampler.finish())
    }

    fn all_arrived(&self, arrivals: &[TraceId], sample_id: SampleId) -> __VMResult<bool> {
        let mut all_arrived = true;
        for trace_id in arrivals.iter() {
            if !self.is_trace_arrived(*trace_id, sample_id)? {
                all_arrived = false;
                break;
            }
        }
        Ok(all_arrived)
    }

    fn is_trace_arrived(&self, trace_id: TraceId, sample_id: SampleId) -> __VMResult<bool> {
        let trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(_) => todo!(),
            TraceVariant::Module { .. } => todo!(),
            TraceVariant::EntityFeature { .. } => todo!(),
            TraceVariant::FeatureStmt(ref stmt) => self
                .runtime()
                .eval_opt_arrival_indicator(stmt.opt_arrival_indicator.as_ref(), sample_id),
            TraceVariant::FeatureBranch(ref branch) => self
                .runtime()
                .eval_opt_arrival_indicator(branch.opt_arrival_indicator.as_ref(), sample_id),
            TraceVariant::FeatureExpr(_) => todo!(),
            TraceVariant::FeatureCallArgument { .. } => todo!(),
            TraceVariant::FuncStmt { .. } => todo!(),
            TraceVariant::ProcStmt { .. } => todo!(),
            TraceVariant::ProcBranch { .. } => todo!(),
            TraceVariant::FuncBranch { .. } => todo!(),
            TraceVariant::LoopFrame { .. } => todo!(),
            TraceVariant::EagerExpr { .. } => todo!(),
            TraceVariant::EagerCallArgument { .. } => todo!(),
            TraceVariant::CallHead { .. } => todo!(),
        }
    }
}
