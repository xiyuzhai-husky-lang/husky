use super::*;

struct MyTabs {
    tree: egui_dock::Tree<String>,
}

impl HuskyNotebookApp {
    pub(super) fn render_main_panel(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let style = egui_dock::Style::from_egui(ui.style().as_ref());
        egui_dock::DockArea::new(&mut self.doctree)
            .style(style)
            .show_inside(ui, &mut TabViewer {})
    }
}

impl MyTabs {
    pub fn new() -> Self {
        let tab1 = "tab1".to_string();
        let tab2 = "tab2".to_string();

        let mut tree = egui_dock::Tree::new(vec![tab1]);
        tree.split_left(egui_dock::NodeIndex::root(), 0.20, vec![tab2]);

        Self { tree }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let style = egui_dock::Style::from_egui(ui.style().as_ref());
        egui_dock::DockArea::new(&mut self.tree)
            .style(style)
            .show_inside(ui, &mut TabViewer {});
    }
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}
