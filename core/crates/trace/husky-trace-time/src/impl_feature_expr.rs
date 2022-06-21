use super::impl_expr::ExprTokenConfig;
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
    ) -> Vec<TraceLineRawData> {
        vec![TraceLineRawData {
            indent: 0,
            idx: 0,
            tokens: self.feature_expr_tokens(expr, config),
        }]
    }

    pub(crate) fn feature_expr_tokens(
        &mut self,
        expr: &Arc<FeatureLazyExpr>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let opt_associated_trace_id = if config.associated {
            Some(self.new_trace(None, 0, TraceVariant::FeatureExpr(expr.clone())))
        } else {
            None
        };
        return match expr.variant {
            FeatureLazyExprVariant::PrimitiveLiteral(value) => vec![literal!(value)],
            FeatureLazyExprVariant::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(lopd, config.subexpr()));
                tokens.push(special!(opr.spaced_code(), opt_associated_trace_id));
                tokens.extend(self.feature_expr_tokens(ropd, config.subexpr()));
                tokens
            }
            FeatureLazyExprVariant::Variable { varname, .. } => {
                vec![ident!(varname.0, opt_associated_trace_id)]
            }
            FeatureLazyExprVariant::RoutineCall {
                opds: ref feature_opds,
                ..
            } => match expr.expr.variant {
                LazyExprVariant::Opn { opn_kind, ref opds } => match opn_kind {
                    LazyOpnKind::Binary { opr, this } => todo!(),
                    LazyOpnKind::Prefix(_) => todo!(),
                    LazyOpnKind::FunctionRoutineCall(ranged_route) => self
                        .feature_routine_call_tokens(
                            expr.expr.file,
                            ranged_route,
                            feature_opds,
                            opt_associated_trace_id,
                            config,
                        ),
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::RecordCall(_) => todo!(),
                    LazyOpnKind::PatternCall => todo!(),
                    LazyOpnKind::FieldAccess { field_ident, .. } => todo!(),
                    LazyOpnKind::MethodCall {
                        method_ident,
                        method_route,
                        output_binding,
                    } => {
                        let mut tokens = vec![];
                        tokens.extend(self.feature_expr_tokens(&feature_opds[0], config.subexpr()));
                        tokens.push(special!("."));
                        tokens.push(ident!(method_ident.ident.0));
                        tokens.push(special!("("));
                        for i in 1..opds.len() {
                            if i > 1 {
                                tokens.push(special!(", "))
                            }
                            tokens.extend(
                                self.feature_expr_tokens(&feature_opds[i], config.subexpr()),
                            );
                        }
                        tokens.push(special!(")"));
                        tokens
                    }
                    LazyOpnKind::ElementAccess { .. } => todo!(),
                },
                _ => panic!(""),
            },
            FeatureLazyExprVariant::EnumKindLiteral { .. } => todo!(),
            FeatureLazyExprVariant::EntityFeature { .. } => {
                let text = self.runtime.compile_time().text(expr.expr.file).unwrap();
                vec![route!(
                    text.ranged(expr.expr.range),
                    opt_associated_trace_id
                )]
            }
            FeatureLazyExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureLazyExprVariant::ThisValue { ref repr } => todo!(),
            FeatureLazyExprVariant::EvalInput => vec![keyword!("input")],
            FeatureLazyExprVariant::PatternCall {} => todo!(),
            FeatureLazyExprVariant::ElementAccess { ref opds, .. } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(&opds[0], config.subexpr()));
                tokens.push(special!("[", opt_associated_trace_id.clone()));
                for i in 1..opds.len() {
                    let index_opd = &opds[i];
                    tokens.extend(self.feature_expr_tokens(index_opd, config.subexpr()));
                }
                tokens.push(special!("]", opt_associated_trace_id));
                tokens
            }
            FeatureLazyExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => self.field_access_tokens(config, this, field_ident),
        };
    }

    fn field_access_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
    ) -> Vec<TraceTokenData> {
        match this {
            FeatureRepr::Expr(this) => {
                let mut tokens = self.feature_expr_tokens(this, config);
                tokens.extend([special!("."), ident!(field_ident.ident.as_str())]);
                tokens
            }
            _ => vec![ident!(field_ident.ident.as_str())],
        }
    }

    fn feature_routine_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureLazyExpr>],
        opt_associated_trace_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let text = self.runtime.compile_time().text(file).unwrap();
        let mut tokens = vec![
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.feature_expr_tokens(input, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }

    pub(crate) fn feature_expr_figure(
        &self,
        expr: &Arc<FeatureLazyExpr>,
        attention: &Attention,
    ) -> Result<FigureCanvasData, (usize, VMRuntimeError)> {
        match attention {
            Attention::Specific { input_id } => {
                let value = self
                    .runtime
                    .eval_feature_expr(expr, *input_id)
                    .map_err(|e| (*input_id, e))?;
                Ok(FigureCanvasData::new_specific(
                    self.runtime
                        .visualize(FeatureRepr::Expr(expr.clone()), *input_id)
                        .unwrap(),
                ))
            }
            Attention::Generic { partitions, .. } => {
                let session = self.runtime.session();
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
                let visualizer = self.runtime.compile_time().visualizer(ty);
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
                                .process(label, || -> VMRuntimeResult<i32> {
                                    let visual_data = self
                                        .runtime
                                        .visualize(expr.clone().into(), labeled_data.sample_id)?;
                                    Ok(match visual_data {
                                        VisualData::Primitive {
                                            value: PrimitiveValueData::I32(i),
                                        } => i,
                                        _ => {
                                            p!(visual_data);
                                            panic!()
                                        }
                                    })
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
                                .process(label, || -> VMRuntimeResult<f32> {
                                    let visual_data = self
                                        .runtime
                                        .visualize(expr.clone().into(), labeled_data.sample_id)?;
                                    Ok(match visual_data {
                                        VisualData::Primitive {
                                            value: PrimitiveValueData::F32(f),
                                        } => f,
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
                                .process(label, || -> VMRuntimeResult<Graphics2dCanvasData> {
                                    let visual_data = self
                                        .runtime
                                        .visualize(expr.clone().into(), labeled_data.sample_id)?;
                                    Ok(Graphics2dCanvasData::from_visual_data(visual_data))
                                })
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
