#![feature(result_flattening)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
mod context;
mod db;
mod entity_path;
mod error;
pub mod helpers;
mod html;
mod list_item;
mod obelisks;
mod parser;
mod pattern;
mod precedence;
mod range;
mod region;
mod snippet;
mod stmt;
pub mod symbol;
#[cfg(test)]
mod tests;

pub use self::context::*;
pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::html::*;
pub use self::list_item::*;
pub use self::obelisks::*;
pub use self::parser::*;
pub use self::pattern::*;
pub use self::range::*;
pub use self::region::*;
pub use self::stmt::*;
pub use self::symbol::*;

use husky_coword::*;
use husky_entity_path::{EntityPath, ItemPath, PrincipalEntityPath};
use husky_entity_syn_tree::{helpers::tokra_region::*, *};
use husky_opr::*;
use husky_regional_token::*;
use husky_term_prelude::*;
use husky_token_data::*;
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
    Literal(RegionalTokenIdx, Literal),
    IdentifiableEntityPath(IdentifiableEntityPathExpr),
    InheritedSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        inherited_symbol_idx: InheritedSynSymbolIdx,
        inherited_symbol_kind: InheritedSynSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        current_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSynSymbolKind,
    },
    FrameVarDecl {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        frame_var_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSynSymbolKind,
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
        target: SynExprResult<BePatternObelisk>,
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
        generic_arguments: Option<SynGenericArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    /// function type or trait
    Ritchie {
        ritchie_kind_regional_token_idx: RegionalTokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LparRegionalToken,
        parameter_ty_items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
        light_arrow_token: Option<LightArrowRegionalToken>,
        /// it's guaranteed that `return_ty_expr` is some if and only if
        /// `light_arrow_token` is some
        return_ty_expr: Option<SynExprIdx>,
    },
    FunctionCall {
        function: SynExprIdx,
        generic_arguments: Option<SynGenericArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[CallListItem; 4]>,
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
        generic_arguments: Option<SynGenericArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
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
    Err(SynExprError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynGenericArgumentList {
    langle: RegionalTokenIdx,
    arguments: SmallVec<[SynCommaListItem; 4]>,
    rangle: RegionalTokenIdx,
}

impl SynGenericArgumentList {
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

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use vec_like::SmallVecMap;

pub type SynExprArena = Arena<SynExpr>;
pub type SynExprIdx = ArenaIdx<SynExpr>;
pub type SynExprIdxRange = ArenaIdxRange<SynExpr>;
pub type SynExprMap<V> = ArenaMap<SynExpr, V>;

type Commas = SmallVec<[RegionalTokenIdx; 2]>;
