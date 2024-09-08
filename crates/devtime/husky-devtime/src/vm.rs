mod history;

use crate::*;
use husky_devsoul::helpers::DevsoulKiControlFlow;

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn eager_expr_trace_value(
        &self,
        biological_parent: Trace,
        pedestal: Devsoul::Pedestal,
    ) -> Option<DevsoulVmControlFlowFrozen<Devsoul>> {
        let db = self.db();
        let history = self.trace_history(biological_parent, pedestal);
        todo!()
    }
}
