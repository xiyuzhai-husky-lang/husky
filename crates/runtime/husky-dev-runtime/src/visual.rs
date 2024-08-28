use husky_trace_protocol::{id::TraceId, server::TraceVisualCache};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};

use super::*;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn trace_ki_repr_visual(
        &self,
        trace_id: TraceId,
        ki_repr: KiRepr,
        pedestal: Devsoul::Pedestal,
        visual_synchrotron: &mut VisualSynchrotron,
        trace_visual_cache: &mut TraceVisualCache<Devsoul::Pedestal>,
    ) -> Option<Visual> {
        use husky_value_interface::IsValue;
        match self.eval_ki_repr(ki_repr) {
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
            KiControlFlow::Throw(_) => todo!(),
        }
    }
}
