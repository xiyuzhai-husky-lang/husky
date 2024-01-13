use super::*;

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceCenterAction<TraceProtocol: IsTraceProtocol> {
    NewTrace(TraceCenterNewTrace),
    ToggleExpansion(TraceCenterToggleExpansion),
    SetSubtraces(TraceCenterSetSubtraces),
    Phantom(TraceCenterActionTraceProtocol<TraceProtocol>),
    ToggleAssociatedTrace {
        trace_id: TraceId,
        associated_trace_id: TraceId,
    },
    CacheStalk {
        pedestal: <TraceProtocol as IsTraceProtocol>::Pedestal,
        trace_id: TraceId,
        stalk: TraceStalk,
    },
}

pub trait IsTraceCenterAction<TraceProtocol>: Into<TraceCenterAction<TraceProtocol>>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome;

    fn act(&self, center: &mut TraceCenter<TraceProtocol>) -> Self::Outcome;
}

impl<TraceProtocol> IsTraceCenterAction<TraceProtocol> for TraceCenterAction<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceCenter<TraceProtocol>) -> Self::Outcome {
        match self {
            TraceCenterAction::NewTrace(action) => action.act(center),
            TraceCenterAction::ToggleExpansion(action) => action.act(center),
            TraceCenterAction::SetSubtraces(action) => action.act(center),
            TraceCenterAction::Phantom(action) => action.act(center),
            &TraceCenterAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => {
                center.entries[trace_id]
                    .1
                    .toggle_associated_traces(associated_trace_id);
            }
            TraceCenterAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => todo!(),
            &TraceCenterAction::CacheStalk {
                pedestal,
                trace_id,
                ref stalk,
            } => {
                let trace_entry = &mut center[trace_id];
                trace_entry.cache_stalk(pedestal, stalk.clone())
            }
        }
    }
}

impl<TraceProtocol> TraceCenter<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    pub(crate) fn take_action<A: IsTraceCenterAction<TraceProtocol>>(
        &mut self,
        action: A,
    ) -> A::Outcome {
        let outcome = action.act(self);
        self.actions.push(action.into());
        outcome
    }

    pub(crate) fn take_actions(
        &mut self,
        actions: impl IntoIterator<Item = TraceCenterAction<TraceProtocol>>,
    ) {
        for action in actions {
            self.take_action(action)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCenterNewTrace {
    trace_id: TraceId,
    view_data: TraceViewData,
}

impl TraceCenterNewTrace {
    pub fn new(trace_id: TraceId, view_data: TraceViewData) -> Self {
        Self {
            trace_id,
            view_data,
        }
    }
}

impl<TraceProtocol> IsTraceCenterAction<TraceProtocol> for TraceCenterNewTrace
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceCenter<TraceProtocol>) -> Self::Outcome {
        center
            .entries
            .insert_new((self.trace_id, TraceCenterEntry::new(self.view_data.clone())))
            .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCenterActionTraceProtocol<TraceProtocol> {
    v: TraceProtocol,
}

impl<TraceProtocol> IsTraceCenterAction<TraceProtocol>
    for TraceCenterActionTraceProtocol<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, _center: &mut TraceCenter<TraceProtocol>) -> Self::Outcome {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCenterToggleExpansion {
    trace_id: TraceId,
}

impl TraceCenterToggleExpansion {
    pub fn new(trace_id: TraceId) -> Self {
        Self { trace_id }
    }
}

impl<TraceProtocol> IsTraceCenterAction<TraceProtocol> for TraceCenterToggleExpansion
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceCenter<TraceProtocol>) -> Self::Outcome {
        center[self.trace_id].toggle_expansion()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCenterSetSubtraces {
    trace_id: TraceId,
    subtrace_ids: Vec<TraceId>,
}

impl TraceCenterSetSubtraces {
    pub fn new(trace_id: TraceId, subtrace_ids: Vec<TraceId>) -> Self {
        Self {
            trace_id,
            subtrace_ids,
        }
    }
}

impl<TraceProtocol> IsTraceCenterAction<TraceProtocol> for TraceCenterSetSubtraces
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceCenter<TraceProtocol>) -> Self::Outcome {
        center[self.trace_id].set_subtraces(self.subtrace_ids.clone())
    }
}
