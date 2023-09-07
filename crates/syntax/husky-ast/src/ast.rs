use crate::*;

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
