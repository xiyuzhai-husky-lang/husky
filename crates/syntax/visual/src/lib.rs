mod props;

pub use props::*;
use vm::AnyValueDyn;

#[derive(Clone, Copy)]
pub struct StaticVisualizer {
    pub compiled: for<'eval> fn(&(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps,
}

impl std::fmt::Debug for StaticVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // f.write_str("BuiltinVisualizer")
    }
}

impl PartialEq for StaticVisualizer {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for StaticVisualizer {}

pub const TRIVIAL_VISUALIZER: StaticVisualizer = StaticVisualizer {
    compiled: visualize_trivial,
};

fn visualize_trivial<'eval>(data: &(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps {
    todo!()
}
