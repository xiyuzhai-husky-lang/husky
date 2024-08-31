use husky_item_path_interface::ItemPathIdInterface;
use husky_value_protocol::presentation::synchrotron::action::ValuePresentationSynchrotronActionsDiff;
use husky_visual_protocol::synchrotron::action::VisualSynchrotronActionsDiff;
use item_path::ItemPathPresentation;
use smallvec::SmallVec;
use var_id::VarIdPresentation;
use vec_like::SmallVecSet;

use super::*;

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceSynchrotronAction<TraceProtocol: IsTraceProtocol> {
    NewTrace(TraceSynchrotronActionNewTrace),
    ToggleExpansion(TraceSynchrotronActionToggleExpansion),
    SetSubtraces(TraceSynchrotronActionSetSubtraces),
    Phantom(TraceSynchrotronActionTraceProtocol<TraceProtocol>),
    ToggleAssocTrace {
        trace_id: TraceId,
        assoc_trace_id: TraceId,
    },
    FollowTrace {
        followed: TraceId,
    },
    CacheStalk {
        pedestal: TraceProtocol::Pedestal,
        trace_id: TraceId,
        stalk: TraceStalk,
    },
    CacheFigure {
        figure_key: TraceFigureKey<TraceProtocol>,
        figure: TraceProtocol::Figure,
    },
    CacheItemPathPresentation {
        item_path_id_interface: ItemPathIdInterface,
        item_path_presentation: ItemPathPresentation,
    },
    CacheVarIdPresentation {
        item_path_id_interface: ItemPathIdInterface,
        var_id: TraceVarId<TraceProtocol>,
        var_id_presentation: VarIdPresentation,
    },
    ToggleAccompany {
        trace_id: TraceId,
    },
    SetCaryatid {
        caryatid: TraceProtocol::Caryatid,
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
        match *self {
            TraceSynchrotronAction::NewTrace(ref action) => action.act(synchrotron),
            TraceSynchrotronAction::ToggleExpansion(ref action) => action.act(synchrotron),
            TraceSynchrotronAction::SetSubtraces(ref action) => action.act(synchrotron),
            TraceSynchrotronAction::Phantom(ref action) => action.act(synchrotron),
            TraceSynchrotronAction::ToggleAssocTrace {
                trace_id,
                assoc_trace_id,
            } => {
                synchrotron
                    .entries
                    .get_mut(&trace_id)
                    .unwrap()
                    .toggle_assoc_traces(assoc_trace_id);
            }
            TraceSynchrotronAction::FollowTrace { followed: trace_id } => {
                synchrotron.followed_trace_id = Some(trace_id)
            }
            TraceSynchrotronAction::CacheStalk {
                ref pedestal,
                trace_id,
                ref stalk,
            } => {
                let trace_entry = &mut synchrotron[trace_id];
                trace_entry.cache_stalk(pedestal.clone(), stalk.clone())
            }
            TraceSynchrotronAction::CacheFigure {
                figure_key: ref key,
                ref figure,
            } => synchrotron.cache_figure(key.clone(), figure.clone()),
            TraceSynchrotronAction::ToggleAccompany { trace_id } => {
                synchrotron.accompanyings.toggle(trace_id)
            }
            TraceSynchrotronAction::SetCaryatid { ref caryatid } => {
                synchrotron.caryatid = caryatid.clone()
            }
            TraceSynchrotronAction::CacheItemPathPresentation {
                item_path_id_interface,
                ref item_path_presentation,
            } => {
                assert!(synchrotron
                    .item_path_presentations
                    .insert(item_path_id_interface, item_path_presentation.clone())
                    .is_none());
            }
            TraceSynchrotronAction::CacheVarIdPresentation {
                item_path_id_interface,
                var_id,
                ref var_id_presentation,
            } => {
                assert!(synchrotron
                    .var_id_presentations
                    .insert(
                        (item_path_id_interface, var_id),
                        var_id_presentation.clone(),
                    )
                    .is_none());
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
pub struct TraceSynchrotronActionNewTrace {
    trace_id: TraceId,
    var_deps: SmallVec<[ItemPathIdInterface; 2]>,
    view_data: TraceViewData,
}

impl TraceSynchrotronActionNewTrace {
    pub fn new(
        trace_id: TraceId,
        var_deps: SmallVec<[ItemPathIdInterface; 2]>,
        view_data: TraceViewData,
    ) -> Self {
        Self {
            trace_id,
            var_deps,
            view_data,
        }
    }
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol> for TraceSynchrotronActionNewTrace
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        assert!(center
            .entries
            .insert(
                self.trace_id,
                TraceSynchrotronEntry::new(self.var_deps.clone(), self.view_data.clone()),
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
pub struct TraceSynchrotronActionToggleExpansion {
    trace_id: TraceId,
}

impl TraceSynchrotronActionToggleExpansion {
    pub fn new(trace_id: TraceId) -> Self {
        Self { trace_id }
    }
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol>
    for TraceSynchrotronActionToggleExpansion
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        center[self.trace_id].toggle_expansion()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronActionSetSubtraces {
    trace_id: TraceId,
    subtrace_ids: Vec<TraceId>,
}

impl TraceSynchrotronActionSetSubtraces {
    pub fn new(trace_id: TraceId, subtrace_ids: Vec<TraceId>) -> Self {
        Self {
            trace_id,
            subtrace_ids,
        }
    }
}

impl<TraceProtocol> IsTraceSynchrotronAction<TraceProtocol> for TraceSynchrotronActionSetSubtraces
where
    TraceProtocol: IsTraceProtocol,
{
    type Outcome = ();

    fn act(&self, center: &mut TraceSynchrotron<TraceProtocol>) -> Self::Outcome {
        center[self.trace_id].cache_subtraces(self.subtrace_ids.clone())
    }
}
