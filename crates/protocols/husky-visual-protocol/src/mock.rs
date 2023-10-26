use crate::*;

#[derive(Serialize, Deserialize)]
pub struct MockVisualProtocol {}

impl IsVisualProtocol for MockVisualProtocol {
    type VisualComponent = ();

    type Visual = MockVisual;
}

pub struct MockVisual;

impl IsVisual for MockVisual {
    type Component = ();

    fn from_components(components: &[Self::Component]) -> Self {
        todo!()
    }
}
