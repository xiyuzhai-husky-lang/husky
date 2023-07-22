use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TupleStructHirDecl {}
