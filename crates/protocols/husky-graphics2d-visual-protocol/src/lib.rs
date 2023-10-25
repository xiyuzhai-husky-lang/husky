pub mod action;

use self::action::*;
use husky_visual_protocol::{IsVisual, IsVisualProtocol};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Graphics2dVisualProtocol;

impl IsVisualProtocol for Graphics2dVisualProtocol {
    type VisualComponent = Graphics2dVisualComponent;

    type Visual = Graphics2dVisual;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Graphics2dVisualComponent {}

pub enum Graphics2dVisual {}

impl IsVisual for Graphics2dVisual {
    type Component = Graphics2dVisualComponent;

    fn from_components(components: &[Graphics2dVisualComponent]) -> Self {
        todo!()
    }
}
