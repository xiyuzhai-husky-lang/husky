use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct MorphismDecl {
    pub module_item_path: ModuleItemPath,
}
