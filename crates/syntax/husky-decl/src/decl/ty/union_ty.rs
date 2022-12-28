use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct UnionTypeDecl {
    pub path: TypePath,
}
