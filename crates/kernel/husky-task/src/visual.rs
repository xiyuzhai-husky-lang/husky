use crate::*;

pub trait IsVisualProtocol {
    type VisualComponent;

    type Visual: IsVisual;
}

pub trait IsVisual {
    type Component;

    type Ui;

    type UiResponse;

    type Action;

    fn from_components(components: &[Self::Component]) -> Self;

    fn render(
        self,
        ui: &mut Self::Ui,
        action_buffer: &mut ActionBuffer<Self::Action>,
    ) -> (Self::UiResponse);
}

pub type VisualProtocol<Task> = <<Task as IsTask>::DevAscension as IsDevAscension>::VisualProtocol;
pub type VisualComponent<Task> = <VisualProtocol<Task> as IsVisualProtocol>::VisualComponent;

pub struct ActionBuffer<Action> {
    actions: smallvec::SmallVec<[Action; 2]>,
}

impl<Action> ActionBuffer<Action> {
    pub fn push(&mut self, action: Action) {
        self.actions.push(action)
    }
}
