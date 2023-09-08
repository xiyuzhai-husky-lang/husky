#![feature(result_flattening)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
mod context;
mod db;
mod error;
mod html;
mod list_item;
mod obelisks;
mod parser;
mod pattern;
mod precedence;
mod principal_entity_path;
mod range;
mod region;
mod snippet;
mod stmt;
pub mod symbol;
#[cfg(test)]
mod tests;

pub use self::context::*;
pub use self::db::*;
pub use self::error::*;
pub use self::html::*;
pub use self::list_item::*;
pub use self::obelisks::*;
pub use self::parser::*;
pub use self::pattern::*;
pub use self::principal_entity_path::*;
pub use self::range::*;
pub use self::region::*;
pub use self::stmt::*;
pub use self::symbol::*;

use husky_coword::*;
use husky_entity_path::{EntityPath, ItemPath, PrincipalEntityPath};
use husky_entity_syn_tree::*;
use husky_opr::*;
use husky_term_prelude::*;
use husky_text::*;
use husky_token::*;
use precedence::*;
use range::*;
use smallvec::SmallVec;
use snippet::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = SynExprDb)]
pub struct SynExprJar(SynExprRegion, parse_expr_from_snippet, expr_range_region);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    Uncertain {
        inclination: BaseEntityPathInclination,
    },
    SelfType,
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
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynExpr {
    Literal(TokenIdx, Literal),
    PrincipalEntityPath {
        item_path_expr: PrincipalEntityPathExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    ScopeResolution {
        parent_expr_idx: SynExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: IdentToken,
    },
    InheritedSymbol {
        ident: Ident,
        token_idx: TokenIdx,
        inherited_symbol_idx: InheritedSynSymbolIdx,
        inherited_symbol_kind: InheritedSynSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        token_idx: TokenIdx,
        current_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSynSymbolKind,
    },
    FrameVarDecl {
        token_idx: TokenIdx,
        ident: Ident,
        frame_var_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSynSymbolKind,
    },
    SelfType(TokenIdx),
    SelfValue(TokenIdx),
    Binary {
        lopd: SynExprIdx,
        opr: BinaryOpr,
        opr_token_idx: TokenIdx,
        ropd: SynExprIdx,
    },
    Be {
        src: SynExprIdx,
        be_token_idx: TokenIdx,
        target: SynExprResult<BeVariablesObelisk>,
    },
    Prefix {
        opr: PrefixOpr,
        opr_token_idx: TokenIdx,
        opd: SynExprIdx,
    },
    Suffix {
        opd: SynExprIdx,
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
        function: SynExprIdx,
        generic_arguments: Option<SynGenericArgumentList>,
        lpar_token_idx: TokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    /// function type or trait
    Ritchie {
        ritchie_kind_token_idx: TokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LparToken,
        parameter_ty_items: SmallVec<[SynCommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
        light_arrow_token: Option<LightArrowToken>,
        /// it's guaranteed that `return_ty_expr` is some if and only if
        /// `light_arrow_token` is some
        return_ty_expr: Option<SynExprIdx>,
    },
    FunctionCall {
        function: SynExprIdx,
        generic_arguments: Option<SynGenericArgumentList>,
        lpar_token_idx: TokenIdx,
        items: SmallVec<[CallListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    Field {
        owner: SynExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
    },
    MethodApplicationOrCall {
        self_argument: SynExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
        generic_arguments: Option<SynGenericArgumentList>,
        lpar_token_idx: TokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    TemplateInstantiation {
        template: SynExprIdx,
        generic_arguments: SynGenericArgumentList,
    },
    // todo: implicit arguments
    ExplicitApplication {
        function_expr_idx: SynExprIdx,
        argument_expr_idx: SynExprIdx,
    },
    Unit {
        lpar_token_idx: TokenIdx,
        rpar_token_idx: TokenIdx,
    },
    Bracketed {
        lpar_token_idx: TokenIdx,
        item: SynExprIdx,
        rpar_token_idx: TokenIdx,
    },
    NewTuple {
        lpar_token_idx: TokenIdx,
        /// guaranteed that items.len() > 0
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_token_idx: TokenIdx,
    },
    /// there are two cases
    /// - index `$owner[$items]` where `$owner` can be indexed
    /// - application `$owner [$items]` where `$owner` is of type `List _ -> S`
    /// the cases are determined by whether `$owner` is of curry type
    IndexOrCompositionWithList {
        owner: SynExprIdx,
        lbox_token_idx: TokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_token_idx: TokenIdx,
    },
    List {
        lbox_token_idx: TokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_token_idx: TokenIdx,
    },
    /// [:] means Slice
    /// [:n] means array as `[_;n]` in Rust
    /// [:n1, n2, ...] means multidimensional array
    BoxColonList {
        lbox_token_idx: TokenIdx,
        colon_token_idx: TokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_token_idx: TokenIdx,
    },
    Block {
        stmts: SynStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        empty_html_bra_idx: TokenIdx,
        function_ident: IdentToken,
        arguments: IdentMap<SynHtmlArgumentExpr>,
        empty_html_ket: EmptyHtmlKetToken,
    },
    /// sorry is for comptime (say proof) terms
    Sorry {
        token_idx: TokenIdx,
    },
    /// todo is for runtime terms
    Todo {
        token_idx: TokenIdx,
    },
    Err(ExprError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynGenericArgumentList {
    langle: TokenIdx,
    arguments: SmallVec<[SynCommaListItem; 4]>,
    rangle: TokenIdx,
}

impl SynGenericArgumentList {
    pub(crate) fn new(
        langle: TokenIdx,
        arguments: SmallVec<[SynCommaListItem; 4]>,
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

    pub fn arguments(&self) -> &[SynCommaListItem] {
        &self.arguments
    }

    pub fn rangle(&self) -> TokenIdx {
        self.rangle
    }
}

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use vec_like::SmallVecMap;

pub type SynExprArena = Arena<SynExpr>;
pub type SynExprIdx = ArenaIdx<SynExpr>;
pub type SynExprIdxRange = ArenaIdxRange<SynExpr>;
pub type SynExprMap<V> = ArenaMap<SynExpr, V>;

type Commas = SmallVec<[TokenIdx; 2]>;
