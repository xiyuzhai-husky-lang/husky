use crate::TraceId;
use husky_visual_protocol::{
    synchrotron::VisualSynchrotron,
    visual::{image::ImageVisual, Visual},
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use ui::ui::{IsUi, UiTextureId};

/// `IsFigure` extends `Serialize` and `Deserialize` for the convenience of deriving `Serialize` and `Deserialize` for generic types
///
/// for example TraceSynchrotron
pub trait IsFigure:
    std::fmt::Debug + PartialEq + Eq + Clone + Serialize + for<'a> Deserialize<'a> + Send + 'static
{
    /// construct a figure for a specific datapoint
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl IntoIterator<Item = (TraceId, Visual)>,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self;

    fn new_generic() -> Self;
}

impl IsFigure for () {
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl IntoIterator<Item = (TraceId, Visual)>,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        ()
    }

    fn new_generic() -> Self {
        ()
    }
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
