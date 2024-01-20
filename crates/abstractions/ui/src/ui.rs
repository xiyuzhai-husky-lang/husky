#[cfg(feature = "egui")]
pub mod egui;

pub trait IsUi: Sized {
    type Cache;
}
