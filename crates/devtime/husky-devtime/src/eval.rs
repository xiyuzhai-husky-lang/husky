use crate::*;

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn eval_trace(&self, trace: Trace, pedestal: Devsoul::Pedestal) {
        // match trace.ki_repr(db)
        todo!()
    }

    pub fn trace_visual(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
        visual_synchrotron: &mut VisualSynchrotron,
        trace_visual_cache: &mut TraceVisualCache<Devsoul::Pedestal>,
    ) -> Option<Visual> {
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
        todo!()
    }
}
