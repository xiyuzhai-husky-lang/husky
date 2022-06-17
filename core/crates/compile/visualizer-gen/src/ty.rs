use semantics_eager::FuncStmtVariant;

use crate::*;

pub(crate) fn infer_visual_ty(stmts: &[Arc<FuncStmt>]) -> VisualTy {
    match stmts.last().unwrap().variant {
        FuncStmtVariant::Init {
            varname,
            ref initial_value,
        } => todo!(),
        FuncStmtVariant::Assert { ref condition } => todo!(),
        FuncStmtVariant::Return { ref result } => todo!(),
        FuncStmtVariant::ReturnXml { ref xml_expr } => todo!(),
        FuncStmtVariant::ConditionFlow { ref branches } => todo!(),
        FuncStmtVariant::Match {
            ref match_expr,
            ref branches,
        } => todo!(),
    }
}
