use super::*;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermComposition {
    f: Term,
    g: Term,
}
