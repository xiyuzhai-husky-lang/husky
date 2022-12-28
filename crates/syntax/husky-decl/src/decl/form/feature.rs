use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FeatureDecl {
    pub module_item_path: ModuleItemPath,
}
