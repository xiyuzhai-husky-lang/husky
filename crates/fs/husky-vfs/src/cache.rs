use crate::*;

#[derive(Default)]
pub struct HuskyFileCache(pub(crate) DashMap<PathBufItd, HuskyFileId>);
