#![feature(trait_upcasting)]
mod collect;
mod convert;
mod db;
mod ext;
mod specs;
#[cfg(test)]
mod tests;
mod token;

pub use db::*;
use husky_text::TextRange;
pub use specs::*;

use collect::*;
use convert::*;
use husky_entity_syn_tree::EntitySynTreeResult;
use husky_token::*;
use husky_token::{Keyword, StmtKeyword};
use husky_token_info::*;
use husky_vfs::*;
use token::*;

#[salsa::jar(db = SemanticTokenDb)]
pub struct SemanticTokenJar(semantic_tokens, semantic_tokens_ext_without_range);

#[salsa::tracked(jar = SemanticTokenJar, return_ref)]
fn semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<RangedSemanticToken>> {
    collect_semantic_tokens(db, module_path)
}

#[salsa::tracked(jar = SemanticTokenJar, return_ref)]
fn semantic_tokens_ext_without_range(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<ext::SemanticToken>> {
    let tokens = semantic_tokens(db, module_path)
        .as_ref()
        .map_err(|e| e.clone())?;
    Ok(to_semantic_tokens(tokens))
}

fn semantic_tokens_ext_within_range(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
    range: &TextRange,
) -> EntitySynTreeResult<Vec<ext::SemanticToken>> {
    let tokens = semantic_tokens(db, module_path)
        .as_ref()
        .map_err(|e| e.clone())?
        .iter()
        .filter(|token| token.range.is_within(range));
    Ok(to_semantic_tokens(tokens))
}
