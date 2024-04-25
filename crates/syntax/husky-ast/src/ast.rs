use crate::*;

/// syntax tree down to TokenVerse level
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum AstData {
    Err {
        token_verse_idx: TokenVerseIdx,
        error: AstError,
    },
    Use {
        token_verse_idx: TokenVerseIdx,
        visibility_expr: VisibilityExpr,
        state_after_visibility_expr: Option<TokenStreamState>,
    },
    /// specify comptime sorceries
    /// doesn't need to be processed until comptime
    Sorc { token_verse_idx: TokenVerseIdx },
    /// decoration, used for deriving trait implementations, etc.
    /// needs to be processed before inference
    Attr {
        token_verse_idx: TokenVerseIdx,
        ident: Ident,
    },
    BasicStmtOrBranch {
        token_verse_idx: TokenVerseIdx,
        body: Option<FormBody>,
    },
    IfElseStmts {
        if_branch: AstIdx,
        elif_branches: AstIdxRange,
        else_branch: Option<AstIdx>,
    },
    MatchStmt {
        token_verse_idx: TokenVerseIdx,
        pattern_stmt: AstIdx,
        case_branches: AstIdxRange,
    },
    Identifiable {
        token_verse_idx: TokenVerseIdx,
        visibility_expr: VisibilityExpr,
        item_kind: EntityKind,
        ident_token: IdentToken,
        is_generic: bool,
        saved_stream_state: TokenStreamState,
        block: DefnBlock,
    },
    TypeVariant {
        token_verse_idx: TokenVerseIdx,
        variant_path: TypeVariantPath,
        vertical_token: VerticalToken,
        ident_token: IdentToken,
        saved_stream_state: TokenStreamState,
    },
    ImplBlock {
        token_verse_idx: TokenVerseIdx,
        items: Option<ImplBlockItems>,
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

pub type AstArena = Arena<AstData>;
pub type AstIdx = ArenaIdx<AstData>;
pub type AstIdxRange = ArenaIdxRange<AstData>;
pub type AstMap<V> = ArenaMap<AstData, V>;
