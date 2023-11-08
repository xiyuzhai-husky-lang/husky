use crate::*;
use husky_val::ValDb;
use husky_vm::InterpreterQueryGroup;

pub trait ValReprDb: salsa::DbWithJar<ValReprJar> + ValDb {}

impl<Db> ValReprDb for Db where Db: salsa::DbWithJar<ValReprJar> + ValDb {}

#[salsa::jar(db = ValReprDb)]
pub struct ValReprJar(ValRepr, val_repr_expansion);
