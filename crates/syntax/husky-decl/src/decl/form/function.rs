use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FunctionDecl {
    pub module_item_path: ModuleItemPath,
}
