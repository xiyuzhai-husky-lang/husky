use husky_compile_time::utils::{self, __RegisterDowncastResult};
use husky_vm_primitive_value::PrimitiveValueData;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_repr_figure(
        &self,
        repr: &FeatureRepr,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        if let Some(sample_id) = self.restriction.opt_sample_id() {
            let value = self
                .runtime
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
        // const COLUMN_HEIGHT: u32 = 5;
        let ty = repr.ty();
        let visualizer = self.runtime().comptime().visualizer(ty);
        match visualizer.visual_ty {
            VisualTy::Void => Ok(FigureCanvasData::void()),
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
            VisualTy::Fp => todo!(),
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
        let mut sampler = PartitionedSampler::<T>::new(restriction.partitions());
        for labeled_data in dev_division.each_labeled_data() {
            let label = labeled_data.label;
            let sample_id = labeled_data.sample_id;
            let f = |e| (sample_id, e);
            if !self.all_arrived(restriction, sample_id).map_err(f)? {
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

    fn all_arrived(&self, restriction: &Restriction, sample_id: SampleId) -> __VMResult<bool> {
        let mut all_arrived = true;
        for (trace_id, arrival_refined_control) in restriction.arrivals().iter() {
            if !self.is_trace_arrived(*trace_id, sample_id)? {
                all_arrived = false;
                break;
            }
            if !self.is_trace_refined_control_ok(
                *trace_id,
                sample_id,
                restriction.partitions(),
                arrival_refined_control,
            )? {
                all_arrived = false;
                break;
            }
        }
        Ok(all_arrived)
    }

    fn is_trace_arrived(&self, trace_id: TraceId, sample_id: SampleId) -> __VMResult<bool> {
        let trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(_) => Ok(true),
            TraceVariant::Module { .. } => panic!(),
            TraceVariant::EntityFeature { .. } => Ok(true),
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

    fn is_trace_refined_control_ok(
        &self,
        trace_id: TraceId,
        sample_id: SampleId,
        partitions: &Partitions,
        arrival_refined_control: &ArrivalRefinedControl,
    ) -> __VMResult<bool> {
        let trace = self.trace(trace_id);
        if arrival_refined_control.strike_evil() {
            let (value, ty) = match trace.variant {
                TraceVariant::Main(ref repr) => {
                    (self.runtime.eval_feature_repr(repr, sample_id)?, repr.ty())
                }
                TraceVariant::EntityFeature { route, ref repr } => {
                    (self.runtime.eval_feature_repr(repr, sample_id)?, repr.ty())
                }
                TraceVariant::FeatureStmt(ref stmt) => (
                    self.runtime.eval_feature_stmt(stmt, sample_id)?,
                    stmt.output_ty,
                ),
                TraceVariant::FeatureBranch(_) => todo!(),
                TraceVariant::FeatureExpr(_) => todo!(),
                TraceVariant::FeatureCallArgument { name, ref argument } => todo!(),
                TraceVariant::FuncStmt {
                    ref stmt,
                    ref history,
                } => todo!(),
                TraceVariant::ProcStmt {
                    ref stmt,
                    ref history,
                } => todo!(),
                TraceVariant::ProcBranch { .. } => todo!(),
                TraceVariant::FuncBranch { .. } => todo!(),
                TraceVariant::LoopFrame { .. } => todo!(),
                TraceVariant::EagerExpr { .. } => todo!(),
                TraceVariant::EagerCallArgument { .. } => todo!(),
                TraceVariant::Module { .. } | TraceVariant::CallHead { .. } => panic!(),
            };
            assert!(ty == self.comptime().target_output_ty().unwrap());
            let label_downcast_result = self.comptime().register_to_label_converter()(&value);
            let true_label = self.runtime.session().dev().label(sample_id);
            match label_downcast_result {
                __RegisterDowncastResult::Value(predicted_label) => {
                    Ok(predicted_label != true_label)
                }
                __RegisterDowncastResult::None => {
                    if partitions.is_nondefault(true_label) {
                        todo!()
                    }
                    Ok(partitions.is_nondefault(true_label))
                }
                __RegisterDowncastResult::Unreturned => Ok(false),
            }
        } else {
            Ok(true)
        }
    }
}
