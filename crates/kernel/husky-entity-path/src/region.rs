use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntityPathDb)]
pub enum RegionPath {
    Snippet(ModulePath),
    Decl(ItemPath),
    Defn(ItemPath),
}
