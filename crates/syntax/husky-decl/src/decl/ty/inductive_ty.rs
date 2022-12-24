use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct InductiveTypeDecl {
    pub module_item_path: ConnectedModuleItemPath,
}
