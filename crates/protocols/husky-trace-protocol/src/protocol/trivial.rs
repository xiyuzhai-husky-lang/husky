use husky_visual_protocol::trivial::TrivialVisualProtocol;

use super::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TrivialTraceProtocol;

impl IsTraceProtocol for TrivialTraceProtocol {
    type Pedestal = ();
    type VisualProtocol = TrivialVisualProtocol;
}
