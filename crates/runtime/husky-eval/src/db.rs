pub trait EvalDb: salsa::DbWithJar<EvalJar> + husky_val_repr::db::ValReprDb {}

impl<Db> EvalDb for Db where Db: salsa::DbWithJar<EvalJar> + husky_val_repr::db::ValReprDb {}

#[salsa::jar(db = EvalDb)]
pub struct EvalJar();
