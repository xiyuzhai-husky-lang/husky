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
            (slf, PlotClass::Void) => slf,
            (PlotClass::Void, other) => other,
            (slf, other) if slf == other => slf,
            _ => PlotClass::Any,
        }
    }

    fn group(self) -> PlotClass {
        match self {
            PlotClass::Void => PlotClass::Void,
            PlotClass::Graphics2D => PlotClass::Graphics2D,
            PlotClass::Graphics3D => PlotClass::Graphics3D,
            PlotClass::Any => PlotClass::Any,
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
            Visual::Primitive(_) => PlotClass::Any,
            Visual::RichText(_) => todo!(),
            Visual::Shape(_) => PlotClass::Graphics2D,
            Visual::Text(_) => todo!(),
            Visual::Video(_) => todo!(),
            Visual::Group(visual) => {
                let mut plot_class = PlotClass::Void;
                for &elem in visual.elements(visual_synchrotron) {
                    plot_class.merge(elem.plot_class(visual_synchrotron).group())
                }
                plot_class
            }
            Visual::Error => PlotClass::Any,
        }
    }
}
