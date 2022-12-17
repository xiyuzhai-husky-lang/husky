mod db;
mod error;
mod parser;
mod range;
mod specs;
#[cfg(test)]
mod tests;

pub use crate::error::{AstError, AstErrorVariant, AstResult};
pub use db::AstDb;
use husky_accessibility::Accessibility;
use husky_entity_kind::EntityKind;
pub use range::*;
pub use specs::*;

use error::*;
use husky_entity_path::EntityPath;
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
    Use {
        token_group: TokenGroupIdx,
    },
    Comment(TokenGroupIdx),
    Decor(TokenGroupIdx),
    Stmt {
        token_group: TokenGroupIdx,
        body: Option<AstIdxRange>,
    },
    IfElseStmts {
        if_stmt: AstIdx,
        elif_stmts: Option<AstIdxRange>,
        else_stmt: Option<AstIdx>,
    },
    MatchStmts {
        pattern_stmt: AstIdx,
        case_stmts: Option<AstIdxRange>,
    },
    Defn {
        token_group: TokenGroupIdx,
        body: Option<AstIdxRange>,
        accessibility: Accessibility,
        entity_kind: EntityKind,
        ident: Identifier,
        is_generic: bool,
        body_kind: DefnBodyKind,
    },
    Impl {
        token_group: TokenGroupIdx,
        body: Option<AstIdxRange>,
    },
    Main {
        token_group: TokenGroupIdx,
        body: Option<AstIdxRange>,
    },
    Config {
        token_group: TokenGroupIdx,
        body: Option<AstIdxRange>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DefnBodyKind {
    None,
    Block,
    Cases,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplAstKind {
    TypeImpl,
    TraitImpl,
}

pub type AstArena = Arena<Ast>;
pub type AstIdx = ArenaIdx<Ast>;
pub type AstIdxRange = ArenaIdxRange<Ast>;
pub type AstMap<V> = ArenaMap<Ast, V>;

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_sheet(db: &dyn AstDb, entity_path: EntityPath) -> VfsResult<AstSheet> {
    let token_sheet = db.token_sheet(entity_path).as_ref()?;
    Ok(AstParser::new(db.word_db(), token_sheet).parse_all())
}

#[derive(Debug, PartialEq, Eq)]
pub struct AstSheet {
    arena: AstArena,
    top_level_asts: Option<AstIdxRange>,
}

impl AstSheet {
    pub(crate) fn new(arena: AstArena, top_level_asts: Option<AstIdxRange>) -> Self {
        Self {
            arena,
            top_level_asts,
        }
    }

    pub fn indexed_asts<'a>(&'a self) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.arena.indexed_iter()
    }

    pub fn top_level_asts<'a>(&'a self) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.top_level_asts
            .iter()
            .map(|range| {
                self.arena[range]
                    .iter()
                    .enumerate()
                    .map(|(i, ast)| (range.start + i, ast))
            })
            .flatten()
    }
}
