use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct ConstantDecl {
    pub module_item_path: ModuleItemPath,
}
