mod context;
mod db;
mod entrance;
mod error;
mod field;
mod parser;
mod sheet;
mod stmt;
#[cfg(test)]
mod tests;
mod variant;
mod xml;

pub use crate::error::{AstError, AstErrorVariant, AstResult, AstResultArc};
pub use context::*;
pub use db::AstDb;
pub use entrance::*;
pub use field::*;
pub use sheet::*;
pub use stmt::*;
pub use variant::*;
pub use xml::*;

use error::*;
use husky_check_utils::*;
use husky_defn_head::*;
use husky_dev_utils::*;
use husky_entity_kind::*;
use husky_entity_path::EntityPath;
use husky_expr::*;
use husky_init_syntax::InitKind;
use husky_opn_syntax::*;
use husky_pattern_syntax::RawPattern;
use husky_print_utils::*;
use husky_term::Term;
use husky_text::*;
use husky_token::TokenGroupIdx;
use husky_word::IdentMap;
use husky_word::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use parser::*;
use salsa::DbWithJar;
use std::sync::Arc;

#[salsa::jar(db = AstDb)]
pub struct AstJar(ast_sheet);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeprecatedAst {
    pub variant: DeprecatedAstVariant,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Err(AstError),
    Mod,
    Use,
    Comment,
    Decor,
    SimpleStmt,
    BlockStmt(AstIdxRange),
    IfElseStmts(AstIdxRange),
    MatchStmts(AstIdxRange),
    Defn(AstIdxRange),
}

pub type AstArena = Arena<Ast>;
pub type AstIdx = ArenaIdx<Ast>;
pub type AstIdxRange = ArenaIdxRange<Ast>;
pub type AstMap<V> = ArenaMap<Ast, V>;
