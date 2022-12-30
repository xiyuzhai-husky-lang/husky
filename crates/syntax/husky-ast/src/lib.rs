#![feature(trait_upcasting)]
mod db;
mod error;
mod parser;
mod range;
mod specs;
#[cfg(test)]
mod tests;
mod use_expr;

pub use crate::error::{AstError, AstResult};
pub use db::AstDb;
use husky_accessibility::Accessibility;
pub use range::*;
pub use specs::*;
pub use use_expr::*;

use husky_entity_path::{EntityPath, VariantPath};
use husky_entity_taxonomy::EntityKind;
use husky_text::*;
use husky_token::{IdentifierToken, TokenGroupIdx, TokenIdx};
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
        ident: Identifier,
        use_expr_idx: UseExprIdx,
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
        entity_kind: EntityKind,
        /// None only when this is under impl block
        entity_path: Option<EntityPath>,
        ident_token: IdentifierToken,
        is_generic: bool,
        body_kind: DefnBodyKind,
        saved_stream_state: TokenIdx,
    },
    ModuleItemVariant {
        token_group_idx: TokenGroupIdx,
        module_item_variant_path: VariantPath,
        ident: Identifier,
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
    EnumVariants,
    MatchCases,
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
pub(crate) fn ast_sheet(db: &dyn AstDb, module_path: ModulePath) -> VfsResult<AstSheet> {
    Ok(AstParser::new(db, module_path)?.parse_all())
}

#[derive(Debug, PartialEq, Eq)]
pub struct AstSheet {
    ast_arena: AstArena,
    top_level_asts: AstIdxRange,
    use_expr_arena: UseExprArena,
}

impl AstSheet {
    pub(crate) fn new(
        ast_arena: AstArena,
        top_level_asts: AstIdxRange,
        use_expr_arena: UseExprArena,
    ) -> Self {
        Self {
            ast_arena,
            use_expr_arena,
            top_level_asts,
        }
    }

    pub fn indexed_asts<'a>(&'a self) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.ast_arena.indexed_iter()
    }

    pub fn top_level_asts(&self) -> &AstIdxRange {
        &self.top_level_asts
    }

    pub fn top_level_asts_iter<'a>(&'a self) -> impl Iterator<Item = &'a Ast> + 'a {
        self.ast_arena[&self.top_level_asts].iter()
    }

    pub fn top_level_asts_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.ast_arena[&self.top_level_asts]
            .iter()
            .enumerate()
            .map(|(i, ast)| (self.top_level_asts.start() + i, ast))
    }
}

impl std::ops::Deref for AstSheet {
    type Target = AstArena;

    fn deref(&self) -> &Self::Target {
        &self.ast_arena
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
                ident,
                accessibility,
                use_expr_idx,
            } => f
                .debug_struct("Use")
                .field("token_group_idx", token_group_idx)
                .field("ident", &ident.debug_with(db, include_all_fields))
                .field("accessibility", accessibility)
                .field("use_expr_idx", use_expr_idx)
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
                entity_kind,
                entity_path,
                ident_token,
                is_generic,
                body_kind,
                saved_stream_state,
            } => f
                .debug_struct("Defn")
                .field("token_group_idx", token_group_idx)
                .field("body", body)
                .field("accessibility", accessibility)
                .field("entity_kind", entity_kind)
                .field(
                    "entity_path",
                    &entity_path.debug_with(db, include_all_fields),
                )
                .field(
                    "ident_token",
                    &ident_token.debug_with(db, include_all_fields),
                )
                .field("is_generic", is_generic)
                .field("body_kind", body_kind)
                .field("saved_stream_state", saved_stream_state)
                .finish(),
            Ast::ModuleItemVariant {
                token_group_idx,
                module_item_variant_path,
                ident,
            } => f
                .debug_struct("ModuleItemVariant")
                .field(
                    "module_item_variant_path",
                    &module_item_variant_path.debug_with(db, include_all_fields),
                )
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
            .field("arena", &self.ast_arena.debug_with(db, include_all_fields))
            .field("top_level_asts", &self.top_level_asts)
            .field(
                "use_expr_arena",
                &self.use_expr_arena.debug_with(db, include_all_fields),
            )
            .finish()
    }
}
