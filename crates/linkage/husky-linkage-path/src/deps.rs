use crate::*;

#[salsa::interned(db = LinkagePathDb, jar = LinkagePathJar)]
pub struct LinkageDeps {}
