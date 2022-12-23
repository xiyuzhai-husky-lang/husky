mod db;
mod old;
mod specs;
#[cfg(test)]
mod tests;

pub use db::*;
pub use specs::*;

use husky_token::*;
use husky_token_infer::*;
use husky_vfs::*;

use husky_token::{Keyword, StmtKeyword};
use lsp_types::{
    Range, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensEdit,
};

#[salsa::jar(db = SemanticTokenDb)]
pub struct SemanticTokenJar();
