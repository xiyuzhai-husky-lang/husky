

use self::visual::primitive::PrimitiveVisual;
use super::*;

impl Visualize for f32 {
    fn visualize(&self, _visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        PrimitiveVisual::F32((*self).into()).into()
    }
}
