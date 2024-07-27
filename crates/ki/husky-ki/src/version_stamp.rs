use crate::*;

// ad hoc
#[salsa::interned(db = ValDb, jar = KiJar)]
pub struct KiVersionStamp {}

impl Ki {
    pub fn version_stamp(self, db: &::salsa::Db) -> KiVersionStamp {
        KiVersionStamp::new(db)
    }
}

impl KiDomain {
    pub fn version_stamp(self, db: &::salsa::Db) -> KiVersionStamp {
        KiVersionStamp::new(db)
    }
}
