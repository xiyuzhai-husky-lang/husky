use entity_route::RangedEntityRoute;
use text::Text;
use vm::{History, StackValueSnapshot};

use super::{impl_token::ExprTokenConfig, *};
use crate::*;

impl HuskyTraceTime {
    pub fn new_eager_expr_trace(
        &mut self,
        expr: Arc<EagerExpr>,
        history: Arc<History<'static>>,
        opt_parent: Option<&Trace>,
        indent: Indent,
    ) -> TraceId {
        self.new_trace(
            opt_parent.map(|parent| parent.id()),
            indent,
            TraceVariant::EagerExpr { expr, history },
        )
    }

    pub(crate) fn eager_expr_lines(
        &mut self,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'static>>,
        indent: u8,
        config: ExprTokenConfig,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent,
            idx: 0,
            tokens: self.eager_expr_tokens(expr, history, config),
        }]
    }

    pub(crate) fn eager_expr_tokens(
        &mut self,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'static>>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let associated_trace_id = if config.associated {
            Some(self.new_eager_expr_trace(expr.clone(), history.clone(), None, 0))
        } else {
            None
        };
        let mut tokens = vec![];
        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => {
                tokens.push(ident!(varname.0, associated_trace_id))
            }
            EagerExprVariant::EntityRoute { route: scope } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => return vec![literal!(value)],
            EagerExprVariant::Bracketed(ref expr) => {
                tokens.push(special!("("));
                tokens.extend(self.eager_expr_tokens(expr, history, config.subexpr()));
                tokens.push(special!(")"));
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, this_ty: this } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!(opr.spaced_code(), associated_trace_id));
                    tokens.extend(self.eager_expr_tokens(&opds[1], history, config.subexpr()));
                }
                EagerOpnVariant::Prefix { opr, .. } => {
                    tokens.push(special!(opr.code(), associated_trace_id));
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                }
                EagerOpnVariant::Suffix { opr, .. } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!(opr.code(), associated_trace_id));
                }
                EagerOpnVariant::RoutineCall(ranged_scope) => {
                    tokens = self.eager_routine_call_tokens(
                        expr.file,
                        *ranged_scope,
                        opds,
                        associated_trace_id,
                        history,
                        &config,
                    )
                }
                EagerOpnVariant::FieldAccess { field_ident, .. } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!("."));
                    tokens.push(ident!(field_ident.ident.0));
                }
                EagerOpnVariant::MethodCall { method_ident, .. } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!("."));
                    tokens.push(ident!(method_ident.ident.0));
                    tokens.push(special!("("));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(&opds[i], history, config.subexpr()));
                    }
                    tokens.push(special!(")"));
                }
                EagerOpnVariant::ElementAccess { element_binding } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!("[", associated_trace_id.clone()));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(&opds[i], history, config.subexpr()));
                    }
                    tokens.push(special!("]", associated_trace_id));
                }
                EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                    let text = self
                        .eval_time_singleton
                        .compile_time()
                        .text(expr.file)
                        .unwrap();
                    tokens.push(route!(text.ranged(ranged_ty.range)));
                    tokens.push(special!("("));
                    for i in 0..opds.len() {
                        if i > 0 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(&opds[i], history, config.subexpr()));
                    }
                    tokens.push(special!(")"));
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { .. } => todo!(),
            EagerExprVariant::ThisField { .. } => todo!(),
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
        };
        if config.appended {
            tokens.push(fade!(" = "));
            tokens.push(history.value(expr).into())
        }
        tokens
    }

    fn eager_routine_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<EagerExpr>],
        opt_associated_trace_id: Option<TraceId>,
        history: &Arc<History<'static>>,
        config: &ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let text = self.eval_time().compile_time().text(file).unwrap();
        let mut tokens = vec![
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.eager_expr_tokens(input, history, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }

    pub(crate) fn eager_expr_figure(
        &self,
        expr: &EagerExpr,
        history: &History<'static>,
    ) -> FigureCanvasData {
        if let Some(entry) = history.get(expr) {
            match entry {
                HistoryEntry::PureExpr { output } => match output {
                    Ok(output) => FigureCanvasData::new_specific(self.visualize_temp_value(
                        output,
                        expr.ty(),
                        expr.file,
                        expr.range,
                    )),
                    Err(e) => FigureCanvasData::void(),
                },
                HistoryEntry::Exec { .. } => todo!(),
                HistoryEntry::Loop { .. } => panic!(),
                HistoryEntry::ControlFlow {
                    opt_branch_entered: enter,
                    ..
                } => todo!(),
                HistoryEntry::Break => todo!(),
                HistoryEntry::PatternMatching { .. } => todo!(),
            }
        } else {
            FigureCanvasData::void()
        }
    }
}
