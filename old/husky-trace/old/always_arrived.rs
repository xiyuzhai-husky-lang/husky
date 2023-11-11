// use crate::*;

// impl TraceVariant {
//     pub fn opt_arrival_indicator(&self) -> Option<&ValDomain> {
//         match self {
//             TraceVariant::Main(..) => None,
//             TraceVariant::Module { .. } => None,
//             TraceVariant::EntityVal { .. } => None,
//             TraceVariant::ValStmt(stmt) => todo!(),
//             // stmt.opt_arrival_indicator.as_ref(),
//             TraceVariant::ValBranch(branch) => todo!(),
//             //  branch.opt_arrival_indicator.as_ref(),
//             TraceVariant::LazyExpr(expr) => todo!(),
//             // expr.opt_arrival_indicator.as_ref(),
//             TraceVariant::ValCallArgument { .. } => None,
//             TraceVariant::EagerStmt { .. } => None, // could be changed in the future
//             TraceVariant::EagerBranch { .. } => None, // could be changed in the future
//             TraceVariant::LoopFrame { .. } => None, // could be changed in the future
//             TraceVariant::EagerExpr { .. } => None, // could be changed in the future
//             TraceVariant::EagerCallArgument { .. } => None,
//             TraceVariant::CallHead { .. } => None,
//         }
//     }
// }
