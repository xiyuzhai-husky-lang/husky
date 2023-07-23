use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct PropsStructTypeHirDefn {
    #[id]
    pub path: TypePath,
    pub decl: PropsStructTypeHirDecl,
}
