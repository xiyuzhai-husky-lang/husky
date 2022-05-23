mod props;

pub use props::*;
use vm::AnyValueDyn;

#[derive(Clone, Copy)]
pub enum StaticVisualizer {
    Compiled(for<'eval> fn(&(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps),
    Vec,
}

impl std::fmt::Debug for StaticVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("StaticVisualizer")
    }
}

impl PartialEq for StaticVisualizer {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for StaticVisualizer {}

pub const TRIVIAL_VISUALIZER: StaticVisualizer = StaticVisualizer::Compiled(visualize_trivial);

fn visualize_trivial<'eval>(_data: &(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps {
    VisualProps::Primitive { value: ().into() }
}
