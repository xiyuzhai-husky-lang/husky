#![feature(result_flattening)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
mod db;
mod error;
mod helpers;
mod html;
mod list_item;
mod parser;
mod pattern;
mod precedence;
mod principal_entity_path;
mod range;
mod region;
mod snippet;
mod stmt;
mod symbol;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::error::*;
pub use self::helpers::*;
pub use self::html::*;
pub use self::list_item::*;
pub use self::parser::*;
pub use self::pattern::*;
pub use self::principal_entity_path::*;
pub use self::range::*;
pub use self::region::*;
pub use self::stmt::*;
pub use self::symbol::*;

use husky_coword::*;
use husky_entity_path::{EntityPath, PrincipalEntityPath};
use husky_entity_tree::*;
use husky_opn_syntax::*;
use husky_term_prelude::*;
use husky_text::*;
use husky_token::*;
use precedence::*;
use range::*;
use smallvec::SmallVec;
use snippet::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = ExprDb)]
pub struct SynExprJar(ExprRegion, parse_expr_from_snippet, expr_range_region);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    Uncertain {
        inclination: BaseEntityPathInclination,
    },
    Err,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPathInclination {
    GlobalValue,
    FunctionOrLocalValue,
    TypeOrVariant,
}

impl BaseEntityPathInclination {
    pub fn from_case(case: IdentCase) -> Self {
        match case {
            IdentCase::SingleCapital | IdentCase::PascalCase => {
                BaseEntityPathInclination::TypeOrVariant
            }
            IdentCase::AllCapital => BaseEntityPathInclination::GlobalValue,
            IdentCase::SnakeCase => BaseEntityPathInclination::FunctionOrLocalValue,
            IdentCase::CamelCase => BaseEntityPathInclination::TypeOrVariant,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum SynExpr {
    Literal(TokenIdx, Literal),
    PrincipalEntityPath {
        entity_path_expr: PrincipalEntityPathExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    ScopeResolution {
        parent_expr_idx: ExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: IdentToken,
    },
    InheritedSymbol {
        ident: Ident,
        token_idx: TokenIdx,
        inherited_symbol_idx: InheritedSymbolIdx,
        inherited_symbol_kind: InheritedSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        token_idx: TokenIdx,
        current_symbol_idx: CurrentSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
    },
    FrameVarDecl {
        token_idx: TokenIdx,
        ident: Ident,
        frame_var_symbol_idx: CurrentSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
    },
    SelfType(TokenIdx),
    SelfValue(TokenIdx),
    Binary {
        lopd: ExprIdx,
        opr: BinaryOpr,
        opr_token_idx: TokenIdx,
        ropd: ExprIdx,
    },
    Be {
        src: ExprIdx,
        be_token_idx: TokenIdx,
        target: ExprResult<BeVariablesPattern>,
    },
    Prefix {
        opr: PrefixOpr,
        opr_token_idx: TokenIdx,
        opd: ExprIdx,
    },
    Suffix {
        opd: ExprIdx,
        opr: SuffixOpr,
        opr_token_idx: TokenIdx,
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
        function: ExprIdx,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        items: SmallVec<[CommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    /// function type or trait
    Ritchie {
        ritchie_kind_token_idx: TokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LeftParenthesisToken,
        parameter_ty_items: SmallVec<[CommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
        light_arrow_token: Option<LightArrowToken>,
        /// it's guaranteed that `return_ty_expr` is some if and only if
        /// `light_arrow_token` is some
        return_ty_expr: Option<ExprIdx>,
    },
    FunctionCall {
        function: ExprIdx,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        items: SmallVec<[CallListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    Field {
        owner: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
    },
    MethodApplicationOrCall {
        self_argument: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        items: SmallVec<[CommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    TemplateInstantiation {
        template: ExprIdx,
        implicit_arguments: ImplicitArgumentList,
    },
    // todo: implicit arguments
    ExplicitApplication {
        function_expr_idx: ExprIdx,
        argument_expr_idx: ExprIdx,
    },
    Unit {
        lpar_token_idx: TokenIdx,
        rpar_token_idx: TokenIdx,
    },
    Bracketed {
        lpar_token_idx: TokenIdx,
        item: ExprIdx,
        rpar_token_idx: TokenIdx,
    },
    NewTuple {
        lpar_token_idx: TokenIdx,
        /// guaranteed that items.len() > 0
        items: SmallVec<[CommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    /// there are two cases
    /// - index `$owner[$items]` where `$owner` can be indexed
    /// - application `$owner [$items]` where `$owner` is of type `List _ -> S`
    /// the cases are determined by whether `$owner` is of curry type
    IndexOrCompositionWithList {
        owner: ExprIdx,
        lbox_token_idx: TokenIdx,
        items: SmallVec<[CommaListItem; 4]>,
        rbox_token_idx: TokenIdx,
    },
    List {
        lbox_token_idx: TokenIdx,
        items: SmallVec<[CommaListItem; 4]>,
        rbox_token_idx: TokenIdx,
    },
    /// [:] means Slice
    /// [:n] means array as `[_;n]` in Rust
    /// [:n1, n2, ...] means multidimensional array
    BoxColonList {
        lbox_token_idx: TokenIdx,
        colon_token_idx: TokenIdx,
        items: SmallVec<[CommaListItem; 4]>,
        rbox_token_idx: TokenIdx,
    },
    Block {
        stmts: StmtIdxRange,
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
pub struct ImplicitArgumentList {
    langle: TokenIdx,
    arguments: SmallVec<[CommaListItem; 4]>,
    rangle: TokenIdx,
}

impl ImplicitArgumentList {
    pub(crate) fn new(
        langle: TokenIdx,
        arguments: SmallVec<[CommaListItem; 4]>,
        rangle: TokenIdx,
    ) -> Self {
        Self {
            langle,
            arguments,
            rangle,
        }
    }

    pub fn langle(&self) -> TokenIdx {
        self.langle
    }

    pub fn arguments(&self) -> &[CommaListItem] {
        &self.arguments
    }

    pub fn rangle(&self) -> TokenIdx {
        self.rangle
    }
}

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use vec_like::SmallVecMap;

pub type ExprArena = Arena<SynExpr>;
pub type ExprIdx = ArenaIdx<SynExpr>;
pub type ExprIdxRange = ArenaIdxRange<SynExpr>;
pub type ExprMap<V> = ArenaMap<SynExpr, V>;

type Commas = SmallVec<[TokenIdx; 2]>;
