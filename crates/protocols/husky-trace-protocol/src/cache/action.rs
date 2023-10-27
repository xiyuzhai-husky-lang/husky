use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[enum_class::from_variants]
pub enum TraceCacheAction<VisualComponent> {
    NewTrace(TraceCacheActionNewTrace),
    Phantom(TraceCacheActionVisualComponent<VisualComponent>),
}

pub trait IsTraceCacheAction<VisualComponent>: Into<TraceCacheAction<VisualComponent>> {
    type Outcome;

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome;
}

impl<VisualComponent> IsTraceCacheAction<VisualComponent> for TraceCacheAction<VisualComponent> {
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome {
        match self {
            TraceCacheAction::NewTrace(action) => action.act(cache),
            TraceCacheAction::Phantom(action) => action.act(cache),
        }
    }
}

impl<VisualComponent> TraceCache<VisualComponent> {
    pub(crate) fn take_action<Action: IsTraceCacheAction<VisualComponent>>(
        &mut self,
        action: Action,
    ) -> Action::Outcome {
        let outcome = action.act(self);
        self.actions.push(action.into());
        outcome
    }

    pub(crate) fn take_actions(&mut self, actions: Vec<TraceCacheAction<VisualComponent>>) {
        for action in actions {
            self.take_action(action)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheActionNewTrace {
    trace_id: TraceId,
    view_data: TraceViewData,
}

impl<VisualComponent> IsTraceCacheAction<VisualComponent> for TraceCacheActionNewTrace {
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
{
    type Outcome = ();

    fn act(&self, cache: &mut TraceCache<VisualComponent>) -> Self::Outcome {
        todo!()
    }
}
