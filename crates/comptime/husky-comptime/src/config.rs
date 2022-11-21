use husky_linkage_table::LinkageTableConfig;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ComptimeConfig {
    pub package_dir: PathBuf,
    pub __resolve_root_defn: fn(
        ident: husky_identifier::RootBuiltinIdentifier,
    ) -> &'static husky_static_defn::EntityStaticDefn,
    pub linkage_table: LinkageTableConfig,
}
