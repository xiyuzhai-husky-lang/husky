use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::image::ImageVisual};

#[cfg(feature = "egui")]
pub mod egui;

pub trait IsUi: Sized {
    type TextureHandle: IsTextureHandle;
    fn load_texture(
        &self,
        image: ImageVisual,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self::TextureHandle;
}

pub trait IsTextureHandle {
    type TextureId;

    fn id(&self) -> Self::TextureId;
}

pub type UiTextureId<Ui> = <<Ui as IsUi>::TextureHandle as IsTextureHandle>::TextureId;
