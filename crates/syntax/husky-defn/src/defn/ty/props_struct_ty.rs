use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct RegularStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RegularStructTypeDecl,
}
