mod query;

use avec::Avec;
pub use query::*;

use compile_time_db::*;
use entity_route::EntityRoutePtr;
use semantics_eager::FuncStmt;
use std::sync::Arc;
use visual_syntax::{StaticVisualizer, VisualProps};
use vm::{AnyValueDyn, InstructionSheet};

#[derive(Clone)]
pub enum RuntimeVisualizer {
    Compiled(for<'eval> fn(&(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps),
    Interpreted {
        stmts: Avec<FuncStmt>,
        instruction_sheet: Arc<InstructionSheet>,
    },
}

impl RuntimeVisualizer {
    pub fn visualize<'a, 'eval>(&self, value: &(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps {
        match self {
            RuntimeVisualizer::Compiled(compiled) => compiled(value),
            RuntimeVisualizer::Interpreted { .. } => todo!(),
        }
    }
}

impl std::fmt::Debug for RuntimeVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVisualizer::Compiled(arg0) => f.write_str("Compiled"),
            RuntimeVisualizer::Interpreted { .. } => f.write_str("Interpreted"),
        }
    }
}

impl PartialEq for RuntimeVisualizer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuntimeVisualizer::Compiled(l0), RuntimeVisualizer::Compiled(r0)) => {
                let l0: *const u8 = *l0 as *const u8;
                let r0: *const u8 = *r0 as *const u8;
                l0 == r0
            }
            (
                RuntimeVisualizer::Interpreted {
                    instruction_sheet: instruction_sheet0,
                    ..
                },
                RuntimeVisualizer::Interpreted {
                    instruction_sheet: instruction_sheet1,
                    ..
                },
            ) => todo!(),
            _ => false,
        }
    }
}

impl Eq for RuntimeVisualizer {}

impl From<&StaticVisualizer> for RuntimeVisualizer {
    fn from(builtin_visualizer: &StaticVisualizer) -> Self {
        RuntimeVisualizer::Compiled(builtin_visualizer.compiled)
    }
}

impl From<StaticVisualizer> for RuntimeVisualizer {
    fn from(builtin_visualizer: StaticVisualizer) -> Self {
        RuntimeVisualizer::Compiled(builtin_visualizer.compiled)
    }
}
