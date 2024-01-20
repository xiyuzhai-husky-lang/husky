pub mod action;

use husky_visual_protocol::figure::IsFigure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Graphics2dFigure;

impl IsFigure for Graphics2dFigure {
    fn new_specific() -> Self {
        todo!()
    }
}
