use husky_trace_protocol::figure::FigureUiCache;

#[derive(Default)]
pub struct MnistUiCache {
    figure_ui_cache: FigureUiCache<egui::Ui>,
}

impl MnistUiCache {
    pub fn figure_ui_cache_mut(&mut self) -> &mut FigureUiCache<egui::Ui> {
        &mut self.figure_ui_cache
    }
}
