mod query;

pub use query::*;

use compile_time_db::*;
use entity_route::{BuiltinScopeSignature, EntityRoutePtr, EntitySource};
use std::sync::Arc;
use visual_syntax::{BuiltinVisualizer, VisualProps};
use vm::AnyValueDyn;

#[derive(Clone)]
pub enum RuntimeVisualizer {
    Compiled(for<'eval> fn(&(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps),
}

impl RuntimeVisualizer {
    pub fn visualize<'a, 'eval>(&self, value: &(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps {
        match self {
            RuntimeVisualizer::Compiled(compiled) => compiled(value),
        }
    }
}

impl std::fmt::Debug for RuntimeVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiled(arg0) => f.write_str("Compiled"),
        }
    }
}

impl PartialEq for RuntimeVisualizer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Compiled(l0), Self::Compiled(r0)) => {
                let l0: *const u8 = *l0 as *const u8;
                let r0: *const u8 = *r0 as *const u8;
                l0 == r0
            }
        }
    }
}

impl Eq for RuntimeVisualizer {}

impl From<&BuiltinVisualizer> for RuntimeVisualizer {
    fn from(builtin_visualizer: &BuiltinVisualizer) -> Self {
        RuntimeVisualizer::Compiled(builtin_visualizer.compiled)
    }
}
