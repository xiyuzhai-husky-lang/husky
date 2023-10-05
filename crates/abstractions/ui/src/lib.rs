pub trait IsUiComponent<Ui, UiComponentConfig, UiActionBuffer> {
    fn render(
        &mut self,
        ui: &mut Ui,
        config: &UiComponentConfig,
        action_buffer: &mut UiActionBuffer,
    );
}

pub struct UiComponent<Ui, UiComponentConfig, UiAction>(
    Box<dyn IsUiComponent<Ui, UiComponentConfig, UiAction>>,
);

impl<Ui, UiComponentConfig, UiActionBuffer> UiComponent<Ui, UiComponentConfig, UiActionBuffer> {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        config: &UiComponentConfig,
        action_buffer: &mut UiActionBuffer,
    ) {
        self.0.render(ui, config, action_buffer)
    }
}

impl<Ui, UiComponentConfig, UiActionBuffer> UiComponent<Ui, UiComponentConfig, UiActionBuffer> {
    pub fn new<UiComponentImpl>(ui_component: UiComponentImpl) -> Self
    where
        UiComponentImpl: IsUiComponent<Ui, UiComponentConfig, UiActionBuffer> + 'static,
    {
        Self(Box::new(ui_component))
    }
}

// pub struct UiActionBuffer<UiAction> {
//     actions: smallvec::SmallVec<[UiAction; 2]>,
// }

// impl<UiAction> Default for UiActionBuffer<UiAction> {
//     fn default() -> Self {
//         Self {
//             actions: Default::default(),
//         }
//     }
// }

// impl<UiAction> UiActionBuffer<UiAction> {
//     pub fn push(&mut self, action: UiAction) {
//         self.actions.push(action)
//     }
// }
