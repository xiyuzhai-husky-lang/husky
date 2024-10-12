use crate::{
    accompany::AccompanyingTraceIdsExceptFollowed, anchor::Anchor, caryatid::IsCaryatid,
    chart::Chart, server::TracePlotInfos, windlass::Windlass, IsTraceProtocol, TraceId,
    TraceSynchrotron,
};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::KiReprInterface;
use husky_linket_impl::{
    pedestal::{IsPedestal, IsPedestalFull, JointPedestal},
    var_id::{IsVarId, IsVarIdFull},
};
use husky_visual_protocol::{
    plot::PlotClass,
    synchrotron::VisualSynchrotron,
    visual::{image::ImageVisual, CompositeVisual, Visual},
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use smallvec::SmallVec;
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

    fn for_all_joint_pedestals(
        &self,
        f: impl FnMut(&JointPedestal<<Self::Pedestal as IsPedestal>::VarId>),
    );
}

pub trait FigureUi<Ui: IsUi> {
    fn figure_ui(
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

pub type FigureKeys<VarId> = SmallVec<[FigureKey<VarId>; 4]>;
pub type TraceFigureKey<TraceProtocol> =
    FigureKey<<<TraceProtocol as IsTraceProtocol>::Pedestal as IsPedestal>::VarId>;
pub type TraceFigureKeys<TraceProtocol> =
    FigureKeys<<<TraceProtocol as IsTraceProtocol>::Pedestal as IsPedestal>::VarId>;

impl<VarId: IsVarIdFull> FigureKey<VarId> {
    pub fn collect_from_caryatid<Pedestal, TraceProtocol>(
        followed: Option<TraceId>,
        accompanyings_except_followed: AccompanyingTraceIdsExceptFollowed,
        caryatid: &TraceProtocol::Caryatid,
        trace_synchrotron: &TraceSynchrotron<TraceProtocol>,
    ) -> FigureKeys<VarId>
    where
        Pedestal: IsPedestal<VarId = VarId>,
        TraceProtocol: IsTraceProtocol<Pedestal = Pedestal, Caryatid: IsCaryatid>,
    {
        let mut keys = smallvec![];
        let mut anchors: SmallVec<[(ItemPathIdInterface, Anchor<Pedestal::VarId>); 4]> =
            smallvec![];
        for (item_path, windlass) in caryatid.var_path_windlasses() {
            match windlass {
                Windlass::Specific(_) => todo!(),
                Windlass::Generic {
                    page_start,
                    followed,
                    zone: None,
                    page_limit,
                } => todo!(),
                Windlass::Generic {
                    page_start,
                    followed,
                    zone: Some(zone),
                    page_limit,
                } => todo!(),
            }
            todo!()
        }
        todo!();
        keys
    }

    fn new<Pedestal, TraceProtocol>(
        followed: Option<TraceId>,
        accompanyings_except_followed: AccompanyingTraceIdsExceptFollowed,
        anchors: &[(ItemPathIdInterface, Anchor<Pedestal::VarId>)],
        trace_synchrotron: &TraceSynchrotron<TraceProtocol>,
    ) -> Self
    where
        Pedestal: IsPedestal<VarId = VarId>,
        TraceProtocol: IsTraceProtocol<Pedestal = Pedestal, Caryatid: IsCaryatid>,
    {
        // Initialize reduced_anchors with all generic anchors,
        // because generic anchors are definitely going to be needed
        let mut reduced_anchors: OrderedSmallVecPairMap<
            ItemPathIdInterface,
            Anchor<Pedestal::VarId>,
            4,
        > = anchors
            .iter()
            .filter(|(id, anchor)| anchor.is_generic())
            .copied()
            .collect();
        let mut t = |&trace_id: &TraceId| -> bool {
            let entry = &trace_synchrotron[trace_id];
            let var_deps = entry.var_deps();
            // if anchors don't contain all var_deps, we can't render the trace
            if !var_deps
                .iter()
                .all(|dep| anchors.iter().any(|(id, _)| id == dep))
            {
                return false;
            }
            reduced_anchors.extend(var_deps.iter().copied().filter_map(|dep| {
                anchors
                    .iter()
                    .find(|(id, _)| *id == dep)
                    .map(|&(_, anchor)| (dep, anchor))
            }));
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
            joint_static_var_anchors: reduced_anchors,
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
