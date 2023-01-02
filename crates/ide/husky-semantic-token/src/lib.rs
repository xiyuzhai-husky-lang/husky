mod collect;
mod convert;
mod db;
mod specs;
#[cfg(test)]
mod tests;
mod token;

pub use db::*;
pub use specs::*;

use collect::*;
use convert::*;
use husky_entity_tree::EntityTreeResult;
use husky_token::*;
use husky_token_info::*;
use husky_vfs::*;
use token::*;

use husky_token::{Keyword, StmtKeyword};
mod ext;

#[salsa::jar(db = SemanticTokenDb)]
pub struct SemanticTokenJar(semantic_tokens, semantic_tokens_ext);

#[salsa::tracked(jar = SemanticTokenJar, return_ref)]
fn semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<RangedSemanticToken>> {
    collect_semantic_tokens(db, module_path)
}

#[salsa::tracked(jar = SemanticTokenJar, return_ref)]
fn semantic_tokens_ext(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<ext::SemanticToken>> {
    let tokens = semantic_tokens(db, module_path)
        .as_ref()
        .map_err(|e| e.clone())?;
    Ok(to_semantic_tokens(tokens))
}
