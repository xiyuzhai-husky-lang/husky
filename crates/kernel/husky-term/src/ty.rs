use crate::*;

#[salsa::interned(jar= TermJar)]
pub struct Type {
    term: Term,
}
