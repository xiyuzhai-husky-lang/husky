use husky_data_viewer::HuskyDataViewerQueryGroup;
use husky_entity_semantics::StoreEntityRoute;
use husky_vm::InterpreterQueryGroup;
use upcast::Upcast;

use crate::*;

impl salsa::Database for HuskyRuntime {}

impl AskCompileTime for HuskyRuntime {
    fn comptime(&self) -> &HuskyComptime {
        &self.comptime
    }
}

impl InternFeature for HuskyRuntime {
    fn feature_interner(&self) -> &husky_feature_gen::FeatureInterner {
        &self.feature_interner
    }
}

impl Upcast<dyn InstructionGenQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn InstructionGenQueryGroup + 'static) {
        self
    }
}

impl InterpreterQueryGroup for HuskyRuntime {
    fn entity_opt_instruction_sheet_by_uid(
        &self,
        uid: husky_vm::EntityUid,
    ) -> Option<Arc<husky_vm::InstructionSheet>> {
        let entity_route = self.comptime.entity_route_by_uid(uid);
        self.entity_instruction_sheet(entity_route)
        // self.comptime.entity_opt_instruction_sheet_by_uid(uid)
    }
}

impl Upcast<dyn InterpreterQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn InterpreterQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn FeatureGenQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn FeatureGenQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn EvalFeature<'static>> for HuskyRuntime {
    fn upcast(&self) -> &(dyn EvalFeature<'static> + 'static) {
        self
    }
}

impl Upcast<dyn HuskyDataViewerQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn HuskyDataViewerQueryGroup + 'static) {
        self
    }
}

impl EvalFeature<'static> for HuskyRuntime {
    fn session(&self) -> &Session<'static> {
        match self.variant {
            HuskyRuntimeVariant::None => todo!(),
            HuskyRuntimeVariant::Learning { ref session } => session,
        }
    }

    fn evaluator_config(&self) -> &EvaluatorConfig {
        &self.config.evaluator
    }

    fn opt_static_husky_feature_eval(&self) -> Option<&dyn EvalFeature<'static>> {
        Some(self)
    }
}
