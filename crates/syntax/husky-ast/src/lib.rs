#![feature(trait_upcasting)]
mod db;
mod decr;
mod error;
mod group;
mod parser;
mod range;
mod specs;
#[cfg(feature = "test-utils")]
pub mod test_utils;
#[cfg(test)]
mod tests;
mod utils;

pub use self::db::AstDb;
pub use self::decr::*;
pub use self::error::*;
pub use self::range::*;
pub use self::specs::*;

use self::parser::*;
use either::*;
use husky_entity_path::{EntityPath, TypeVariantPath};
use husky_entity_taxonomy::EntityKind;
use husky_token::{DecrIdentToken, IdentToken, TokenGroupIdx, TokenIdx};
use husky_vfs::*;
use husky_visibility::Visibility;
use husky_visibility_expr::VisibilityExpr;
use husky_word::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use salsa::DbWithJar;

#[salsa::jar(db = AstDb)]
pub struct AstJar(ast_sheet, ast_token_idx_range_sheet);

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = AstDb)]
pub enum Ast {
    Err {
        token_group_idx: TokenGroupIdx,
        error: AstError,
    },
    Use {
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state_after_visibility_expr: Option<TokenIdx>,
    },
    /// specify internal attributes
    /// doesn't need to be processed until comptime
    Attr { token_group_idx: TokenGroupIdx },
    /// decoration, used for deriving trait implementations, etc.
    /// needs to be processed before inference
    Decr {
        token_group_idx: TokenGroupIdx,
        ident: Ident,
    },
    BasicStmtOrBranch {
        token_group_idx: TokenGroupIdx,
        body: AstIdxRange,
    },
    IfElseStmts {
        if_branch: AstIdx,
        elif_branches: AstIdxRange,
        else_branch: Option<AstIdx>,
    },
    MatchStmts {
        token_group_idx: TokenGroupIdx,
        pattern_stmt: AstIdx,
        case_stmts: AstIdxRange,
    },
    Defn {
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        entity_kind: EntityKind,
        /// None only when this is under impl block
        entity_path: Option<EntityPath>,
        ident_token: IdentToken,
        is_generic: bool,
        saved_stream_state: TokenIdx,
        body_kind: DefnBodyKind,
        body: AstIdxRange,
    },
    TypeVariant {
        token_group_idx: TokenGroupIdx,
        path: TypeVariantPath,
        ident: Ident,
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
    TypeVariants,
    FormVariants,
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
#[salsa::derive_debug_with_db(db = AstDb)]
pub struct AstSheet {
    ast_arena: AstArena,
    top_level_asts: AstIdxRange,
    // a list of siblings indices
    // list index has nothing to do with ast idx
    siblings: Vec<AstIdxRange>,
}

impl std::ops::Index<AstIdx> for AstSheet {
    type Output = Ast;

    fn index(&self, index: AstIdx) -> &Self::Output {
        &self.ast_arena[index]
    }
}

impl AstSheet {
    pub(crate) fn new(
        ast_arena: AstArena,
        top_level_asts: AstIdxRange,
        siblings: Vec<AstIdxRange>,
    ) -> Self {
        Self {
            ast_arena,
            top_level_asts,
            siblings,
        }
    }

    pub fn all_ast_indexed_iter<'a>(&'a self) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.ast_arena.indexed_iter()
    }

    pub fn indexed_iter<'a>(
        &'a self,
        ast_idx_range: AstIdxRange,
    ) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        ast_idx_range.into_iter().map(|idx| (idx, &self[idx]))
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
