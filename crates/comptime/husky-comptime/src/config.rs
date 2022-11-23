use husky_linkage_table::LinkageTableConfig;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ComptimeConfig {
    pub package_dir: PathBuf,
    pub linkage_table: LinkageTableConfig,
}
