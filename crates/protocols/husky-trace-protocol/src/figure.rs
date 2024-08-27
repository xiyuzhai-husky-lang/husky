use crate::{
    accompany::AccompanyingTraceIdsExceptFollowed, anchor::Anchor, caryatid::IsCaryatid,
    chart::Chart, IsTraceProtocol, TraceId, TraceSynchrotron,
};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::KiReprInterface;
use husky_linket_impl::{
    pedestal::{IsPedestal, IsPedestalFull},
    static_var::{IsStaticVarId, IsStaticVarIdFull},
};
use husky_visual_protocol::{
    synchrotron::VisualSynchrotron,
    visual::{image::ImageVisual, CompositeVisual, Visual},
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use ui::ui::{IsUi, UiTextureId};
use vec_like::{ordered_small_vec_map::OrderedSmallVecPairMap, OrderedSmallVecSet, SmallVecSet};

/// `IsFigure` extends `Serialize` and `Deserialize` for the convenience of deriving `Serialize` and `Deserialize` for generic types
///
/// for example TraceSynchrotron
pub trait IsFigure:
    std::fmt::Debug + PartialEq + Eq + Clone + Serialize + for<'a> Deserialize<'a> + Send + 'static
{
    type Pedestal: IsPedestal;

    fn from_chart_of_composite_visuals(
        chart: Option<Chart<<Self::Pedestal as IsPedestal>::StaticVarId, CompositeVisual<TraceId>>>,
    ) -> Self;
}

pub trait FigureUi<Ui: IsUi> {
    fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    );
}

pub struct FigureUiCache<Ui: IsUi> {
    //todo: optimize by LRU??
    texture_handles: FxHashMap<ImageVisual, Ui::TextureHandle>,
}

impl<Ui: IsUi> Default for FigureUiCache<Ui> {
    fn default() -> Self {
        Self {
            texture_handles: Default::default(),
        }
    }
}

impl<Ui: IsUi> FigureUiCache<Ui> {
    #[inline(always)]
    pub fn texture_id(
        &mut self,
        image: ImageVisual,
        visual_synchrotron: &VisualSynchrotron,
        ui: &Ui,
    ) -> UiTextureId<Ui> {
        self.texture_id_aux(image, || ui.load_texture(image, visual_synchrotron))
    }

    // get or insert with
    fn texture_id_aux(
        &mut self,
        image: ImageVisual,
        f: impl FnOnce() -> Ui::TextureHandle,
    ) -> UiTextureId<Ui> {
        use ui::ui::IsTextureHandle;
        self.texture_handles.entry(image).or_insert_with(f).id()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct FigureKey<StaticVarId: IsStaticVarId> {
    followed_reduced: Option<TraceId>,
    accompanyings_except_followed_reduced: OrderedSmallVecSet<TraceId, 4>,
    joint_static_var_anchors: OrderedSmallVecPairMap<ItemPathIdInterface, Anchor<StaticVarId>, 4>,
}

pub type TraceFigureKey<TraceProtocol> =
    FigureKey<<<TraceProtocol as IsTraceProtocol>::Pedestal as IsPedestal>::StaticVarId>;

impl<StaticVarId: IsStaticVarIdFull> FigureKey<StaticVarId> {
    pub fn new<Pedestal, TraceProtocol>(
        followed: Option<TraceId>,
        accompanyings_except_followed: AccompanyingTraceIdsExceptFollowed,
        caryatid: &TraceProtocol::Caryatid,
        trace_synchrotron: &TraceSynchrotron<TraceProtocol>,
    ) -> Self
    where
        Pedestal: IsPedestal<StaticVarId = StaticVarId>,
        TraceProtocol: IsTraceProtocol<Pedestal = Pedestal>,
    {
        let mut joint_static_var_anchors: OrderedSmallVecPairMap<
            ItemPathIdInterface,
            Anchor<Pedestal::StaticVarId>,
            4,
        > = Default::default();
        let mut t = |&trace_id: &TraceId| -> bool {
            let entry = &trace_synchrotron[trace_id];
            let var_deps = entry.var_deps();
            if !caryatid.covers(var_deps) {
                return false;
            }
            joint_static_var_anchors.extend(var_deps.iter().copied().map(|_| todo!()));
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

impl<StaticVarId: IsStaticVarIdFull> FigureKey<StaticVarId> {
    pub fn followed_reduced(&self) -> Option<TraceId> {
        self.followed_reduced
    }

    pub fn accompanyings_except_followed_reduced(&self) -> &[TraceId] {
        &self.accompanyings_except_followed_reduced
    }

    pub fn joint_static_var_anchors(&self) -> &[(ItemPathIdInterface, Anchor<StaticVarId>)] {
        &self.joint_static_var_anchors
    }
}
