use super::*;
use crate::*;

use husky_print_utils::msg_once;

impl<'a> Evaluator<'a> {
    pub(crate) fn eval_lazy_block(&self, block: &ValBlock) -> VMResult<RegularValue> {
        todo!()
        // self.cache(EvalKey::Feature(block.feature), |this: &Self| {
        //     for stmt in block.stmts.iter() {
        //         let value = this.eval_stmt(stmt)?;
        //         match value.data_kind() {
        //             __RegisterDataKind::Unreturned => (),
        //             _ => return Ok(value),
        //         }
        //     }
        //     Ok(__RegularValue::unreturned())
        // })
    }

    pub(crate) fn eval_val_block(&self, block: ValBlock) -> VMResult<RegularValue> {
        todo!()
        // let arguments = match block.opt_this {
        //     Some(ref this_repr) => {
        //         vec![self.eval_feature_repr(this_repr)]
        //     }
        //     None => vec![],
        // };
        // let nargs: u8 = arguments.len().try_into().unwrap();
        // msg_once!("kwargs");
        // assert!(block.opt_linkage.is_some());
        // let result = husky_vm::eval_fast(
        //     self.db.upcast(),
        //     unsafe { self.some_ctx() },
        //     Some(&block.instruction_sheet),
        //     block.opt_linkage,
        //     arguments.into_iter(),
        //     [].into_iter(),
        //     nargs,
        //     &self.evaluator_config.vm,
        // );
        // result
    }

    // pub(crate) fn eval_proc_block(&self, block: &FeatureProcBody) -> __VMResult<__RegularValue> {
    //     let arguments = match block.opt_this {
    //         Some(ref this_repr) => {
    //             vec![self.eval_feature_repr(this_repr)]
    //         }
    //         None => vec![],
    //     };
    //     let nargs: u8 = arguments.len().try_into().unwrap();
    //     msg_once!("kwargs");
    //     // remove this once hot reload is implemented
    //     assert!(block.opt_linkage.is_some());
    //     let result = husky_vm::eval_fast(
    //         self.db.upcast(),
    //         unsafe { self.some_ctx() },
    //         Some(&block.instruction_sheet),
    //         block.opt_linkage,
    //         arguments.into_iter(),
    //         [].into_iter(),
    //         nargs,
    //         &self.evaluator_config.vm,
    //     );
    //     result
    // }
}
