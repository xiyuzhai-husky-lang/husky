use crate::*;

pub trait IsVisualProtocol {
    type VisualComponent;

    type Visual: IsVisual;
}

pub trait IsVisual {
    type Component;

    type Action;

    fn from_components(components: &[Self::Component]) -> Self;

    fn render<Ui: ui::IsUi>(
        self,
        ui: &mut Ui,
        action_buffer: &mut ActionBuffer<Self::Action>,
    ) -> Ui::Response;
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
