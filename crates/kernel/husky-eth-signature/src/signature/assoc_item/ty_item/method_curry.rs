use super::*;

// functions are called in functional style, i.e. without parentheses
#[salsa::interned]
pub struct TypeMethodCurryEthTemplate {
    pub path: TypeItemPath,
}
