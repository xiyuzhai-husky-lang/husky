use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TraitDecl {
    #[id]
    pub module_item_path: ModuleItemPath,
}
