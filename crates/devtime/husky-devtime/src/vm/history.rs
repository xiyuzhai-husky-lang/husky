use husky_trace::trace::TraceData;

use super::*;
use std::sync::Arc;

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn trace_history(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
    ) -> Arc<VmHistory<Devsoul::LinketImpl>> {
        let db = self.db();
        match trace.data(db) {
            TraceData::Val(_) => todo!(),
            TraceData::StaticVar(_) => todo!(),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPattern(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
            TraceData::Place(_) => todo!(),
            TraceData::Script(_) => todo!(),
            _ => unreachable!("trace = {:?}", trace.debug(db)),
        }
    }

    fn trace_history_aux(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
    ) -> Arc<VmHistory<Devsoul::LinketImpl>> {
        self.cache_trace_history(trace, pedestal)
    }

    fn cache_trace_history(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
    ) -> Arc<VmHistory<Devsoul::LinketImpl>> {
        let key = (trace, pedestal.clone());
        self.eager_trace_cache
            .entry(key)
            .or_insert_with(|| Arc::new(self.calc_trace_history(trace, pedestal).1))
            .value()
            .clone()
    }

    fn calc_trace_history(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
    ) -> (
        DevsoulVmControlFlowFrozen<Devsoul>,
        VmHistory<Devsoul::LinketImpl>,
    ) {
        let db = self.db();
        self.runtime
            .with_default_var_ids(
                trace.history_var_deps(db).unwrap().iter().copied(),
                pedestal.clone(),
                |pedestal, _| self.calc_trace_history_aux(trace, pedestal, db),
            )
            .unwrap_or_else(|_| todo!("Handle error case"))
    }

    fn calc_trace_history_aux(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
        db: &::salsa::Db,
    ) -> (
        DevsoulVmControlFlowFrozen<Devsoul>,
        VmHistory<Devsoul::LinketImpl>,
    ) {
        match trace.data(db) {
            TraceData::Val(_) => todo!(),
            TraceData::StaticVar(_) => todo!(),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPattern(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
            TraceData::Place(_) => todo!(),
            TraceData::Script(_) => todo!(),
            _ => unreachable!("trace = {:?}", trace.debug(db)),
        }
    }
}
