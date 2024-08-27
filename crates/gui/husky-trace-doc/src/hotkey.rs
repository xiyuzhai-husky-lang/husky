use husky_trace_protocol::{
    caryatid::IsCaryatid, protocol::IsTraceProtocol, synchrotron::TraceSynchrotron,
    view::action::TraceViewAction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceDocHotkeyAction {
    FillCaryatidWithTraceVarDeps,
}

impl TraceDocHotkeyAction {
    pub(crate) fn view_action<TraceProtocol>(
        self,
        number: Option<usize>,
        trace_synchrotron: Option<&TraceSynchrotron<TraceProtocol>>,
    ) -> Option<TraceViewAction<TraceProtocol>>
    where
        TraceProtocol: IsTraceProtocol,
    {
        match self {
            TraceDocHotkeyAction::FillCaryatidWithTraceVarDeps => {
                let trace_synchrotron = trace_synchrotron?;
                let trace_id = match number {
                    Some(number) => todo!(),
                    None => trace_synchrotron.followed()?,
                };
                let trace_var_deps = trace_synchrotron[trace_id].var_deps();
                let mut caryatid = trace_synchrotron
                    .caryatid()
                    .with_extra_var_deps(trace_var_deps);
                Some(TraceViewAction::SetCaryatid { caryatid })
            }
        }
    }
}
