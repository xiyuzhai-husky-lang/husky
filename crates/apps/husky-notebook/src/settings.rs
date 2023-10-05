mod background;
mod doc;
mod foreground;
mod layout;

use egui::Color32;
use husky_trace_view_doc::doc::HasTraceViewSettings;
use ui::IsUiComponent;

pub(crate) use self::background::*;
pub(crate) use self::doc::*;
pub(crate) use self::foreground::*;
pub(crate) use self::layout::*;

#[derive(Default, PartialEq, Eq)]
pub(crate) struct NotebookSettings {
    background: HuskyNotebookBackgroundSettings,
    foreground: HuskyNotebookForegroundSettings,
    layout: HuskyNotebookLayoutSettings,
    doc: HuskyNotebookDocSettings,
}

impl NotebookSettings {
    pub(crate) fn activity_bar_frame(&self) -> egui::Frame {
        egui::Frame::none()
    }

    pub(crate) fn main_panel_frame(&self) -> egui::containers::Frame {
        egui::containers::Frame::default().fill(Color32::LIGHT_BLUE)
    }
}

impl HasTraceViewSettings for NotebookSettings {}

pub(crate) struct NotebookSettingsView;

impl<ParentActionBuffer> IsUiComponent<egui::Ui, NotebookSettings, ParentActionBuffer>
    for NotebookSettingsView
{
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut NotebookSettings,
        super_action_buffer: &mut ParentActionBuffer,
    ) {
        ui.label("Ui Component Context");
    }
}
