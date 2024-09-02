use crate::*;
use husky_devsoul::helpers::DevsoulKiControlFlow;
use husky_trace::trace::TraceData;

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn eval_trace_at_pedestal(
        &self,
        trace: Trace,
        pedestal: &Devsoul::Pedestal,
    ) -> DevsoulStaticVarResult<Devsoul, Option<DevsoulKiControlFlow<Devsoul>>> {
        self.runtime
            .with_pedestal(pedestal, |_| self.eval_trace(trace))
    }

    /// assuming that vars have been set up properly
    pub fn eval_trace(&self, trace: Trace) -> Option<DevsoulKiControlFlow<Devsoul>> {
        let db = self.db();
        match trace.data(db) {
            TraceData::Submodule(_) => None,
            TraceData::Val(data) => Some(self.runtime.eval_ki_repr(data.ki_repr(db))),
            TraceData::StaticVar(data) => Some(self.runtime.eval_ki_repr(data.ki_repr(db))),
            TraceData::LazyCallInput(data) => Some(self.runtime.eval_ki_repr(data.ki_repr(db))),
            TraceData::LazyCall(data) => Some(self.runtime.eval_ki_repr(data.ki_repr(db))),
            TraceData::LazyExpr(data) => Some(self.runtime.eval_ki_repr(data.ki_repr(trace, db)?)),
            TraceData::LazyPattern(_) => todo!(),
            TraceData::LazyStmt(data) => Some(self.runtime.eval_ki_repr(data.ki_repr(trace, db)?)),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPattern(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
            TraceData::Place(_) => todo!(),
            TraceData::Script(_) => todo!(),
        }
    }

    pub fn trace_visual(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
        visual_synchrotron: &mut VisualSynchrotron,
        trace_visual_cache: &mut TraceVisualCache<Devsoul::Pedestal>,
    ) -> Option<Visual> {
        use husky_value_interface::IsValue;

        let trace_id = trace.into();
        // TODO panic if `self.eval_trace(trace)` is `None`
        match self.eval_trace(trace)? {
            KiControlFlow::Continue(value) => {
                Some(trace_visual_cache.visual(trace_id, pedestal, || {
                    let visual = value.visualize(visual_synchrotron);
                    let plot_class = visual.plot_class(visual_synchrotron);
                    (visual, plot_class)
                }))
            }
            KiControlFlow::LoopContinue => todo!(),
            KiControlFlow::LoopExit(_) => todo!(),
            KiControlFlow::Return(_) => todo!(),
            KiControlFlow::Undefined => None,
            KiControlFlow::Throw(_) => Some(Visual::Error),
        }
        // match trace.ki_repr(db) {
        //     Some(ki_repr) => runtime
        //         .trace_ki_repr_visual(
        //             trace_id,
        //             ki_repr,
        //             pedestal,
        //             visual_synchrotron,
        //             trace_visual_cache,
        //         )
        //         .map(|visual| (trace_id, visual)),
        //     None => todo!(),
        // }
    }
}
