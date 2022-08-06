use husky_linkage_table::LinkageTableConfig;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct HuskyCompileTimeConfig {
    pub package_dir: PathBuf,
    pub __resolve_root_defn:
        fn(ident: husky_word::RootIdentifier) -> &'static static_defn::EntityStaticDefn,
    pub linkage_table: LinkageTableConfig,
}

impl HuskyCompileTimeConfig {
    pub fn target_entrance(&self) -> PathBuf {
        todo!()
    }
}
