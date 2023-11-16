use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[enum_class::from_variants]
pub enum TraceCacheAction<TraceProtocol> {
    NewTrace(TraceCacheNewTrace),
    ToggleExpansion(TraceCacheToggleExpansion),
    SetSubtraces(TraceCacheSetSubtraces),
    Phantom(TraceCacheActionTraceProtocol<TraceProtocol>),
    ToggleAssociatedTrace {
        trace_id: TraceId,
        associated_trace_id: TraceId,
    },
}

pub trait IsTraceCacheAction<TraceProtocol>: Into<TraceCacheAction<TraceProtocol>>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome;

    fn act(&self, cache: &mut TraceCache<TraceProtocol>) -> Self::Outcome;
}

impl<TraceProtocol> IsTraceCacheAction<TraceProtocol> for TraceCacheAction<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<TraceProtocol>) -> Self::Outcome {
        match self {
            TraceCacheAction::NewTrace(action) => action.act(cache),
            TraceCacheAction::ToggleExpansion(action) => action.act(cache),
            TraceCacheAction::SetSubtraces(action) => action.act(cache),
            TraceCacheAction::Phantom(action) => action.act(cache),
            &TraceCacheAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => {
                cache.entries[trace_id]
                    .1
                    .toggle_associated_traces(associated_trace_id);
            }
        }
    }
}

impl<TraceProtocol> TraceCache<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    pub(crate) fn take_action<A: IsTraceCacheAction<TraceProtocol>>(
        &mut self,
        cache_action: A,
    ) -> A::Outcome {
        let outcome = cache_action.act(self);
        self.actions.push(cache_action.into());
        outcome
    }

    pub(crate) fn take_actions(
        &mut self,
        actions: impl IntoIterator<Item = TraceCacheAction<TraceProtocol>>,
    ) {
        for action in actions {
            self.take_action(action)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheNewTrace {
    trace_id: TraceId,
    view_data: TraceViewData,
}

impl TraceCacheNewTrace {
    pub fn new(trace_id: TraceId, view_data: TraceViewData) -> Self {
        Self {
            trace_id,
            view_data,
        }
    }
}

impl<TraceProtocol> IsTraceCacheAction<TraceProtocol> for TraceCacheNewTrace
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<TraceProtocol>) -> Self::Outcome {
        cache
            .entries
            .insert_new((self.trace_id, TraceCacheEntry::new(self.view_data.clone())))
            .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheActionTraceProtocol<TraceProtocol> {
    v: TraceProtocol,
}

impl<TraceProtocol> IsTraceCacheAction<TraceProtocol>
    for TraceCacheActionTraceProtocol<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, _cache: &mut TraceCache<TraceProtocol>) -> Self::Outcome {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheToggleExpansion {
    trace_id: TraceId,
}

impl TraceCacheToggleExpansion {
    pub fn new(trace_id: TraceId) -> Self {
        Self { trace_id }
    }
}

impl<TraceProtocol> IsTraceCacheAction<TraceProtocol> for TraceCacheToggleExpansion
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<TraceProtocol>) -> Self::Outcome {
        cache[self.trace_id].toggle_expansion()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheSetSubtraces {
    trace_id: TraceId,
    subtrace_ids: Vec<TraceId>,
}

impl TraceCacheSetSubtraces {
    pub fn new(trace_id: TraceId, subtrace_ids: Vec<TraceId>) -> Self {
        Self {
            trace_id,
            subtrace_ids,
        }
    }
}

impl<TraceProtocol> IsTraceCacheAction<TraceProtocol> for TraceCacheSetSubtraces
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<TraceProtocol>) -> Self::Outcome {
        cache[self.trace_id].set_subtraces(self.subtrace_ids.clone())
    }
}
