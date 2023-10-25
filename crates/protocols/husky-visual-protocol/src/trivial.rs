use crate::*;

#[derive(Serialize)]
pub struct TrivialVisualProtocol;

impl IsVisualProtocol for TrivialVisualProtocol {
    type VisualComponent = ();

    type Visual = TrivialVisual;
}

pub struct TrivialVisual;

impl IsVisual for TrivialVisual {
    type Component = ();

    #[inline(always)]
    fn from_components(components: &[Self::Component]) -> Self {
        TrivialVisual
    }
}
