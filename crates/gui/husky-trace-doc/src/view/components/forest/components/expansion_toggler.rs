use super::*;
use egui::{Response, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ExpansionTogglerButtonStyle {
    PlusMinus,
    ConcreteArrowHead,
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in super::super) fn render_expansion_toggler(
        &mut self,
        trace_id: TraceId,
        expanded: bool,
        ui: &mut Ui,
    ) -> Response {
        // todo: different styles
        let toggler_text = ExpansionTogglerButtonStyle::PlusMinus.fun_name(expanded);
        let toggler_text = RichText::new(toggler_text).family(FontFamily::Monospace);
        let toggler = Label::new(toggler_text).sense(Sense::click());
        let toggler_response = toggler.ui(ui);
        if toggler_response.clicked() {
            self.add_action(TraceViewAction::ToggleExpansion { trace_id })
        };
        toggler_response
    }
}

impl ExpansionTogglerButtonStyle {
    fn fun_name(&self, expanded: bool) -> &str {
        match self {
            ExpansionTogglerButtonStyle::PlusMinus => match expanded {
                true => "-",
                false => "+",
            },
            ExpansionTogglerButtonStyle::ConcreteArrowHead => match expanded {
                true => "-",
                false => "+",
            },
        }
    }
}
