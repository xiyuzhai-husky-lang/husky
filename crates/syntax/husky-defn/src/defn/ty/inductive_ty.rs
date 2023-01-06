use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: InductiveTypeDecl,
}
