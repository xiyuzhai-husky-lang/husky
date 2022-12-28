use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct TraitPath {
    pub module: ModulePath,
    pub ident: Identifier,
}
