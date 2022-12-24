use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct RecordTypeDecl {
    pub module_item_path: ConnectedModuleItemPath,
}
