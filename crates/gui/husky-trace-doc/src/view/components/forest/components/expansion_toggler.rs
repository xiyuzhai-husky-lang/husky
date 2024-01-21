use super::*;
use egui::{Response, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ExpansionTogglerSymbolStyle {
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
        let symbol_style = ExpansionTogglerSymbolStyle::PlusMinus;
        let toggler =
            Label::new(RichText::new(symbol_style.text(expanded)).family(FontFamily::Monospace))
                .sense(Sense::click());
        let toggler_response = toggler.ui(ui);
        if toggler_response.clicked() {
            self.add_action(TraceViewAction::ToggleExpansion { trace_id })
        };
        toggler_response
    }
}

impl ExpansionTogglerSymbolStyle {
    fn text(&self, expanded: bool) -> &str {
        match self {
            ExpansionTogglerSymbolStyle::PlusMinus => match expanded {
                true => "-",
                false => "+",
            },
            ExpansionTogglerSymbolStyle::ConcreteArrowHead => match expanded {
                true => "⏷",
                false => "⏵",
            },
        }
    }
}
