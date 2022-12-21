use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct ModuleItemPath {
    module: ModulePath,
    ident: Identifier,
}
