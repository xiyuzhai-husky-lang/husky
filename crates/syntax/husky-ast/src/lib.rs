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
use husky_entity_card::EntityCard;
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
    Err {
        token_group_idx: TokenGroupIdx,
        error: AstError,
    },
    Use {
        token_group_idx: TokenGroupIdx,
        accessibility: Accessibility,
    },
    Comment {
        token_group_idx: TokenGroupIdx,
    },
    Decor {
        token_group_idx: TokenGroupIdx,
    },
    Stmt {
        token_group_idx: TokenGroupIdx,
        body: AstIdxRange,
    },
    IfElseStmts {
        if_stmt: AstIdx,
        elif_stmts: AstIdxRange,
        else_stmt: Option<AstIdx>,
    },
    MatchStmts {
        pattern_stmt: AstIdx,
        case_stmts: AstIdxRange,
    },
    Defn {
        token_group_idx: TokenGroupIdx,
        body: AstIdxRange,
        accessibility: Accessibility,
        entity_card: EntityCard,
        ident: Identifier,
        is_generic: bool,
        body_kind: DefnBodyKind,
    },
    Impl {
        token_group_idx: TokenGroupIdx,
        body: AstIdxRange,
    },
    Main {
        token_group_idx: TokenGroupIdx,
        body: AstIdxRange,
    },
    Config {
        token_group_idx: TokenGroupIdx,
        body: AstIdxRange,
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

#[test]
fn ast_sheet_works() {
    use tests::*;
    DB::expect_test_modules("ast_sheet", AstDb::ast_sheet);
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

    pub fn top_level_asts(&self) -> &AstIdxRange {
        &self.top_level_asts
    }

    pub fn top_level_asts_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.arena[&self.top_level_asts]
            .iter()
            .enumerate()
            .map(|(i, ast)| (self.top_level_asts.start() + i, ast))
    }
}

impl std::ops::Deref for AstSheet {
    type Target = AstArena;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}
