use crate::*;
use husky_vm::InterpreterQueryGroup;

pub trait ValReprDb: salsa::DbWithJar<ValReprJar> + InterpreterQueryGroup {}

impl<Db> ValReprDb for Db where Db: salsa::DbWithJar<ValReprJar> + InterpreterQueryGroup {}

#[salsa::jar(db = ValReprDb)]
pub struct ValReprJar(ValDomain, ValExpr, ValStmt, ValBranch, ValBlock);
