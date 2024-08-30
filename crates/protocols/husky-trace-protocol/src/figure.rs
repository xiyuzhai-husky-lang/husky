use crate::{
    accompany::AccompanyingTraceIdsExceptFollowed, anchor::Anchor, caryatid::IsCaryatid,
    chart::Chart, server::TracePlotInfos, IsTraceProtocol, TraceId, TraceSynchrotron,
};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::KiReprInterface;
use husky_linket_impl::{
    pedestal::{IsPedestal, IsPedestalFull},
    var_id::{IsVarId, IsVarIdFull},
};
use husky_visual_protocol::{
    plot::PlotClass,
    synchrotron::VisualSynchrotron,
    visual::{image::ImageVisual, CompositeVisual, Visual},
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use ui::{
    ui::{IsUi, UiTextureId},
    visual::cache::VisualUiCache,
};
use vec_like::{ordered_small_vec_map::OrderedSmallVecPairMap, OrderedSmallVecSet, SmallVecSet};

/// `IsFigure` extends `Serialize` and `Deserialize` for the convenience of deriving `Serialize` and `Deserialize` for generic types
///
/// for example TraceSynchrotron
pub trait IsFigure:
    std::fmt::Debug + PartialEq + Eq + Clone + Serialize + for<'a> Deserialize<'a> + Send + 'static
{
    type Pedestal: IsPedestal;

    fn from_chart(
        chart: Option<Chart<<Self::Pedestal as IsPedestal>::VarId, CompositeVisual<TraceId>>>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self;
}

pub trait FigureUi<Ui: IsUi> {
    fn ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut VisualUiCache<Ui>,
        ui: &mut Ui,
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct FigureKey<VarId: IsVarId> {
    followed_reduced: Option<TraceId>,
    accompanyings_except_followed_reduced: OrderedSmallVecSet<TraceId, 4>,
    joint_static_var_anchors: OrderedSmallVecPairMap<ItemPathIdInterface, Anchor<VarId>, 4>,
}

pub type TraceFigureKey<TraceProtocol> =
    FigureKey<<<TraceProtocol as IsTraceProtocol>::Pedestal as IsPedestal>::VarId>;

impl<VarId: IsVarIdFull> FigureKey<VarId> {
    pub fn new<Pedestal, TraceProtocol>(
        followed: Option<TraceId>,
        accompanyings_except_followed: AccompanyingTraceIdsExceptFollowed,
        caryatid: &TraceProtocol::Caryatid,
        trace_synchrotron: &TraceSynchrotron<TraceProtocol>,
    ) -> Self
    where
        Pedestal: IsPedestal<VarId = VarId>,
        TraceProtocol: IsTraceProtocol<Pedestal = Pedestal, Caryatid: IsCaryatid>,
    {
        let mut joint_static_var_anchors: OrderedSmallVecPairMap<
            ItemPathIdInterface,
            Anchor<Pedestal::VarId>,
            4,
        > = Default::default();
        let mut t = |&trace_id: &TraceId| -> bool {
            let entry = &trace_synchrotron[trace_id];
            let var_deps = entry.var_deps();
            if !caryatid.has(var_deps) {
                return false;
            }
            joint_static_var_anchors.extend(
                var_deps
                    .iter()
                    .copied()
                    .map(|dep| (dep, caryatid[dep].into())),
            );
            true
        };
        let followed_reduced = followed.filter(&mut t);
        let accompanyings_except_followed_reduced = accompanyings_except_followed
            .iter()
            .copied()
            .filter(t)
            .collect();
        Self {
            followed_reduced,
            accompanyings_except_followed_reduced,
            joint_static_var_anchors,
        }
    }
}

impl<VarId: IsVarIdFull> FigureKey<VarId> {
    pub fn followed_reduced(&self) -> Option<TraceId> {
        self.followed_reduced
    }

    pub fn accompanyings_except_followed_reduced(&self) -> &[TraceId] {
        &self.accompanyings_except_followed_reduced
    }

    pub fn traces<'a>(&'a self) -> impl Iterator<Item = TraceId> + 'a {
        self.followed_reduced
            .into_iter()
            .chain(self.accompanyings_except_followed_reduced.iter().copied())
    }

    pub fn joint_static_var_anchors(&self) -> &[(ItemPathIdInterface, Anchor<VarId>)] {
        &self.joint_static_var_anchors
    }
}
