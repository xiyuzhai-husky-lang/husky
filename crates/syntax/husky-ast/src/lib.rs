mod db;
mod debug;
mod error;
mod parser;
mod range;
mod specs;
#[cfg(test)]
mod tests;

pub use crate::error::{AstError, AstErrorVariant, AstResult};
pub use db::AstDb;
use husky_entity_card::EntityCard;
pub use range::*;
pub use specs::*;

use husky_entity_path::{Accessibility, EntityPath};

use husky_text::*;
use husky_token::TokenGroupIdx;
use husky_vfs::*;

use husky_word::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use parser::*;

use salsa::DbWithJar;

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
    Ok(AstParser::new(token_sheet).parse_all())
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

impl<Db: AstDb> salsa::DebugWithDb<Db> for Ast {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Ast::Err {
                token_group_idx,
                error,
            } => f
                .debug_struct("Err")
                .field("token_group_idx", token_group_idx)
                .field("error", error)
                .finish(),
            Ast::Use {
                token_group_idx,
                accessibility,
            } => f
                .debug_struct("Use")
                .field("token_group_idx", token_group_idx)
                .field("accessibility", accessibility)
                .finish(),
            Ast::Comment { token_group_idx } => f
                .debug_struct("Comment")
                .field("token_group_idx", token_group_idx)
                .finish(),
            Ast::Decor { token_group_idx } => f
                .debug_struct("Decor")
                .field("token_group_idx", token_group_idx)
                .finish(),
            Ast::Stmt {
                token_group_idx,
                body,
            } => f
                .debug_struct("Stmt")
                .field("token_group_idx", token_group_idx)
                .field("body", body)
                .finish(),
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => f
                .debug_struct("IfElseStmts")
                .field("if_stmt", if_stmt)
                .field("elif_stmts", elif_stmts)
                .field("else_stmt", else_stmt)
                .finish(),
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => f
                .debug_struct("MatchStmts")
                .field("pattern_stmt", pattern_stmt)
                .field("case_stmts", case_stmts)
                .finish(),
            Ast::Defn {
                token_group_idx,
                body,
                accessibility,
                entity_card,
                ident,
                is_generic,
                body_kind,
            } => f
                .debug_struct("Defn")
                .field("token_group_idx", token_group_idx)
                .field("body", body)
                .field("accessibility", accessibility)
                .field("entity_card", entity_card)
                .field("ident", &ident.debug_with(db, include_all_fields))
                .field("is_generic", is_generic)
                .field("body_kind", body_kind)
                .finish(),
            Ast::Impl {
                token_group_idx,
                body,
            } => f
                .debug_struct("Impl")
                .field("token_group_idx", token_group_idx)
                .field("body", body)
                .finish(),
            Ast::Main {
                token_group_idx,
                body,
            } => f
                .debug_struct("Main")
                .field("token_group_idx", token_group_idx)
                .field("body", body)
                .finish(),
            Ast::Config {
                token_group_idx,
                body,
            } => f
                .debug_struct("Config")
                .field("token_group_idx", token_group_idx)
                .field("body", body)
                .finish(),
        }
    }
}

impl<Db: AstDb> salsa::DebugWithDb<Db> for AstSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("AstSheet")
            .field("arena", &self.arena.debug_with(db, include_all_fields))
            .field("top_level_asts", &self.top_level_asts)
            .finish()
    }
}
