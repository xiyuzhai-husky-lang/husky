use entity_route::RangedEntityRoute;
use text::Text;
use vm::{History, StackValueSnapshot};

use super::{impl_tokens::ExprTokenConfig, *};
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
}
