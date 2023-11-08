use crate::*;

#[salsa::interned(db = ValDb, jar = ValJar)]
pub struct ValDeps {}

impl Val {
    pub fn deps(self, db: &dyn ValDb) -> ValDeps {
        todo!()
    }
}
