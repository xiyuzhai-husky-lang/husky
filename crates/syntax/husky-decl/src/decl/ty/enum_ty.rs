use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct EnumTypeDecl {
    pub path: TypePath,
}
