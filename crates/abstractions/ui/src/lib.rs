pub trait IsUiComponent<Ui, UiComponentConfig> {
    fn render(&mut self, ui: &mut Ui, config: &UiComponentConfig);
}

pub struct UiComponent<Ui, UiComponentConfig>(Box<dyn IsUiComponent<Ui, UiComponentConfig>>);

impl<Ui, UiComponentConfig> UiComponent<Ui, UiComponentConfig> {
    pub fn render(&mut self, ui: &mut Ui, config: &UiComponentConfig) {
        self.0.render(ui, config)
    }
}

impl<Ui, UiComponentConfig> UiComponent<Ui, UiComponentConfig> {
    pub fn new<UiComponentImpl>(ui_component: UiComponentImpl) -> Self
    where
        UiComponentImpl: IsUiComponent<Ui, UiComponentConfig> + 'static,
    {
        Self(Box::new(ui_component))
    }
}
