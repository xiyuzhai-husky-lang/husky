//! Daniel Jackson is a character in Stargate SG1.
//!
//! He is a linguist.
//!
//! So this visualization serves mainly for linguistics.
pub mod action;

use self::action::*;
use husky_task::visual::{ActionBuffer, IsVisual, IsVisualProtocol};

pub struct DanielJacksonVisualProtocol;

impl IsVisualProtocol for DanielJacksonVisualProtocol {
    type VisualComponent = DanielJacksonVisualComponent;

    type Visual = DanielJacksonVisual;
}

pub enum DanielJacksonVisualComponent {
    Text,
    Shape2d,
    Mesh2d,
    Mesh3d,
}

pub enum DanielJacksonVisual {
    Text,
    Shape2d,
    Mesh2d,
    Mesh3d,
}

impl IsVisual for DanielJacksonVisual {
    type Component = DanielJacksonVisualComponent;

    type Ui = ();

    type UiResponse = ();

    type Action = DanielJacksonVisualAction;

    fn from_components(components: &[DanielJacksonVisualComponent]) -> Self {
        todo!()
    }

    fn render(
        self,
        ui: &mut Self::Ui,
        action_buffer: &mut ActionBuffer<DanielJacksonVisualAction>,
    ) -> Self::UiResponse {
        todo!()
    }
}
