use husky_data_viewer::DataViewerDb;
use husky_item_semantics::StoreEntityRoute;
use husky_vm::InterpreterQueryGroup;
use upcast::Upcast;

use crate::*;

impl salsa::Database for DevRuntime {}

impl InternFeature for DevRuntime {
    fn feature_interner(&self) -> &husky_val_repr::FeatureInterner {
        &self.feature_interner
    }
}

impl Upcast<dyn InstructionDb> for DevRuntime {
    fn upcast(&self) -> &(dyn InstructionDb + 'static) {
        self
    }
}

impl InterpreterQueryGroup for DevRuntime {
    fn item_opt_instruction_sheet_by_uid(
        &self,
        uid: husky_vm::EntityUid,
    ) -> Option<Arc<husky_vm::InstructionSheet>> {
        todo!()
        // let item_path = self.item_route_by_uid(uid);
        // self.item_instruction_sheet(item_path)
        // self.comptime.item_opt_instruction_sheet_by_uid(uid)
    }
}

impl Upcast<dyn InterpreterQueryGroup> for DevRuntime {
    fn upcast(&self) -> &(dyn InterpreterQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn ValReprDb> for DevRuntime {
    fn upcast(&self) -> &(dyn ValReprDb + 'static) {
        self
    }
}

impl Upcast<dyn EvalFeature<'static>> for DevRuntime {
    fn upcast(&self) -> &(dyn EvalFeature<'static> + 'static) {
        self
    }
}

impl Upcast<dyn DataViewerDb> for DevRuntime {
    fn upcast(&self) -> &(dyn DataViewerDb + 'static) {
        self
    }
}

impl EvalFeature<'static> for DevRuntime {
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
