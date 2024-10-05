use crate::ui::IsUi;

pub trait ComponentUi<Ui: IsUi, Settings, ParentActionBuffer> {
    fn component_ui(
        &mut self,
        settings: &mut Settings,
        hotkey_buffer: &mut Ui::HotkeyBuffer,
        action_buffer: &mut ParentActionBuffer,
        ui: &mut Ui,
    );

    fn toggle_help_facade(&mut self);
}

pub struct UiComponent<Ui: IsUi, Settings, ParentActionBuffer>(
    Box<dyn ComponentUi<Ui, Settings, ParentActionBuffer>>,
);

impl<Ui: IsUi, Settings, ParentActionBuffer> UiComponent<Ui, Settings, ParentActionBuffer> {
    pub fn component_ui(
        &mut self,
        settings: &mut Settings,
        hotkey_buffer: &mut Ui::HotkeyBuffer,
        action_buffer: &mut ParentActionBuffer,
        ui: &mut Ui,
    ) {
        self.0
            .component_ui(settings, hotkey_buffer, action_buffer, ui)
    }
}

impl<Ui: IsUi, UiComponentConfig, ParentActionBuffer>
    UiComponent<Ui, UiComponentConfig, ParentActionBuffer>
{
    pub fn new<UiComponentImpl>(ui_component: UiComponentImpl) -> Self
    where
        UiComponentImpl: ComponentUi<Ui, UiComponentConfig, ParentActionBuffer> + 'static,
    {
        Self(Box::new(ui_component))
    }
}
