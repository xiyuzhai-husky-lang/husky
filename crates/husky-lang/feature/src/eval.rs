use common::*;
use vm::{PrimitiveValue, VMError};

use crate::{sheet::FeatureSheet, *};
use vm::{Conditional, EvalValue, StackValue};

use crate::*;

pub struct Evaluator<'a, 'eval: 'a> {
    input: EvalValue<'eval, 'eval>,
    features: &'a FeatureUniqueAllocator,
    sheet: &'a mut FeatureSheet<'eval>,
}

impl<'a, 'eval: 'a> Evaluator<'a, 'eval> {
    pub(super) fn new(
        input: EvalValue<'eval, 'eval>,
        features: &'a FeatureUniqueAllocator,
        sheet: &'a mut FeatureSheet<'eval>,
    ) -> Self {
        Self {
            input,
            sheet,
            features,
        }
    }

    // pub(super) fn _eval(&self, cached_ptr: FeaturePtr) -> EvalValue<'eval, 'eval> {
    //     match *cached_ptr.ptr {
    //         Feature::Input => todo!(),
    //         Feature::Literal(literal) => Ok(Conditional::Defined(StackValue::Primitive(literal))),
    //         Feature::Assert { condition } => match self.eval(condition) {
    //             Ok(_) => Ok(Conditional::Undefined),
    //             Err(_) => todo!(),
    //         },
    //         Feature::Do { first, then } => match self.eval(first)? {
    //             Conditional::Defined(value) => Ok(Conditional::Defined(value)),
    //             Conditional::Undefined => self.eval(then),
    //         },
    //         Feature::PrimitiveBinaryFunc { func, lopd, ropd } => Ok(Conditional::Defined(
    //             StackValue::Primitive(func.act_on_primitives(
    //                 self.eval(lopd)?.defined()?.as_primitive()?,
    //                 self.eval(ropd)?.defined()?.as_primitive()?,
    //             )?),
    //         )),
    //     }
    // }

    fn eval_expr(&mut self, expr: &FeatureExpr) -> EvalValue<'eval, 'eval> {
        todo!()
    }

    fn eval_stmt(&mut self, stmt: &FeatureStmt) -> EvalValue<'eval, 'eval> {
        match stmt.kind {
            stmt::FeatureStmtKind::Init { .. } => Ok(Conditional::Undefined),
            stmt::FeatureStmtKind::Assert { ref condition } => {
                let satisfied: bool = match self.eval_expr(condition)?.defined()? {
                    StackValue::Primitive(value) => match value {
                        PrimitiveValue::I32(_) => todo!(),
                        PrimitiveValue::F32(_) => todo!(),
                        PrimitiveValue::B32(_) => todo!(),
                        PrimitiveValue::B64(_) => todo!(),
                        PrimitiveValue::Bool(b) => *b,
                        PrimitiveValue::Void => todo!(),
                    },
                    _ => todo!(),
                };
                if satisfied {
                    Ok(Conditional::Undefined)
                } else {
                    Err(VMError::AssertionFailure)
                }
            }
            stmt::FeatureStmtKind::Return { ref result } => todo!(),
        }
    }

    pub(crate) fn eval_block(&mut self, block: &FeatureBlock) -> EvalValue<'eval, 'eval> {
        for stmt in block.stmts.iter() {
            let value = self.eval_stmt(stmt)?;
            match value {
                Conditional::Defined(_) => return Ok(value),
                Conditional::Undefined => (),
            }
        }
        Ok(Conditional::Undefined)
    }
}
