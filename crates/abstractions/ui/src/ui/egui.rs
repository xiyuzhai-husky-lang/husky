use super::*;
use ::egui::{
    ColorImage, TextureFilter, TextureHandle, TextureId, TextureOptions, TextureWrapMode,
};

impl IsUi for ::egui::Ui {
    type TextureHandle = TextureHandle;

    fn load_texture(
        &self,
        image: ImageVisual,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self::TextureHandle {
        let image: ColorImage = image.color_image(visual_synchrotron);
        let options = TextureOptions {
            magnification: TextureFilter::Nearest,
            minification: TextureFilter::Linear,
            wrap_mode: TextureWrapMode::default(),
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
