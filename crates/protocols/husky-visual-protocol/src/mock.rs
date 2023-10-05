use crate::*;

pub struct MockVisualProtocol {}

impl IsVisualProtocol for MockVisualProtocol {
    type VisualComponent = ();

    type Visual = MockVisual;
}

pub struct MockVisual;

impl IsVisual for MockVisual {
    type Component = ();

    type Action = ();

    fn from_components(components: &[Self::Component]) -> Self {
        todo!()
    }

    fn render<Ui: ui::IsUi>(
        self,
        ui: &mut Ui,
        action_buffer: &mut VisualActionBuffer<Self::Action>,
    ) -> Ui::Response {
        todo!()
    }
}
