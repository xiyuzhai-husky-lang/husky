use super::*;

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn eager_trace_history(
        &self,
        trace: Trace,
        pedestal: Devsoul::Pedestal,
    ) -> &VmHistory<Devsoul::LinketImpl> {
        todo!()
    }
}
