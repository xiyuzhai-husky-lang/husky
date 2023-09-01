use husky_vm::{RegularValue, VMResult, __VMError};

use crate::*;

use super::Evaluator;

impl<'a> Evaluator<'a> {
    pub(crate) fn eval_stmt(&self, stmt: &ValStmt) -> VMResult<RegularValue> {
        todo!()
        // match stmt.variant {
        //     ValStmtData::Init { .. } => Ok(__RegularValue::unreturned()),
        //     ValStmtData::Assert { ref condition } => {
        //         if self.satisfies(condition)? {
        //             Ok(__RegularValue::unreturned())
        //         } else {
        //             Err(__VMError::new_normal(format!(
        //                 "assertion failed at {:?}:{:?}",
        //                 stmt.file, stmt.range.start.row
        //             )))
        //         }
        //     }
        //     ValStmtData::Require { ref condition, .. } => {
        //         if self.satisfies(condition)? {
        //             Ok(__RegularValue::unreturned())
        //         } else {
        //             Ok(__RegularValue::none(0))
        //         }
        //     }
        //     ValStmtData::Return { ref result } => self.eval_expr(result),
        //     ValStmtData::ReturnUnveil {
        //         ref result,
        //         implicit_conversion,
        //         ..
        //     } => self.eval_expr(result).map(|v| {
        //         v.unveil()
        //             .map(|v| match implicit_conversion {
        //                 ImplicitConversion::None => v,
        //                 ImplicitConversion::WrapInSome { number_of_somes } => {
        //                     v.wrap_in_some(number_of_somes)
        //                 }
        //                 ImplicitConversion::ConvertToBool => todo!(),
        //             })
        //             .unwrap_or(__RegularValue::unreturned())
        //     }),
        //     ValStmtData::ReturnHtml { ref result } => self.eval_xml_expr(result),
        //     ValStmtData::ConditionFlow { ref branches, .. } => {
        //         for branch in branches {
        //             let execute_branch: bool = match branch.variant {
        //                 FeatureLazyBranchVariant::If { ref condition } => {
        //                     self.satisfies(condition)?
        //                 }
        //                 FeatureLazyBranchVariant::Elif { ref condition } => {
        //                     self.satisfies(condition)?
        //                 }
        //                 FeatureLazyBranchVariant::Else => true,
        //             };
        //             if execute_branch {
        //                 return self.eval_lazy_block(&branch.block);
        //             }
        //         }
        //         Ok(__RegularValue::unreturned())
        //     }
        // }
    }

    fn satisfies(&self, condition: ValExpr) -> VMResult<bool> {
        todo!()
        // let value = self.eval_expr_cached(condition)?;
        // Ok(value.to_bool())
    }
}
