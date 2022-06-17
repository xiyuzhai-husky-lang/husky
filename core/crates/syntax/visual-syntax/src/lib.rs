use husky_tracer_protocol::*;
use vm::*;

pub struct StaticVisualizer {
    pub ty: StaticVisualTy,
    pub variant: StaticVisualizerVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticVisualTy {
    Void,
    B32,
    I32,
    F32,
    Group,
    Point2d,
    Shape2d,
    Region2d,
    Image2d,
    Graphics2d,
}

#[derive(Clone)]
pub enum StaticVisualizerVariant {
    Compiled {
        call: for<'temp, 'eval> fn(&(dyn AnyValueDyn<'eval> + 'temp)) -> VisualData,
    },
    Vec,
    CyclicSlice,
}

impl std::fmt::Debug for StaticVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("StaticVisualizer")
    }
}

impl PartialEq for StaticVisualizer {
    fn eq(&self, other: &Self) -> bool {
        if self.ty != other.ty {
            return false;
        }
        match (&self.variant, &other.variant) {
            (
                StaticVisualizerVariant::Compiled { call: f0 },
                StaticVisualizerVariant::Compiled { call: f1 },
            ) => {
                let f0 = *f0 as usize;
                let f1 = *f1 as usize;
                f0 == f1
            }
            (StaticVisualizerVariant::Vec, StaticVisualizerVariant::Vec) => true,
            _ => false,
        }
    }
}

impl Eq for StaticVisualizer {}

pub const TRIVIAL_VISUALIZER: StaticVisualizer = StaticVisualizer {
    ty: StaticVisualTy::Void,
    variant: StaticVisualizerVariant::Compiled {
        call: visualize_trivial,
    },
};

fn visualize_trivial<'temp, 'eval>(_data: &(dyn AnyValueDyn<'eval> + 'temp)) -> VisualData {
    VisualData::Primitive { value: ().into() }
}
