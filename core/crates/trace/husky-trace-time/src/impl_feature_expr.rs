use super::impl_expr::ExprTokenConfig;
use crate::*;
use entity_route::RangedEntityRoute;
use eval_feature::EvalFeature;
use print_utils::epin;
use semantics_lazy::{LazyExprVariant, LazyOpnKind};
use text::RangedCustomIdentifier;
use vm::InterpreterQueryGroup;
use word::CustomIdentifier;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_lines(
        &mut self,
        expr: &Arc<FeatureExpr>,
        config: ExprTokenConfig,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: 0,
            idx: 0,
            tokens: self.feature_expr_tokens(expr, config),
        }]
    }

    pub(crate) fn feature_expr_tokens(
        &mut self,
        expr: &Arc<FeatureExpr>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let opt_associated_trace_id = if config.associated {
            Some(self.new_trace(None, 0, TraceVariant::FeatureExpr(expr.clone())))
        } else {
            None
        };
        return match expr.variant {
            FeatureExprVariant::PrimitiveLiteral(value) => vec![literal!(value)],
            FeatureExprVariant::PrimitiveBinaryOpr {
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
            FeatureExprVariant::Variable { varname, .. } => {
                vec![ident!(varname.0, opt_associated_trace_id)]
            }
            FeatureExprVariant::RoutineCall {
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
                    LazyOpnKind::FieldAccess {
                        field_ident,
                        field_kind,
                        ..
                    } => todo!(),
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
            FeatureExprVariant::EnumKindLiteral { .. } => todo!(),
            FeatureExprVariant::EntityFeature { .. } => {
                let text = self.runtime.compile_time().text(expr.expr.file).unwrap();
                vec![route!(
                    text.ranged(expr.expr.range),
                    opt_associated_trace_id
                )]
            }
            FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureExprVariant::ThisValue { ref repr } => todo!(),
            FeatureExprVariant::EvalInput => vec![keyword!("input")],
            FeatureExprVariant::PatternCall {} => todo!(),
            FeatureExprVariant::ElementAccess { ref opds, .. } => {
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
            FeatureExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(config, this, field_ident),
            FeatureExprVariant::StructOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(config, this, field_ident),
            FeatureExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(config, this, field_ident),
            FeatureExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => self.field_access_tokens(config, this, field_ident),
        };
    }

    fn field_access_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
    ) -> Vec<TraceTokenData> {
        let mut tokens = self.feature_expr_tokens(this, config);
        tokens.extend([special!("."), ident!(field_ident.ident.as_str())]);
        tokens
    }

    fn feature_routine_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureExpr>],
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
        expr: &FeatureExpr,
        focus: &Focus,
    ) -> FigureCanvasData {
        match focus {
            Focus::Specific { input_id } => {
                if let Ok(value) = self.runtime.eval_feature_expr(expr, *input_id) {
                    FigureCanvasData::new_specific(
                        self.runtime.visualize(expr.expr.ty(), value.any_ref()),
                    )
                } else {
                    FigureCanvasData::error()
                }
            }
            Focus::Generic { partitions, .. } => {
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
                let mut partitioned_samples_collector =
                    PartitionedSamplesCollector::<Graphics2dCanvasData>::new(partitions.clone());
                for labeled_data in dev_division.each_labeled_data() {
                    let label = labeled_data.label;
                    if partitioned_samples_collector.process(label, || {
                        let value = self
                            .runtime
                            .eval_feature_expr(expr, labeled_data.input_id)
                            .unwrap();
                        let visual_data = self.runtime.visualize(expr.expr.ty(), value.any_ref());
                        match FigureCanvasData::new_specific(visual_data) {
                            FigureCanvasData::Graphics2d { graphics2d_data } => graphics2d_data,
                            _ => panic!(),
                        }
                    }) {
                        break;
                    }
                }
                FigureCanvasData::GenericGraphics2d {
                    partitioned_samples: partitioned_samples_collector.finish(),
                }
            }
        }
    }
}
