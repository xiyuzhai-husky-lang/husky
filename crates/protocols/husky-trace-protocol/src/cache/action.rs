use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[enum_class::from_variants]
pub enum TraceCacheAction<VisualComponent> {
    NewTrace(TraceCacheNewTrace),
    ToggleExpansion(TraceCacheToggleExpansion),
    SetSubtraces(TraceCacheSetSubtraces),
    Phantom(TraceCacheActionVisualComponent<VisualComponent>),
}

pub trait IsTraceCacheAction<VisualComponent>: Into<TraceCacheAction<VisualComponent>>
where
    VisualComponent: IsVisualComponent,
{
    type Outcome;

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome;
}

impl<VisualComponent> IsTraceCacheAction<VisualComponent> for TraceCacheAction<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome {
        match self {
            TraceCacheAction::NewTrace(action) => action.act(cache),
            TraceCacheAction::ToggleExpansion(action) => action.act(cache),
            TraceCacheAction::SetSubtraces(action) => action.act(cache),
            TraceCacheAction::Phantom(action) => action.act(cache),
        }
    }
}

impl<VisualComponent> TraceCache<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    pub(crate) fn take_action<Action: IsTraceCacheAction<VisualComponent>>(
        &mut self,
        action: Action,
    ) -> Action::Outcome {
        let outcome = action.act(self);
        self.actions.push(action.into());
        outcome
    }

    pub(crate) fn take_actions(
        &mut self,
        actions: impl IntoIterator<Item = TraceCacheAction<VisualComponent>>,
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

impl<VisualComponent> IsTraceCacheAction<VisualComponent> for TraceCacheNewTrace
where
    VisualComponent: IsVisualComponent,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome {
        debug_assert_eq!(self.trace_id.index(), cache.entries.len());
        cache
            .entries
            .push(TraceCacheEntry::new(self.view_data.clone()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheActionVisualComponent<VisualComponent> {
    v: VisualComponent,
}

impl<VisualComponent> IsTraceCacheAction<VisualComponent>
    for TraceCacheActionVisualComponent<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome {
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

impl<VisualComponent> IsTraceCacheAction<VisualComponent> for TraceCacheToggleExpansion
where
    VisualComponent: IsVisualComponent,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome {
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

impl<VisualComponent> IsTraceCacheAction<VisualComponent> for TraceCacheSetSubtraces
where
    VisualComponent: IsVisualComponent,
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome {
        cache[self.trace_id].set_subtraces(self.subtrace_ids.clone())
    }
}
