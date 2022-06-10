use husky_debugger_protocol::*;
use vm::*;

#[derive(Clone, Copy)]
pub enum StaticVisualizer {
    Compiled(for<'temp, 'eval> fn(&(dyn AnyValueDyn<'eval> + 'temp)) -> VisualProps),
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
        match (self, other) {
            (StaticVisualizer::Compiled(f0), StaticVisualizer::Compiled(f1)) => {
                let f0 = *f0 as usize;
                let f1 = *f1 as usize;
                f0 == f1
            }
            (StaticVisualizer::Vec, StaticVisualizer::Vec) => true,
            _ => false,
        }
    }
}

impl Eq for StaticVisualizer {}

pub const TRIVIAL_VISUALIZER: StaticVisualizer = StaticVisualizer::Compiled(visualize_trivial);

fn visualize_trivial<'temp, 'eval>(_data: &(dyn AnyValueDyn<'eval> + 'temp)) -> VisualProps {
    VisualProps::Primitive { value: ().into() }
}
