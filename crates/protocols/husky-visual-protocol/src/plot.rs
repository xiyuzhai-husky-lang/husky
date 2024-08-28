use crate::{synchrotron::VisualSynchrotron, visual::Visual};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PlotClass {
    Void,
    Graphics2D,
    Graphics3D,
    Any,
}

impl PlotClass {
    pub fn merge(&mut self, other: Self) {
        *self = match (*self, other) {
            (PlotClass::Void, other) => other,
            (slf, other) if slf == other => slf,
            _ => PlotClass::Any,
        }
    }
}

impl Visual {
    pub fn plot_class(self, visual_synchrotron: &VisualSynchrotron) -> PlotClass {
        match self {
            Visual::Void => PlotClass::Void,
            Visual::Image(_) => PlotClass::Graphics2D,
            Visual::Math(_) => todo!(),
            Visual::Mesh(_) => todo!(),
            Visual::Primitive(_) => todo!(),
            Visual::RichText(_) => todo!(),
            Visual::Shape(_) => todo!(),
            Visual::Text(_) => todo!(),
            Visual::Video(_) => todo!(),
            Visual::Group(visual) => todo!(),
        }
    }
}
