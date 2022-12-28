use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct ConstantDecl {
    #[id]
    pub path: FormPath,
}
