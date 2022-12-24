use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct StructTypeDecl {
    pub module_item_path: ConnectedModuleItemPath,
}
