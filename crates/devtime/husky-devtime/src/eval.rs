use crate::*;
use either::*;
use husky_devsoul::helpers::{DevsoulKiControlFlow, DevsoulVmControlFlow};
use husky_trace::trace::TraceData;

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn eval_trace_at_pedestal(
        &self,
        trace: Trace,
        pedestal: &Devsoul::Pedestal,
    ) -> DevsoulStaticVarResult<
        Devsoul,
        Option<Either<DevsoulKiControlFlow<Devsoul>, Option<DevsoulVmControlFlowFrozen<Devsoul>>>>,
    > {
        self.runtime
            .with_pedestal(pedestal, |_| self.eval_trace(trace, pedestal))
    }

    /// assuming that vars have been set up in the binary side properly
    ///
    /// returns None means the trace doesn't have an interesting value
    pub fn eval_trace(
        &self,
        trace: Trace,
        pedestal: &Devsoul::Pedestal,
    ) -> Option<Either<DevsoulKiControlFlow<Devsoul>, Option<DevsoulVmControlFlowFrozen<Devsoul>>>>
    {
        let db = self.db();
        match trace.data(db) {
            TraceData::Submodule(_) => None,
            TraceData::Val(data) => Some(Left(self.runtime.eval_ki_repr(data.ki_repr(db)))),
            TraceData::StaticVar(data) => Some(Left(self.runtime.eval_ki_repr(data.ki_repr(db)))),
            TraceData::LazyCallInput(data) => {
                Some(Left(self.runtime.eval_ki_repr(data.ki_repr(db))))
            }
            TraceData::LazyCall(data) => Some(Left(self.runtime.eval_ki_repr(data.ki_repr(db)))),
            TraceData::LazyExpr(data) => {
                Some(Left(self.runtime.eval_ki_repr(data.ki_repr(trace, db)?)))
            }
            TraceData::LazyPattern(_) => todo!(),
            TraceData::LazyStmt(data) => {
                Some(Left(self.runtime.eval_ki_repr(data.ki_repr(trace, db)?)))
            }
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(data) => {
                // think about it
                // Some(self.eager_expr_trace_value(data.biological_parent(), pedestal.clone()))
                todo!()
            }
            TraceData::EagerExpr(data) => {
                // think about it
                Some(Right(self.eager_expr_trace_value(
                    data.biological_parent(),
                    pedestal.clone(),
                )))
            }
            TraceData::EagerPattern(_) => todo!(),
            TraceData::EagerStmt(data) => {
                // think about it
                Some(Right(self.eager_expr_trace_value(
                    data.biological_parent(),
                    pedestal.clone(),
                )))
            }
            TraceData::Place(_) => todo!(),
            TraceData::Script(_) => todo!(),
            TraceData::LazyLoopFrame(_) => todo!(),
            TraceData::LazyLoopRange(_) => todo!(),
            TraceData::EagerLoopFrame(_) => todo!(),
            TraceData::EagerLoopRange(_) => todo!(),
        }
    }

    pub fn trace_visual(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
        visual_synchrotron: &mut VisualSynchrotron,
        trace_visual_cache: &mut TraceVisualCache<Devsoul::Pedestal>,
    ) -> Option<Visual> {
        use husky_value::IsValue;

        let trace_id = trace.into();
        // TODO panic if `self.eval_trace(trace)` is `None`
        match self.eval_trace(trace, &pedestal)? {
            Left(kcf) => match kcf {
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
            },
            Right(vcf) => todo!(),
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
