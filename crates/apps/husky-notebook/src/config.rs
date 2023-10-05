mod background;
mod doc;
mod foreground;
mod layout;

use egui::Color32;

pub(crate) use self::background::*;
pub(crate) use self::doc::*;
pub(crate) use self::foreground::*;
pub(crate) use self::layout::*;

#[derive(Default, PartialEq, Eq)]
pub(crate) struct NotebookConfig {
    background: HuskyNotebookBackgroundConfig,
    foreground: HuskyNotebookForegroundConfig,
    layout: HuskyNotebookLayoutConfig,
    doc: HuskyNotebookDocConfig,
}

impl NotebookConfig {
    pub(crate) fn layout(&self) -> &HuskyNotebookLayoutConfig {
        &self.layout
    }

    pub(crate) fn activity_bar_frame(&self) -> egui::Frame {
        egui::Frame::none()
    }

    pub(crate) fn main_panel_frame(&self) -> egui::containers::Frame {
        egui::containers::Frame::default().fill(Color32::LIGHT_BLUE)
    }
}
