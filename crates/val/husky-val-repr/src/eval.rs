use crate::*;
use husky_task::{IsTask, Value};
use husky_val::ValOpn;

impl ValRepr {
    pub fn eval<Task: IsTask>(self, db: &dyn ValReprDb) -> Value<Task> {
        match self.opr(db) {
            ValOpn::ValItem(_) => todo!(),
            ValOpn::Require => todo!(),
            ValOpn::Linkage(_) => todo!(),
            ValOpn::FunctionGn(_) => todo!(),
            ValOpn::Prefix(_) => todo!(),
            ValOpn::Suffix(_) => todo!(),
            ValOpn::Binary(_) => todo!(),
        }
    }
}
