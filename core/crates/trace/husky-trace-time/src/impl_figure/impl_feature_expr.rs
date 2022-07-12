use husky_feature_eval::Division;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_figure(
        &self,
        expr: &Arc<FeatureExpr>,
    ) -> Result<FigureCanvasData, (SampleId, EvalError)> {
        if let Some(sample_id) = self.restriction.opt_sample_id() {
            let value = self
                .eval_time_singleton
                .husky_feature_eval_expr(expr, sample_id)
                .map_err(|e| (sample_id, e))?;
            Ok(FigureCanvasData::new_specific(
                self.eval_time()
                    .visualize_feature(FeatureRepr::Expr(expr.clone()), sample_id)
                    .unwrap(),
            ))
        } else {
            self.feature_expr_generic_figure(expr)
        }
    }

    fn feature_expr_generic_figure(
        &self,
        expr: &Arc<FeatureExpr>,
    ) -> Result<FigureCanvasData, (SampleId, EvalError)> {
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
        let ty = expr.expr.ty();
        let visualizer = self.eval_time().compile_time().visualizer(ty);
        match visualizer.visual_ty {
            VisualTy::Void => {
                p!(ty);
                todo!()
            }
            VisualTy::Bool => todo!(),
            VisualTy::B32 => todo!(),
            VisualTy::B64 => todo!(),
            VisualTy::I32 => {
                let ref this = self;
                Ok(FigureCanvasData::GenericI32 {
                    partitioned_samples: this.feature_expr_partitioned_samples(
                        expr,
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
            VisualTy::F32 => Ok(FigureCanvasData::GenericF32 {
                partitioned_samples: self.feature_expr_partitioned_samples(
                    expr,
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
                        .feature_expr_partitioned_samples(expr, |visual_data| {
                            Graphics2dCanvasData::from_visual_data(visual_data)
                        })?,
                })
            }
            VisualTy::Dataset => todo!(),
        }
    }

    fn feature_expr_partitioned_samples<T>(
        &self,
        expr: &Arc<FeatureExpr>,
        map: impl Fn(VisualData) -> T,
    ) -> Result<Vec<(PartitionDefnData, Vec<(SampleId, T)>)>, (SampleId, EvalError)> {
        let session = self.eval_time().session();
        let dev_division = session.dev();
        let restriction = &self.restriction;
        let mut partitioned_samples_collector = PartitionedSampler::<T>::new(restriction);
        for labeled_data in dev_division.each_labeled_data() {
            let label = labeled_data.label;
            p!(restriction.arrivals());
            for trace_id in restriction.arrivals().iter() {
                todo!()
            }
            for trace_id in restriction.enters().iter() {
                todo!()
            }
            if partitioned_samples_collector
                .process(label, || {
                    let visual_data = self
                        .eval_time()
                        .visualize_feature(expr.clone().into(), labeled_data.sample_id)?;
                    Ok((labeled_data.sample_id, map(visual_data)))
                })
                .map_err(|e| (labeled_data.sample_id, e))?
            {
                break;
            }
        }
        Ok(partitioned_samples_collector.finish())
    }
}
