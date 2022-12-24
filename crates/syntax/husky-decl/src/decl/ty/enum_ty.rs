use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct EnumTypeDecl {
    pub module_item_path: ConnectedModuleItemPath,
}
