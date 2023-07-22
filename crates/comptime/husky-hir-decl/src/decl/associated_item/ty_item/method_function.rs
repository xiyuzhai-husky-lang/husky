use super::*;

// functions are called in functional style, i.e. without parentheses
#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMethodFunctionHirDecl {}
