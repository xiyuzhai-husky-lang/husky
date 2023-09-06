#![feature(trait_upcasting)]
mod children;
mod db;
mod decr;
mod error;
mod parser;
mod range;
mod specs;
#[cfg(feature = "test-utils")]
pub mod test_utils;
#[cfg(test)]
mod tests;
mod utils;

pub use self::children::*;
pub use self::db::AstDb;
pub use self::decr::*;
pub use self::error::*;
pub use self::range::*;
pub use self::specs::*;

use self::parser::*;
use either::*;
use husky_coword::*;
use husky_entity_path::{ItemPath, TypeVariantPath};
use husky_entity_taxonomy::EntityKind;
use husky_scope::Scope;
use husky_scope_expr::VisibilityExpr;
use husky_token::{
    DecrIdentToken, IdentToken, TokenGroupIdx, TokenIdx, TokenStreamState, VerticalToken,
};
use husky_vfs::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use salsa::DbWithJar;

#[salsa::jar(db = AstDb)]
pub struct AstJar(ast_sheet, ast_token_idx_range_sheet);

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub enum Ast {
    Err {
        token_group_idx: TokenGroupIdx,
        error: AstError,
    },
    Use {
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state_after_visibility_expr: Option<TokenStreamState>,
    },
    /// specify comptime sorceries
    /// doesn't need to be processed until comptime
    Sorc { token_group_idx: TokenGroupIdx },
    /// decoration, used for deriving trait implementations, etc.
    /// needs to be processed before inference
    Decr {
        token_group_idx: TokenGroupIdx,
        ident: Ident,
    },
    BasicStmtOrBranch {
        token_group_idx: TokenGroupIdx,
        body: Option<FugitiveBody>,
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
        item_kind: EntityKind,
        ident_token: IdentToken,
        is_generic: bool,
        saved_stream_state: TokenStreamState,
        block: DefnBlock,
    },
    TypeVariant {
        token_group_idx: TokenGroupIdx,
        variant_path: TypeVariantPath,
        vertical_token: VerticalToken,
        ident_token: IdentToken,
        state_after: TokenStreamState,
    },
    ImplBlock {
        token_group_idx: TokenGroupIdx,
        items: Option<ImplBlockItems>,
    },
    Main {
        token_group_idx: TokenGroupIdx,
        body: FugitiveBody,
    },
    Config {
        token_group_idx: TokenGroupIdx,
        body: FugitiveBody,
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
#[salsa::debug_with_db(db = AstDb)]
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
