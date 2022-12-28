use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct FormPath {
    pub module: ModulePath,
    pub ident: Identifier,
}
