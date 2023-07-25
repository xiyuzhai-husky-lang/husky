pub trait ValReprDb: salsa::DbWithJar<ValReprJar> {}

#[salsa::jar(db = ValReprDb)]
pub struct ValReprJar();
