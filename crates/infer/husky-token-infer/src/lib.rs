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
use husky_entity_tree::*;
use husky_token::*;
use husky_vfs::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db =  TokenInferDb)]
pub struct TokenInferJar(token_infer_sheet);

#[salsa::tracked(jar = TokenInferJar, return_ref)]
fn token_infer_sheet(
    db: &dyn TokenInferDb,
    module_path: ModulePath,
) -> EntityTreeResult<TokenInferSheet> {
    Ok(TokenInferEngine::new(db, module_path)?.infer_all())
}
