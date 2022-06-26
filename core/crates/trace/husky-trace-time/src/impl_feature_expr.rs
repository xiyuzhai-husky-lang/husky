use super::impl_token::ExprTokenConfig;
use crate::*;
use entity_route::RangedEntityRoute;
use eval_feature::EvalFeature;
use print_utils::epin;
use semantics_lazy::{LazyExprVariant, LazyOpnKind};
use text::RangedCustomIdentifier;
use visualizer_gen::VisualTy;
use vm::InterpreterQueryGroup;
use word::CustomIdentifier;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_lines(
        &mut self,
        expr: &Arc<FeatureLazyExpr>,
        config: ExprTokenConfig,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: 0,
            idx: 0,
            tokens: self.feature_expr_tokens(expr, config),
        }]
    }

    pub(crate) fn feature_expr_figure(
        &self,
        expr: &Arc<FeatureLazyExpr>,
        attention: &Attention,
    ) -> Result<FigureCanvasData, (SampleId, EvalError)> {
        match attention {
            Attention::Specific {
                sample_id: sample_id,
            } => {
                let value = self
                    .eval_time_singleton
                    .eval_feature_lazy_expr(expr, *sample_id)
                    .map_err(|e| (*sample_id, e))?;
                Ok(FigureCanvasData::new_specific(
                    self.eval_time_singleton
                        .visualize(FeatureRepr::Expr(expr.clone()), *sample_id)
                        .unwrap(),
                ))
            }
            Attention::Generic { partitions, .. } => {
                let session = self.eval_time_singleton.session();
                let dev_division = session.dev();
                assert_eq!(
                    partitions.last().unwrap().variant,
                    PartitionDefnDataVariant::Other
                );
                const COLUMN_NUMBER: u32 = 7;
                const COLUMN_HEIGHT: u32 = 5;
                let others_size: u32 = COLUMN_HEIGHT
                    * (COLUMN_NUMBER
                        - partitions
                            .iter()
                            .map(|partition| partition.ncol)
                            .sum::<u32>());
                let ty = expr.expr.ty();
                use visualizer_gen::VisualizerQueryGroup;
                let visualizer = self.eval_time_singleton.visualizer(ty);
                match visualizer.ty {
                    VisualTy::Void => {
                        p!(ty);
                        todo!()
                    }
                    VisualTy::Bool => todo!(),
                    VisualTy::B32 => todo!(),
                    VisualTy::B64 => todo!(),
                    VisualTy::I32 => {
                        let mut partitioned_samples_collector =
                            PartitionedSamplesCollector::<i32>::new(partitions.clone());
                        for labeled_data in dev_division.each_labeled_data() {
                            let label = labeled_data.label;
                            if partitioned_samples_collector
                                .process(label, || -> EvalResult<(SampleId, i32)> {
                                    let visual_data = self
                                        .eval_time_singleton
                                        .visualize(expr.clone().into(), labeled_data.sample_id)?;
                                    Ok((
                                        labeled_data.sample_id,
                                        match visual_data {
                                            VisualData::Primitive {
                                                value: PrimitiveValueData::I32(i),
                                            } => i,
                                            _ => {
                                                p!(visual_data);
                                                panic!()
                                            }
                                        },
                                    ))
                                })
                                .map_err(|e| (labeled_data.sample_id, e))?
                            {
                                break;
                            }
                        }
                        Ok(FigureCanvasData::GenericI32 {
                            partitioned_samples: partitioned_samples_collector.finish(),
                        })
                    }
                    VisualTy::F32 => {
                        let mut partitioned_samples_collector =
                            PartitionedSamplesCollector::<f32>::new(partitions.clone());
                        for labeled_data in dev_division.each_labeled_data() {
                            let label = labeled_data.label;
                            if partitioned_samples_collector
                                .process(label, || -> EvalResult<(SampleId, f32)> {
                                    let visual_data = self
                                        .eval_time_singleton
                                        .visualize(expr.clone().into(), labeled_data.sample_id)?;
                                    Ok(match visual_data {
                                        VisualData::Primitive {
                                            value: PrimitiveValueData::F32(f),
                                        } => (labeled_data.sample_id, f),
                                        _ => panic!(),
                                    })
                                })
                                .map_err(|e| (labeled_data.sample_id, e))?
                            {
                                break;
                            }
                        }
                        Ok(FigureCanvasData::GenericF32 {
                            partitioned_samples: partitioned_samples_collector.finish(),
                        })
                    }
                    VisualTy::Point2d => todo!(),
                    VisualTy::Shape2d
                    | VisualTy::Region2d
                    | VisualTy::Image2d
                    | VisualTy::Graphics2d => {
                        let mut partitioned_samples_collector =
                            PartitionedSamplesCollector::<Graphics2dCanvasData>::new(
                                partitions.clone(),
                            );
                        for labeled_data in dev_division.each_labeled_data() {
                            let label = labeled_data.label;
                            if partitioned_samples_collector
                                .process(
                                    label,
                                    || -> EvalResult<(SampleId, Graphics2dCanvasData)> {
                                        let visual_data = self.eval_time_singleton.visualize(
                                            expr.clone().into(),
                                            labeled_data.sample_id,
                                        )?;
                                        Ok((
                                            labeled_data.sample_id,
                                            Graphics2dCanvasData::from_visual_data(visual_data),
                                        ))
                                    },
                                )
                                .map_err(|e| (labeled_data.sample_id, e))?
                            {
                                break;
                            }
                        }
                        Ok(FigureCanvasData::GenericGraphics2d {
                            partitioned_samples: partitioned_samples_collector.finish(),
                        })
                    }
                }
            }
        }
    }
}
