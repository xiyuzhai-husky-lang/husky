use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_figure(
        &self,
        expr: &Arc<FeatureExpr>,
    ) -> Result<FigureCanvasData, (SampleId, EvalError)> {
        match self.attention {
            Attention::Specific { sample_id } => {
                let value = self
                    .eval_time_singleton
                    .husky_feature_eval_expr(expr, sample_id)
                    .map_err(|e| (sample_id, e))?;
                Ok(FigureCanvasData::new_specific(
                    self.eval_time()
                        .visualize_feature(FeatureRepr::Expr(expr.clone()), sample_id)
                        .unwrap(),
                ))
            }
            Attention::Generic { ref partitions, .. } => {
                let session = self.eval_time().session();
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
                        let mut partitioned_samples_collector =
                            PartitionedSamplesCollector::<i32>::new(partitions.clone());
                        for labeled_data in dev_division.each_labeled_data() {
                            let label = labeled_data.label;
                            if partitioned_samples_collector
                                .process(label, || -> __EvalResult<(SampleId, i32)> {
                                    let visual_data = self.eval_time().visualize_feature(
                                        expr.clone().into(),
                                        labeled_data.sample_id,
                                    )?;
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
                                .process(label, || -> __EvalResult<(SampleId, f32)> {
                                    let visual_data = self.eval_time_singleton.visualize_feature(
                                        expr.clone().into(),
                                        labeled_data.sample_id,
                                    )?;
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
                                    || -> __EvalResult<(SampleId, Graphics2dCanvasData)> {
                                        let visual_data = self.eval_time().visualize_feature(
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
                    VisualTy::Dataset => todo!(),
                }
            }
        }
    }
}
