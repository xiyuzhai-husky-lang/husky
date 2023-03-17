use super::*;

#[salsa::interned(db = TomlAstDb, jar = TomlAstJar)]
pub struct TomlSectionTitle {
    pub words: SmallVec<[Word; 2]>,
}
