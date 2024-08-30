use ui::visual::cache::VisualUiCache;

#[derive(Default)]
pub struct MnistUiCache {
    figure_ui_cache: VisualUiCache<egui::Ui>,
}

impl MnistUiCache {
    pub fn figure_ui_cache_mut(&mut self) -> &mut VisualUiCache<egui::Ui> {
        &mut self.figure_ui_cache
    }
}
