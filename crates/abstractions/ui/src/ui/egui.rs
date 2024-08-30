use super::*;
use ::egui::{
    ColorImage, TextureFilter, TextureHandle, TextureId, TextureOptions, TextureWrapMode,
};

impl IsUi for ::egui::Ui {
    type Rect = ::egui::Rect;
    type TextureHandle = TextureHandle;
    type HotkeyBuffer = crate::hotkey::egui::HotkeyBuffer;
    type Color32 = ::egui::Color32;

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

    fn paint_image(
        &self,
        texture_id: UiTextureId<Self>,
        rect: Self::Rect,
        uv: Self::Rect,
        tint: Self::Color32,
    ) {
        self.painter().image(texture_id, rect, uv, tint);
    }
}

impl IsTextureHandle for TextureHandle {
    type TextureId = TextureId;

    fn id(&self) -> Self::TextureId {
        TextureHandle::id(self)
    }
}
