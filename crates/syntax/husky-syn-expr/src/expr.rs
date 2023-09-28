mod html;
mod list_item;

pub use self::html::*;
pub use self::list_item::*;

use crate::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use vec_like::SmallVecMap;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynExprData {
    Literal(RegionalTokenIdx, LiteralData),
    PrincipalEntityPath {
        path_expr_idx: SynPrincipalEntityPathExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    AssociatedItem {
        parent_expr_idx: SynPrincipalEntityPathExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
    },
    InheritedSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        inherited_symbol_idx: SynInheritedSymbolIdx,
        inherited_symbol_kind: SynInheritedSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        current_symbol_idx: SynCurrentSymbolIdx,
        current_symbol_kind: SynCurrentSymbolKind,
    },
    FrameVarDecl {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        frame_var_symbol_idx: SynCurrentSymbolIdx,
        current_symbol_kind: SynCurrentSymbolKind,
    },
    SelfType(RegionalTokenIdx),
    SelfValue(RegionalTokenIdx),
    Binary {
        lopd: SynExprIdx,
        opr: BinaryOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        ropd: SynExprIdx,
    },
    Be {
        src: SynExprIdx,
        be_regional_token_idx: RegionalTokenIdx,
        target: SynExprResult<BePatternSynObelisk>,
    },
    Prefix {
        opr: PrefixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        opd: SynExprIdx,
    },
    Suffix {
        opd: SynExprIdx,
        opr: SuffixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
    },
    /// we shall need the exact type of `f` to disambiguate the following:
    /// - `f(x1, ..., xn)` can be interpreted in two ways:
    ///   - `f` is a curry function, `(x1, ..., xn)` is a tuple, this is an application
    ///   - `f` is a Ritchie function, `(x1, ..., xn)` is the list of arguments, this is a Ritchie function call
    ///
    /// - `f(x)` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon `x`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with one element
    ///
    /// - `f(x,)` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon one element tuple `(x,)`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with one element
    ///
    /// - `f()` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon unit `()`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with zero element
    ///
    /// - `f(,)` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon zero element tuple `(,)`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with zero element
    FunctionApplicationOrCall {
        function: SynExprIdx,
        template_arguments: Option<SynTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    /// function type or trait
    Ritchie {
        ritchie_kind: RitchieKind,
        ritchie_kind_regional_token_idx: RegionalTokenIdx,
        lpar_token: LparRegionalToken,
        parameter_ty_items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
        light_arrow_token: Option<LightArrowRegionalToken>,
        /// it's guaranteed that `return_ty_expr` is some if and only if
        /// `light_arrow_token` is some
        return_ty_syn_expr_idx: Option<SynExprIdx>,
    },
    FunctionCall {
        function: SynExprIdx,
        generic_arguments: Option<SynTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCallListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    Field {
        owner: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
    },
    MethodApplicationOrCall {
        self_argument: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        template_arguments: Option<SynTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    TemplateInstantiation {
        template: SynExprIdx,
        generic_arguments: SynTemplateArgumentList,
    },
    // todo: implicit arguments
    ExplicitApplication {
        function_expr_idx: SynExprIdx,
        argument_expr_idx: SynExprIdx,
    },
    At {
        at_regional_token_idx: RegionalTokenIdx,
        place_label_regional_token: Option<PlaceLabelRegionalToken>,
    },
    Unit {
        lpar_regional_token_idx: RegionalTokenIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    Bracketed {
        lpar_regional_token_idx: RegionalTokenIdx,
        item: SynExprIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    NewTuple {
        lpar_regional_token_idx: RegionalTokenIdx,
        /// guaranteed that items.len() > 0
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    /// there are two cases
    /// - index `$owner[$items]` where `$owner` can be indexed
    /// - application `$owner [$items]` where `$owner` is of type `List _ -> S`
    /// the cases are determined by whether `$owner` is of curry type
    IndexOrCompositionWithList {
        owner: SynExprIdx,
        lbox_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    List {
        lbox_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    /// [:] means Slice
    /// [:n] means array as `[_;n]` in Rust
    /// [:n1, n2, ...] means multidimensional array
    BoxColonList {
        lbox_regional_token_idx: RegionalTokenIdx,
        colon_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    Block {
        stmts: SynStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        empty_html_bra_idx: RegionalTokenIdx,
        function_ident: IdentRegionalToken,
        arguments: IdentMap<SynHtmlArgumentExpr>,
        empty_html_ket: EmptyHtmlKetRegionalToken,
    },
    /// sorry is for comptime (say proof) terms
    Sorry {
        regional_token_idx: RegionalTokenIdx,
    },
    /// todo is for runtime terms
    Todo {
        regional_token_idx: RegionalTokenIdx,
    },
    Unreachable {
        regional_token_idx: RegionalTokenIdx,
    },
    Err(SynExprError),
}

impl From<IdentifiableEntityPathExpr> for SynExprData {
    fn from(expr: IdentifiableEntityPathExpr) -> Self {
        match expr {
            IdentifiableEntityPathExpr::Principal {
                path_expr_idx,
                opt_path,
            } => SynExprData::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            },
            IdentifiableEntityPathExpr::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => SynExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            },
        }
    }
}

pub type SynExprArena = Arena<SynExprData>;
pub type SynExprIdx = ArenaIdx<SynExprData>;
pub type SynExprIdxRange = ArenaIdxRange<SynExprData>;
pub type SynExprMap<V> = ArenaMap<SynExprData, V>;

#[derive(Debug, PartialEq, Eq)]
pub struct SynTemplateArgumentList {
    langle: RegionalTokenIdx,
    arguments: SmallVec<[SynCommaListItem; 4]>,
    rangle: RegionalTokenIdx,
}

impl SynTemplateArgumentList {
    pub(crate) fn new(
        langle: RegionalTokenIdx,
        arguments: SmallVec<[SynCommaListItem; 4]>,
        rangle: RegionalTokenIdx,
    ) -> Self {
        Self {
            langle,
            arguments,
            rangle,
        }
    }

    pub fn langle(&self) -> RegionalTokenIdx {
        self.langle
    }

    pub fn arguments(&self) -> &[SynCommaListItem] {
        &self.arguments
    }

    pub fn rangle(&self) -> RegionalTokenIdx {
        self.rangle
    }
}

type CommaRegionalTokens = SmallVec<[RegionalTokenIdx; 2]>;
