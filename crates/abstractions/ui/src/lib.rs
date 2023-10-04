pub trait IsUi {
    type Response;
}

pub trait IsWidget<Ui: IsUi> {
    fn ui(self, ui: &mut Ui) -> Ui::Response;
}

pub trait IsUiComponent<Ui: IsUi> {
    fn render(&mut self, ui: &mut Ui) -> Ui::Response;
}

pub struct UiComponent<Ui: IsUi>(Box<dyn IsUiComponent<Ui>>);

impl<Ui: IsUi> UiComponent<Ui> {
    pub fn render(&mut self, ui: &mut Ui) -> Ui::Response {
        self.0.render(ui)
    }
}

#[cfg(feature = "egui")]
impl IsUi for egui::Ui {
    type Response = egui::Response;
}
