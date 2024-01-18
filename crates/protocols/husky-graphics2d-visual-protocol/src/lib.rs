pub mod action;

use husky_visual_protocol::IsFigure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Graphics2dFigure;

impl IsFigure for Graphics2dFigure {}
