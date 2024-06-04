pub mod db;
mod engine;
mod info;
mod sheet;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::info::*;
pub use self::sheet::*;

use engine::*;
use husky_entity_tree::error::*;
use husky_token::*;
use husky_vfs::path::module_path::ModulePath;

#[salsa::tracked(jar = TokenInfoJar, return_ref)]
fn token_info_sheet(db: &::salsa::Db, module_path: ModulePath) -> EntityTreeResult<TokenInfoSheet> {
    TokenInfoEngine::new(db, module_path)?.visit_all()
}
