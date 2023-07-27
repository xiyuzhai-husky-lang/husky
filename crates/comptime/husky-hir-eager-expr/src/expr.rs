use crate::*;

pub type HirExprIdx = ();

#[derive(Debug, PartialEq, Eq)]
pub enum HirExpr {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    InheritedSymbol {
        ident: Ident,
        inherited_symbol_idx: InheritedHirSymbolIdx,
        inherited_symbol_kind: InheritedHirSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        current_symbol_idx: CurrentHirSymbolIdx,
        current_symbol_kind: CurrentHirSymbolKind,
    },
    FrameVarDecl {
        ident: Ident,
        frame_var_symbol_idx: CurrentHirSymbolIdx,
        current_symbol_kind: CurrentHirSymbolKind,
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
        generic_arguments: Option<HirGenericArgumentList>,
        item_groups: SmallVec<[HirCallListItemGroup; 4]>,
    },
    Field {
        owner: HirExprIdx,
        ident: Ident,
    },
    MethodCall {
        self_argument: HirExprIdx,
        ident: Ident,
        generic_arguments: Option<HirGenericArgumentList>,
        items: SmallVec<[HirExprIdx; 4]>,
    },
    // todo: implicit arguments
    ExplicitApplication {
        function_expr_idx: HirExprIdx,
        argument_expr_idx: HirExprIdx,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirExprIdx; 4]>,
    },
    /// there are two cases
    /// - index `$owner[$items]` where `$owner` can be indexed
    /// - application `$owner [$items]` where `$owner` is of type `List _ -> S`
    /// the cases are determined by whether `$owner` is of curry type
    IndexOrCompositionWithList {
        owner: HirExprIdx,
        items: SmallVec<[HirExprIdx; 4]>,
    },
    List {
        items: SmallVec<[HirExprIdx; 4]>,
    },
    Block {
        stmts: HirStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        function_ident: Ident,
        arguments: IdentMap<HtmlArgumentHirExpr>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum HirCallListItemGroup {
    Regular,
    Variadic,
    Keyed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HtmlArgumentHirExpr {
    Expanded {
        property_ident: Ident,
        expr: HirExprIdx,
    },
    Shortened {
        property_ident: Ident,
    },
}

impl vec_like::AsVecMapEntry for HtmlArgumentHirExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            HtmlArgumentHirExpr::Expanded { property_ident, .. }
            | HtmlArgumentHirExpr::Shortened { property_ident, .. } => *property_ident,
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            HtmlArgumentHirExpr::Expanded { property_ident, .. }
            | HtmlArgumentHirExpr::Shortened { property_ident, .. } => property_ident,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirGenericArgumentList {/*todo */}

#[cfg(feature = "rust-syn-gen")]
impl Expr {}
