use husky_task::visual::IsVisualProtocol;

pub struct DefaultVisualProtocol;

impl IsVisualProtocol for DefaultVisualProtocol {
    type VisualElement = DefaultVisualElement;
}

pub enum DefaultVisualElement {
    Text,
    Shape2d,
    Mesh2d,
    Mesh3d,
}
