#[cfg(feature = "mock")]
pub mod mock;
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
        action_buffer: &mut VisualActionBuffer<Self::Action>,
    ) -> Ui::Response;
}

pub type VisualComponent<VisualProtocol> = <VisualProtocol as IsVisualProtocol>::VisualComponent;

pub struct VisualActionBuffer<VisualAction> {
    actions: smallvec::SmallVec<[VisualAction; 2]>,
}

impl<VisualAction> Default for VisualActionBuffer<VisualAction> {
    fn default() -> Self {
        Self {
            actions: Default::default(),
        }
    }
}

impl<Action> VisualActionBuffer<Action> {
    pub fn push(&mut self, action: Action) {
        self.actions.push(action)
    }
}
