mod db;
mod error;
mod parser;
mod range;
mod specs;
#[cfg(test)]
mod tests;
mod xml;

pub use crate::error::{AstError, AstErrorVariant, AstResult};
pub use db::AstDb;
pub use range::*;
pub use specs::*;
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
use husky_vfs::VfsResult;
use husky_word::IdentMap;
use husky_word::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use parser::*;
use range::*;
use salsa::DbWithJar;
use std::sync::Arc;

#[salsa::jar(db = AstDb)]
pub struct AstJar(ast_sheet, ast_range_sheet);

#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Err(TokenGroupIdx, AstError),
    Mod(TokenGroupIdx),
    Use(TokenGroupIdx),
    Comment(TokenGroupIdx),
    Decor(TokenGroupIdx),
    Stmt(AstBlock),
    IfElseStmts {
        if_stmt: AstIdx,
        elif_stmts: AstIdxRange,
        else_stmt: Option<AstIdx>,
    },
    MatchStmts {
        pattern_stmt: AstIdx,
        case_stmts: AstIdxRange,
    },
    BlockDefn(AstBlock),
}

pub type AstArena = Arena<Ast>;
pub type AstIdx = ArenaIdx<Ast>;
pub type AstIdxRange = ArenaIdxRange<Ast>;
pub type AstMap<V> = ArenaMap<Ast, V>;

#[derive(Debug, PartialEq, Eq)]
pub struct AstBlock {
    head: TokenGroupIdx,
    body: AstIdxRange,
}

impl AstBlock {
    pub fn new(head: TokenGroupIdx, body: AstIdxRange) -> Self {
        Self { head, body }
    }

    pub fn head(&self) -> TokenGroupIdx {
        self.head
    }

    pub fn body_asts<'a>(&self, ast_sheet: &'a AstSheet) -> &'a [Ast] {
        &ast_sheet.arena[&self.body]
    }

    pub fn last(&self) -> Option<AstIdx> {
        if self.body.start < self.body.end {
            Some(self.body.end - 1)
        } else {
            None
        }
    }

    pub fn empty(&self) -> bool {
        self.body.start >= self.body.end
    }
}

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_sheet(db: &dyn AstDb, entity_path: EntityPath) -> VfsResult<AstSheet> {
    let token_sheet = db.token_sheet(entity_path).as_ref()?;
    Ok(AstParser::new(db.word_db(), token_sheet).parse_all())
}

#[derive(Debug, PartialEq, Eq)]
pub struct AstSheet {
    arena: AstArena,
    top_level_asts: AstIdxRange,
}

impl AstSheet {
    pub(crate) fn new(arena: AstArena, top_level_asts: AstIdxRange) -> Self {
        Self {
            arena,
            top_level_asts,
        }
    }

    pub fn indexed_asts<'a>(&'a self) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.arena.indexed_iter()
    }

    pub fn top_level_asts(&self) -> &[Ast] {
        &self.arena[&self.top_level_asts]
    }
}
