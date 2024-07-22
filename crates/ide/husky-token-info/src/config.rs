use crate::*;
use husky_vfs::path::module_path::ModulePath;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenInfoConfig {}

#[salsa::tracked(return_ref)]
pub(crate) fn token_info_config(db: &::salsa::Db, module_path: ModulePath) -> TokenInfoConfig {
    // ad hoc
    // todo: implement read from file
    TokenInfoConfig {}
}

fn read_token_info_config_from_file(file_path: &Path) -> TokenInfoConfig {
    todo!()
}
