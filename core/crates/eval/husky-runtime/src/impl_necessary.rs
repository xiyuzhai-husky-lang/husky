use eval_feature::FeatureEvalQueryGroup;
use upcast::Upcast;
use vm::InterpreterQueryGroup;

use crate::*;

impl AskCompileTime for HuskyRuntime {
    fn compile_time(&self) -> &HuskyCompileTime {
        &self.compile_time
    }
}

// impl ProduceTrace  for HuskyRuntime {
//     fn trace_factory(&self) -> &trace::TraceFactory<'static> {
//         &self.trace_factory
//     }
// }

impl FeatureEvalQueryGroup for HuskyRuntime {}

impl InterpreterQueryGroup for HuskyRuntime {
    fn entity_opt_instruction_sheet_by_uid(
        &self,
        uid: vm::EntityUid,
    ) -> Option<Arc<vm::InstructionSheet>> {
        self.compile_time.entity_opt_instruction_sheet_by_uid(uid)
    }

    fn visualize<'temp, 'eval>(
        &self,
        ty: entity_route::EntityRoutePtr,
        value: &(dyn AnyValueDyn<'eval> + 'temp),
    ) -> VisualData {
        let visualizer = self.compile_time().visualizer(ty);
        visualizer.visualize(self.compile_time(), value, self.verbose())
    }
}

impl Upcast<dyn InterpreterQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn InterpreterQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn FeatureEvalQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn FeatureEvalQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn EvalFeature<'static>> for HuskyRuntime {
    fn upcast(&self) -> &(dyn EvalFeature<'static> + 'static) {
        self
    }
}

impl EvalFeature<'static> for HuskyRuntime {
    fn session(&self) -> &Session<'static> {
        &self.session
    }

    fn verbose(&self) -> bool {
        self.config.verbose
    }
}
