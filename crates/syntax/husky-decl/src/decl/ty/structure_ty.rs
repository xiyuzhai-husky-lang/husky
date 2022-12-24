use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct StructureTypeDecl {
    pub module_item_path: ConnectedModuleItemPath,
}
