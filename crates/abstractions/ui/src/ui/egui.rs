use super::*;
use ::egui::{ColorImage, ImageData, TextureFilter, TextureHandle, TextureId, TextureOptions, Ui};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::image::ImageVisual};
use rustc_hash::FxHashMap;

impl IsUi for ::egui::Ui {
    type Cache = UiCache;
}

#[derive(Default)]
pub struct UiCache {
    //todo: optimize by LRU??
    texture_handles: FxHashMap<TextureCacheKey, TextureHandle>,
}

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureCacheKey {
    ImageVisual(ImageVisual),
}

impl UiCache {
    #[inline(always)]
    pub fn texture_id(
        &mut self,
        texture_cache_key: impl Into<TextureCacheKey>,
        visual_synchrotron: &VisualSynchrotron,
        ui: &Ui,
    ) -> TextureId {
        let texture_cache_key = texture_cache_key.into();
        let (image, options): (ImageData, _) = match texture_cache_key {
            TextureCacheKey::ImageVisual(image) => {
                let image: ColorImage = image.data(visual_synchrotron).into();
                (
                    image.into(),
                    TextureOptions {
                        magnification: TextureFilter::Nearest,
                        minification: TextureFilter::Linear,
                    },
                )
            }
        };
        self.get_texture_id_or_insert_texture_handle_with(texture_cache_key, || {
            ui.ctx().load_texture("whatever", image, options)
        })
    }

    fn get_texture_id_or_insert_texture_handle_with(
        &mut self,
        texture_cache_key: TextureCacheKey,
        f: impl FnOnce() -> TextureHandle,
    ) -> TextureId {
        let texture_cache_key = texture_cache_key.into();
        self.texture_handles
            .entry(texture_cache_key)
            .or_insert_with(f)
            .id()
    }
}
