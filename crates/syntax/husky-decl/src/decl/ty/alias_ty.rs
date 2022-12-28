use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct AliasTypeDecl {
    pub module_item_path: ModuleItemPath,
}
