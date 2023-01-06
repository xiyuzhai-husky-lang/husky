use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeDecl,
}
