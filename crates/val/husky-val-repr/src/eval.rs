use crate::*;
use husky_task::{IsTask, Value};
use husky_val::ValOpr;

impl ValRepr {
    pub fn eval<Task: IsTask>(self, db: &dyn ValReprDb) -> Value<Task> {
        match self.opr(db) {
            ValOpr::Fugitive(_) => todo!(),
            ValOpr::Require => todo!(),
        }
    }
}
