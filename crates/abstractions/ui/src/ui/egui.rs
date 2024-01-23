use super::*;
use ::egui::{ColorImage, ImageData, TextureFilter, TextureHandle, TextureId, TextureOptions, Ui};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::image::ImageVisual};
use rustc_hash::FxHashMap;

impl IsUi for ::egui::Ui {
    type TextureHandle = TextureHandle;

    fn load_texture(
        &self,
        image: ImageVisual,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self::TextureHandle {
        let image: ColorImage = image.data(visual_synchrotron).into();
        let options = TextureOptions {
            magnification: TextureFilter::Nearest,
            minification: TextureFilter::Linear,
        };
        self.ctx().load_texture("whatever", image, options)
    }
}

impl IsTextureHandle for TextureHandle {
    type TextureId = TextureId;

    fn id(&self) -> Self::TextureId {
        TextureHandle::id(self)
    }
}
