use super::*;
use husky_visual_protocol::mock::MockVisualProtocol;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockTraceProtocol;

impl IsTraceProtocol for MockTraceProtocol {
    type Pedestal = ();
    type VisualProtocol = MockVisualProtocol;
}
