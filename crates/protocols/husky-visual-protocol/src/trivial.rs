use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TrivialVisualProtocol;

impl IsVisualProtocol for TrivialVisualProtocol {
    type VisualComponent = ();

    type Visual = TrivialVisual;
}

pub struct TrivialVisual;

impl IsVisual for TrivialVisual {
    type Component = ();

    #[inline(always)]
    fn from_components(_components: &[Self::Component]) -> Self {
        TrivialVisual
    }
}
