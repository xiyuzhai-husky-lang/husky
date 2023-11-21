use crate::*;

#[salsa::interned(db = LinkageDb, jar = LinkageJar)]
pub struct LinkageVersionStamp {}

impl Linkage {
    pub fn version_stamp(self, db: &dyn LinkageDb) -> LinkageVersionStamp {
        todo!()
    }
}
