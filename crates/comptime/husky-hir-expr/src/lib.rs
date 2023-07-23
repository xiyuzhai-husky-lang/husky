pub mod db;
mod region;
mod source_map;

pub use self::region::*;
pub use self::source_map::*;

use self::db::*;
use husky_coword::*;
use husky_entity_path::*;
use husky_term_prelude::*;

pub type HirExprIdx = ();

#[derive(Debug, PartialEq, Eq)]
pub enum HirExpr {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    InheritedSymbol {
        ident: Ident,
        inherited_symbol_idx: InheritedSymbolIdx,
        inherited_symbol_kind: InheritedSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        current_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
    },
    FrameVarDecl {
        ident: Ident,
        frame_var_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
    },
    SelfType,
    SelfValue,
    Binary {
        lopd: HirExprIdx,
        opr: BinaryOpr,
        ropd: HirExprIdx,
    },
    Be {
        src: HirExprIdx,
        target: BeVariablesPattern,
    },
    Prefix {
        opr: PrefixOpr,
        opd: HirExprIdx,
    },
    Suffix {
        opd: HirExprIdx,
        opr: SuffixOpr,
    },
    FunctionCall {
        function: HirExprIdx,
        generic_arguments: Option<SynGenericArgumentList>,
        items: SmallVec<[CallListItem; 4]>,
    },
    Field {
        owner: HirExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
    },
    MethodCall {
        self_argument: HirExprIdx,
        ident_token: IdentToken,
        generic_arguments: Option<SynGenericArgumentList>,
        items: SmallVec<[SynCommaListItem; 4]>,
    },
    // todo: implicit arguments
    ExplicitApplication {
        function_expr_idx: HirExprIdx,
        argument_expr_idx: HirExprIdx,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[SynCommaListItem; 4]>,
    },
    /// there are two cases
    /// - index `$owner[$items]` where `$owner` can be indexed
    /// - application `$owner [$items]` where `$owner` is of type `List _ -> S`
    /// the cases are determined by whether `$owner` is of curry type
    IndexOrCompositionWithList {
        owner: HirExprIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
    },
    List {
        items: SmallVec<[SynCommaListItem; 4]>,
    },
    Block {
        stmts: SynStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        empty_html_bra_idx: TokenIdx,
        function_ident: IdentToken,
        arguments: IdentMap<HtmlArgumentExpr>,
        empty_html_ket: EmptyHtmlKetToken,
    },
    Err(ExprError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum HirStmt {}
