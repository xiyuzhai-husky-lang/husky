use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct TypePath {
    pub module: ModulePath,
    pub ident: Identifier,
}
