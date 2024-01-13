use husky_value_protocol::presentation::ValuePresentationSynchrotronAction;

use super::*;

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceSynchrotronAction<TraceProtocol: IsTraceProtocol> {
    NewTrace(TraceSynchrotronNewTrace),
    ToggleExpansion(TraceSynchrotronToggleExpansion),
    SetSubtraces(TraceSynchrotronSetSubtraces),
    Phantom(TraceSynchrotronActionTraceProtocol<TraceProtocol>),
    ToggleAssociatedTrace {
        trace_id: TraceId,
        associated_trace_id: TraceId,
    },
    CacheStalk {
        pedestal: <TraceProtocol as IsTraceProtocol>::Pedestal,
        trace_id: TraceId,
        stalk: TraceStalk,
    },
    ValuePresentation(ValuePresentationSynchrotronAction),
}

pub trait IsTraceSynchrotronAction<TraceProtocol>:
    Into<TraceSynchrotronAction<TraceProtocol>>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome;

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome;
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol>
    for TraceSynchrotronAction<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        match self {
            TraceSynchrotronAction::NewTrace(action) => action.act(center),
            TraceSynchrotronAction::ToggleExpansion(action) => action.act(center),
            TraceSynchrotronAction::SetSubtraces(action) => action.act(center),
            TraceSynchrotronAction::Phantom(action) => action.act(center),
            &TraceSynchrotronAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => {
                center.entries[trace_id]
                    .1
                    .toggle_associated_traces(associated_trace_id);
            }
            TraceSynchrotronAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => todo!(),
            &TraceSynchrotronAction::CacheStalk {
                pedestal,
                trace_id,
                ref stalk,
            } => {
                let trace_entry = &mut center[trace_id];
                trace_entry.cache_stalk(pedestal, stalk.clone())
            }
            TraceSynchrotronAction::CacheStalk {
                pedestal,
                trace_id,
                stalk,
            } => todo!(),
            TraceSynchrotronAction::ValuePresentation(_) => todo!(),
        }
    }
}

impl<TraceProtocol> TraceSynchrotron<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    pub(crate) fn take_action<A: IsTraceSynchrotronAction<TraceProtocol>>(
        &mut self,
        action: A,
    ) -> A::Outcome {
        let outcome = action.act(self);
        self.actions.push(action.into());
        outcome
    }

    pub(crate) fn take_actions(
        &mut self,
        actions: impl IntoIterator<Item = TraceSynchrotronAction<TraceProtocol>>,
    ) {
        for action in actions {
            self.take_action(action)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronNewTrace {
    trace_id: TraceId,
    view_data: TraceViewData,
}

impl TraceSynchrotronNewTrace {
    pub fn new(trace_id: TraceId, view_data: TraceViewData) -> Self {
        Self {
            trace_id,
            view_data,
        }
    }
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol> for TraceSynchrotronNewTrace
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        center
            .entries
            .insert_new((
                self.trace_id,
                TraceSynchrotronEntry::new(self.view_data.clone()),
            ))
            .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronActionTraceProtocol<TraceProtocol> {
    v: TraceProtocol,
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol>
    for TraceSynchrotronActionTraceProtocol<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, _center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronToggleExpansion {
    trace_id: TraceId,
}

impl TraceSynchrotronToggleExpansion {
    pub fn new(trace_id: TraceId) -> Self {
        Self { trace_id }
    }
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol> for TraceSynchrotronToggleExpansion
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        center[self.trace_id].toggle_expansion()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronSetSubtraces {
    trace_id: TraceId,
    subtrace_ids: Vec<TraceId>,
}

impl TraceSynchrotronSetSubtraces {
    pub fn new(trace_id: TraceId, subtrace_ids: Vec<TraceId>) -> Self {
        Self {
            trace_id,
            subtrace_ids,
        }
    }
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol> for TraceSynchrotronSetSubtraces
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        center[self.trace_id].set_subtraces(self.subtrace_ids.clone())
    }
}
