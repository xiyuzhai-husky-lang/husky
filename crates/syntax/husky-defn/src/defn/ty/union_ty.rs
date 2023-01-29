use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnionTypeDecl,
}
