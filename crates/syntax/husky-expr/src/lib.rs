#![feature(result_flattening)]
#![feature(trait_upcasting)]
mod db;
mod entity_path;
mod error;
mod html;
mod parser;
mod pattern;
mod precedence;
mod range;
mod region;
mod snippet;
mod stmt;
mod symbol;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::html::*;
pub use self::parser::*;
pub use self::pattern::*;
pub use self::range::*;
pub use self::region::*;
pub use self::stmt::*;
pub use self::symbol::*;

use husky_entity_path::EntityPath;
use husky_entity_tree::*;
use husky_opn_syntax::*;
use husky_term_prelude::*;
use husky_text::*;
use husky_token::*;
use husky_word::*;
use precedence::*;
use range::*;
use smallvec::SmallVec;
use snippet::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = ExprDb)]
pub struct ExprJar(ExprRegion, parse_expr_from_snippet, expr_range_region);

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
pub enum Expr {
    Literal(TokenIdx, Literal),
    PrincipalEntityPath {
        entity_path_expr: EntityPathExprIdx,
        path: Option<EntityPath>,
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
        function: ExprIdx,
        argument: ExprIdx,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct KeyedArgumentExpr {
    key_token_idx: TokenIdx,
    key: Ident,
    argument_expr_idx: ExprIdx,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommaListItem {
    expr_idx: ExprIdx,
    comma_token_idx: Option<TokenIdx>,
}

impl CommaListItem {
    pub fn expr_idx(self) -> ExprIdx {
        self.expr_idx
    }

    pub fn comma_token_idx(self) -> Option<TokenIdx> {
        self.comma_token_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CallListItem {
    kind: CallListItemKind,
    argument_expr_idx: ExprIdx,
    separator: CallListSeparator,
}

impl From<CommaListItem> for CallListItem {
    fn from(item: CommaListItem) -> Self {
        CallListItem {
            kind: CallListItemKind::Argument,
            argument_expr_idx: item.expr_idx,
            separator: match item.comma_token_idx {
                Some(comma_token_idx) => CallListSeparator::Comma(comma_token_idx),
                None => CallListSeparator::None,
            },
        }
    }
}

impl CallListItem {
    pub fn new_regular(argument_expr_idx: ExprIdx, comma: Option<TokenIdx>) -> Self {
        Self {
            kind: CallListItemKind::Argument,
            separator: match comma {
                Some(comma_token_idx) => CallListSeparator::Comma(comma_token_idx),
                None => CallListSeparator::None,
            },
            argument_expr_idx,
        }
    }

    pub fn kind(&self) -> CallListItemKind {
        self.kind
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }

    pub fn argument_expr_idx(&self) -> ExprIdx {
        self.argument_expr_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CallListItemKind {
    Argument,
    KeyedArgument { key_token_idx: TokenIdx, key: Ident },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CallListSeparator {
    None,
    Comma(TokenIdx),
    Semicolon(TokenIdx),
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

pub type ExprArena = Arena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprIdxRange = ArenaIdxRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;

type Commas = SmallVec<[TokenIdx; 2]>;
