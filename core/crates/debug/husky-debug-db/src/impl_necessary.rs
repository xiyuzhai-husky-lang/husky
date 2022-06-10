use runtime_db::AskRuntime;

use crate::*;

impl ProduceTrace for HuskyDebugTime {
    fn trace_factory(&self) -> &TraceFactory {
        todo!()
    }
}

impl AskRuntime for HuskyDebugTime {
    fn runtime(&self) -> &runtime_db::HuskyRuntime {
        todo!()
    }
}
