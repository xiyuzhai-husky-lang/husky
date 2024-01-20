use crate::ui::IsUi;

pub trait IsUiComponent<Ui: IsUi, Settings, ParentActionBuffer> {
    fn render_dyn(
        &mut self,
        ui: &mut Ui,
        settings: &mut Settings,
        action_buffer: &mut ParentActionBuffer,
    );
}

pub struct UiComponent<Ui: IsUi, Settings, ParentActionBuffer>(
    Box<dyn IsUiComponent<Ui, Settings, ParentActionBuffer>>,
);

impl<Ui: IsUi, Settings, ParentActionBuffer> UiComponent<Ui, Settings, ParentActionBuffer> {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        settings: &mut Settings,
        action_buffer: &mut ParentActionBuffer,
    ) {
        self.0.render_dyn(ui, settings, action_buffer)
    }
}

impl<Ui: IsUi, UiComponentConfig, ParentActionBuffer>
    UiComponent<Ui, UiComponentConfig, ParentActionBuffer>
{
    pub fn new<UiComponentImpl>(ui_component: UiComponentImpl) -> Self
    where
        UiComponentImpl: IsUiComponent<Ui, UiComponentConfig, ParentActionBuffer> + 'static,
    {
        Self(Box::new(ui_component))
    }
}
