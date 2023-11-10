use crate::*;
use husky_task::{IsTask, Value};
use husky_val::ValOpn;

impl ValRepr {
    pub fn eval<Task: IsTask>(self, db: &dyn ValReprDb) -> (ValControlFlow, Value<Task>) {
        match self.opn(db) {
            ValOpn::Literal(_) => todo!(),
            ValOpn::ValItem(_) => todo!(),
            ValOpn::Require => todo!(),
            ValOpn::Linkage(_) => todo!(),
            ValOpn::FunctionGn(_) => todo!(),
            ValOpn::Prefix(_) => todo!(),
            ValOpn::Suffix(_) => todo!(),
            ValOpn::Binary(_) => todo!(),
            ValOpn::EvalDiscarded => todo!(),
            ValOpn::NewList => todo!(),
        }
    }
}

pub enum ValControlFlow {
    Continue,
    Break,
    Return,
}
