use husky_entity_semantics::StoreEntityRoute;
use upcast::Upcast;
use vm::InterpreterQueryGroup;

use crate::*;

impl salsa::Database for HuskyEvalTime {}

impl AskCompileTime for HuskyEvalTime {
    fn compile_time(&self) -> &HuskyCompileTime {
        &self.compile_time
    }
}

// impl ProduceTrace  for HuskyEvalTime {
//     fn trace_factory(&self) -> &trace::TraceFactory<'static> {
//         &self.trace_factory
//     }
// }
impl AllocateUniqueFeature for HuskyEvalTime {
    fn feature_interner(&self) -> &husky_feature_gen::FeatureInterner {
        &self.feature_interner
    }
}

impl Upcast<dyn InstructionGenQueryGroup> for HuskyEvalTime {
    fn upcast(&self) -> &(dyn InstructionGenQueryGroup + 'static) {
        self
    }
}

impl InterpreterQueryGroup for HuskyEvalTime {
    fn entity_opt_instruction_sheet_by_uid(
        &self,
        uid: vm::EntityUid,
    ) -> Option<Arc<vm::InstructionSheet>> {
        let entity_route = self.compile_time.entity_route_by_uid(uid);
        self.entity_instruction_sheet(entity_route)
        // self.compile_time.entity_opt_instruction_sheet_by_uid(uid)
    }
}

impl Upcast<dyn InterpreterQueryGroup> for HuskyEvalTime {
    fn upcast(&self) -> &(dyn InterpreterQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn FeatureGenQueryGroup> for HuskyEvalTime {
    fn upcast(&self) -> &(dyn FeatureGenQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn EvalFeature<'static>> for HuskyEvalTime {
    fn upcast(&self) -> &(dyn EvalFeature<'static> + 'static) {
        self
    }
}

impl EvalFeature<'static> for HuskyEvalTime {
    fn session(&self) -> &Session<'static> {
        match self.variant {
            HuskyEvalTimeVariant::None => todo!(),
            HuskyEvalTimeVariant::Learning { ref session } => session,
        }
    }

    fn verbose(&self) -> bool {
        self.config.verbose
    }

    fn opt_static_husky_feature_eval(&self) -> Option<&dyn EvalFeature<'static>> {
        Some(self)
    }
}
