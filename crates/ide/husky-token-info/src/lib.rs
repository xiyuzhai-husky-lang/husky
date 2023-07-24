#![feature(trait_upcasting)]
mod db;
mod engine;
mod info;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use info::*;
pub use sheet::*;

use engine::*;
use husky_item_tree::*;
use husky_token::*;
use husky_vfs::*;

#[salsa::jar(db =  TokenInfoDb)]
pub struct TokenInfoJar(token_info_sheet);

#[salsa::tracked(jar = TokenInfoJar, return_ref)]
fn token_info_sheet(
    db: &dyn TokenInfoDb,
    module_path: ModulePath,
) -> ItemSynTreeResult<TokenInfoSheet> {
    InferEngine::new(db, module_path)?.visit_all()
}
