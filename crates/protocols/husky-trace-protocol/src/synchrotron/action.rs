use husky_value_protocol::presentation::synchrotron::action::ValuePresentationSynchrotronActionsDiff;
use husky_visual_protocol::synchrotron::action::VisualSynchrotronActionsDiff;

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
    FollowTrace {
        trace_id: TraceId,
    },
    CacheStalk {
        pedestal: TraceProtocol::Pedestal,
        trace_id: TraceId,
        stalk: TraceStalk,
    },
    CacheFigure {
        followed_trace_id: Option<TraceId>,
        accompanying_trace_ids_except_followed: AccompanyingTraceIdsExceptFollowed,
        pedestal: TraceProtocol::Pedestal,
        figure: TraceProtocol::Figure,
    },
    ToggleAccompany {
        trace_id: TraceId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronActionsDiff<TraceProtocol: IsTraceProtocol> {
    new_actions: smallvec::SmallVec<[TraceSynchrotronAction<TraceProtocol>; 3]>,
    value_presentation_actions_diff: ValuePresentationSynchrotronActionsDiff,
    visual_actions_diff: VisualSynchrotronActionsDiff,
}

impl<TraceProtocol: IsTraceProtocol> TraceSynchrotronActionsDiff<TraceProtocol> {
    pub fn new(
        new_actions: smallvec::SmallVec<[TraceSynchrotronAction<TraceProtocol>; 3]>,
        value_presentation_actions_diff: ValuePresentationSynchrotronActionsDiff,
        visual_actions_diff: VisualSynchrotronActionsDiff,
    ) -> Self {
        Self {
            new_actions,
            value_presentation_actions_diff,
            visual_actions_diff,
        }
    }
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

    fn act(&self, synchrotron: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        match self {
            TraceSynchrotronAction::NewTrace(action) => action.act(synchrotron),
            TraceSynchrotronAction::ToggleExpansion(action) => action.act(synchrotron),
            TraceSynchrotronAction::SetSubtraces(action) => action.act(synchrotron),
            TraceSynchrotronAction::Phantom(action) => action.act(synchrotron),
            &TraceSynchrotronAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => {
                synchrotron
                    .entries
                    .get_mut(&trace_id)
                    .unwrap()
                    .toggle_associated_traces(associated_trace_id);
            }
            TraceSynchrotronAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => todo!(),
            &TraceSynchrotronAction::FollowTrace { trace_id } => {
                synchrotron.followed_trace_id = Some(trace_id)
            }
            &TraceSynchrotronAction::CacheStalk {
                pedestal,
                trace_id,
                ref stalk,
            } => {
                let trace_entry = &mut synchrotron[trace_id];
                trace_entry.cache_stalk(pedestal, stalk.clone())
            }
            &TraceSynchrotronAction::CacheFigure {
                pedestal,
                followed_trace_id,
                accompanying_trace_ids_except_followed: ref accompanying_trace_ids,
                ref figure,
            } => synchrotron.cache_figure(
                followed_trace_id,
                accompanying_trace_ids.clone(),
                pedestal,
                figure.clone(),
            ),
            TraceSynchrotronAction::CacheStalk {
                pedestal,
                trace_id,
                stalk,
            } => todo!(),
            TraceSynchrotronAction::CacheFigure {
                pedestal,
                followed_trace_id,
                accompanying_trace_ids_except_followed: accompanying_trace_ids,
                figure,
            } => todo!(),
            &TraceSynchrotronAction::ToggleAccompany { trace_id } => {
                synchrotron.accompanying_trace_ids.toggle(trace_id)
            }
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

    pub(crate) fn take_actions_diff(
        &mut self,
        actions_diff: TraceSynchrotronActionsDiff<TraceProtocol>,
    ) {
        for action in actions_diff.new_actions {
            self.take_action(action)
        }
        self.value_presentation_synchrotron
            .take_actions_diff(actions_diff.value_presentation_actions_diff);
        self.visual_synchrotron
            .take_actions_diff(actions_diff.visual_actions_diff)
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
        assert!(center
            .entries
            .insert(
                self.trace_id,
                TraceSynchrotronEntry::new(self.view_data.clone()),
            )
            .is_none())
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
        center[self.trace_id].cache_subtraces(self.subtrace_ids.clone())
    }
}
