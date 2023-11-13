//! Daniel Jackson is a character in Stargate SG1.
//!
//! He is a linguist.
//!
//! So this visualization serves mainly for linguistics.
pub mod action;


use husky_visual_protocol::{IsVisual, IsVisualComponent, IsVisualProtocol};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DanielJacksonVisualProtocol;

impl IsVisualProtocol for DanielJacksonVisualProtocol {
    type VisualComponent = DanielJacksonVisualComponent;

    type Visual = DanielJacksonVisual;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DanielJacksonVisualComponent {
    Text,
    Shape2d,
    Mesh2d,
    Mesh3d,
}

impl IsVisualComponent for DanielJacksonVisualComponent {}

pub enum DanielJacksonVisual {
    Text,
    Shape2d,
    Mesh2d,
    Mesh3d,
}

impl IsVisual for DanielJacksonVisual {
    type Component = DanielJacksonVisualComponent;

    fn from_components(_components: &[DanielJacksonVisualComponent]) -> Self {
        todo!()
    }
}
