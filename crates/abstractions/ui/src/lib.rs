pub trait IsUi {
    type Response;
}

pub trait IsWidget<Ui: IsUi> {
    fn ui(self, ui: &mut Ui) -> Ui::Response;
}

pub trait IsUiComponent<Ui: IsUi, UiComponentConfig> {
    fn render(&mut self, ui: &mut Ui, config: &UiComponentConfig) -> Ui::Response;
}

pub struct UiComponent<Ui: IsUi, UiComponentConfig>(Box<dyn IsUiComponent<Ui, UiComponentConfig>>);

impl<Ui: IsUi, UiComponentConfig> UiComponent<Ui, UiComponentConfig> {
    pub fn render(&mut self, ui: &mut Ui, config: &UiComponentConfig) -> Ui::Response {
        self.0.render(ui, config)
    }
}

impl<Ui: IsUi, UiComponentConfig> UiComponent<Ui, UiComponentConfig> {
    pub fn new<UiComponentImpl>(ui_component: UiComponentImpl) -> Self
    where
        UiComponentImpl: IsUiComponent<Ui, UiComponentConfig> + 'static,
    {
        Self(Box::new(ui_component))
    }
}

#[cfg(feature = "egui")]
impl IsUi for egui::Ui {
    type Response = egui::Response;
}
