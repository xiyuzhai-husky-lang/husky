use super::*;

#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TermPlace {}
