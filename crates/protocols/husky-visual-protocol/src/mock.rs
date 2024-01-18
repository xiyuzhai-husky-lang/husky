use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MockFigure {}

impl IsFigure for MockFigure {}
