use husky_trace_protocol::figure::FigureUiCache;

#[derive(Default)]
pub struct MnistUiCache {
    figure_ui_cache: FigureUiCache<egui::Ui>,
}
