use husky_ethereal_term::EtherealTerm;
use husky_expr_ty::{
    ApplicationOrFunctionCallExprDisambiguation, ExprDisambiguation, RitchieParameterArgumentMatch,
};
use husky_syn_expr::{SynExpr, SynExprIdx, SynStmtIdx};

use crate::*;

pub type HirEagerExprArena = Arena<HirEagerExpr>;
pub type HirEagerExprIdx = ArenaIdx<HirEagerExpr>;
pub type HirEagerExprIdxRange = ArenaIdxRange<HirEagerExpr>;
pub type HirEagerExprMap<V> = ArenaMap<HirEagerExpr, V>;

#[derive(Debug, PartialEq, Eq)]
pub enum HirEagerExpr {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    InheritedSymbol {
        ident: Ident,
        // inherited_symbol_idx: InheritedHirEagerSymbolIdx,
        // inherited_symbol_kind: InheritedHirEagerSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        // current_symbol_idx: CurrentHirEagerSymbolIdx,
        // current_symbol_kind: CurrentHirEagerSymbolKind,
    },
    FrameVarDecl {
        ident: Ident,
        // frame_var_symbol_idx: CurrentHirEagerSymbolIdx,
        // current_symbol_kind: CurrentHirEagerSymbolKind,
    },
    SelfType,
    SelfValue,
    Binary {
        lopd: HirEagerExprIdx,
        opr: BinaryOpr,
        ropd: HirEagerExprIdx,
    },
    Be {
        src: HirEagerExprIdx,
        target: HirEagerBeVariablesPattern,
    },
    Prefix {
        opr: PrefixOpr,
        opd: HirEagerExprIdx,
    },
    Suffix {
        opd: HirEagerExprIdx,
        opr: SuffixOpr,
    },
    FunctionCall {
        function: HirEagerExprIdx,
        generic_arguments: Option<HirGenericArgumentList>,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    Field {
        owner: HirEagerExprIdx,
        ident: Ident,
    },
    MethodCall {
        self_argument: HirEagerExprIdx,
        ident: Ident,
        generic_arguments: Option<HirGenericArgumentList>,
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    Index {
        owner: HirEagerExprIdx,
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    List {
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    Block {
        stmts: HirEagerStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        function_ident: Ident,
        arguments: IdentMap<HtmlArgumentHirEagerExpr>,
    },
    Todo,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum HirEagerCallListItemGroup {
    Regular(HirEagerExprIdx),
    Variadic,
    Keyed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HtmlArgumentHirEagerExpr {
    Expanded {
        property_ident: Ident,
        expr: HirEagerExprIdx,
    },
    Shortened {
        property_ident: Ident,
    },
}

impl vec_like::AsVecMapEntry for HtmlArgumentHirEagerExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            HtmlArgumentHirEagerExpr::Expanded { property_ident, .. }
            | HtmlArgumentHirEagerExpr::Shortened { property_ident, .. } => *property_ident,
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            HtmlArgumentHirEagerExpr::Expanded { property_ident, .. }
            | HtmlArgumentHirEagerExpr::Shortened { property_ident, .. } => property_ident,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirGenericArgumentList {/*todo */}

#[cfg(feature = "rust-syn-gen")]
impl Expr {}

impl<'a> HirEagerExprBuilder<'a> {
    pub fn new_expr(&mut self, syn_expr_idx: SynExprIdx) -> HirEagerExprIdx {
        let hir_eager_expr = match self.syn_expr_region_data()[syn_expr_idx] {
            SynExpr::Literal(_, _) => HirEagerExpr::Literal(match self.expr_term(syn_expr_idx) {
                EtherealTerm::Literal(lit) => lit,
                _ => unreachable!(),
            }),
            SynExpr::PrincipalEntityPath {
                item_path_expr,
                opt_path,
            } => {
                let path = opt_path.expect("whatever");
                // ad hoc
                HirEagerExpr::PrincipalEntityPath(path)
            }
            SynExpr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
            } => todo!(),
            SynExpr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => HirEagerExpr::InheritedSymbol { ident },
            SynExpr::CurrentSymbol {
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SynExpr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SynExpr::SelfType(_) => todo!(),
            SynExpr::SelfValue(_) => todo!(),
            SynExpr::Binary {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => todo!(),
            SynExpr::Be {
                src,
                be_token_idx,
                ref target,
            } => todo!(),
            SynExpr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => todo!(),
            SynExpr::Suffix {
                opd,
                opr,
                opr_token_idx,
            } => todo!(),
            SynExpr::FunctionApplicationOrCall {
                function,
                ref generic_arguments,
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => {
                let ExprDisambiguation::ExplicitApplicationOrFunctionCall(disambiguation) =
                    self.expr_disambiguation(syn_expr_idx)
                else {
                    unreachable!()
                };
                match disambiguation {
                    ApplicationOrFunctionCallExprDisambiguation::Application => {
                        todo!()
                    }
                    ApplicationOrFunctionCallExprDisambiguation::FnCall {
                        ritchie_parameter_argument_matches,
                    } => HirEagerExpr::FunctionCall {
                        function: self.new_expr(function),
                        generic_arguments: generic_arguments.as_ref().map(|_| todo!()),
                        item_groups: ritchie_parameter_argument_matches
                            .iter()
                            .map(|pam| match pam {
                                RitchieParameterArgumentMatch::Regular(_, item) => {
                                    HirEagerCallListItemGroup::Regular(
                                        self.new_expr(item.argument_expr_idx()),
                                    )
                                }
                                RitchieParameterArgumentMatch::Variadic(_, _) => todo!(),
                                RitchieParameterArgumentMatch::Keyed(_, _) => todo!(),
                            })
                            .collect(),
                    },
                    ApplicationOrFunctionCallExprDisambiguation::GnCall {
                        ritchie_parameter_argument_matches,
                    } => unreachable!(),
                }
            }
            SynExpr::Ritchie {
                ritchie_kind_token_idx,
                ritchie_kind,
                lpar_token,
                ref parameter_ty_items,
                rpar_token_idx,
                light_arrow_token,
                return_ty_expr,
            } => todo!(),
            SynExpr::FunctionCall {
                function,
                ref generic_arguments,
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            SynExpr::MethodApplicationOrCall {
                self_argument,
                dot_token_idx,
                ident_token,
                ref generic_arguments,
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => {
                // todo: method application should be ignored
                let ExprDisambiguation::MethodDispatch(disambiguation) =
                    self.expr_disambiguation(syn_expr_idx)
                else {
                    unreachable!()
                };
                HirEagerExpr::MethodCall {
                    self_argument: self.new_expr(self_argument),
                    ident: ident_token.ident(),
                    generic_arguments: generic_arguments.as_ref().map(|_| todo!()),
                    items: items.iter().map(|item| todo!()).collect(),
                }
            }
            SynExpr::TemplateInstantiation {
                template,
                ref generic_arguments,
            } => todo!(),
            SynExpr::ExplicitApplication {
                function_expr_idx,
                argument_expr_idx,
            } => todo!(),
            SynExpr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => todo!(),
            SynExpr::NewTuple {
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                ref items,
                rbox_token_idx,
            } => todo!(),
            SynExpr::List {
                lbox_token_idx,
                ref items,
                rbox_token_idx,
            } => todo!(),
            SynExpr::BoxColonList {
                lbox_token_idx,
                colon_token_idx,
                ref items,
                rbox_token_idx,
            } => todo!(),
            SynExpr::Block { stmts } => {
                let mut syn_stmt_indices: Vec<SynStmtIdx> = vec![];
                let mut hir_eager_stmts: Vec<HirEagerStmt> = vec![];
                for syn_stmt_idx in stmts {
                    match self.new_stmt(syn_stmt_idx) {
                        Some(hir_eager_stmt) => {
                            syn_stmt_indices.push(syn_stmt_idx);
                            hir_eager_stmts.push(hir_eager_stmt)
                        }
                        None => todo!(),
                    }
                }
                // todo: record syn_stmt_indices in source map
                HirEagerExpr::Block {
                    stmts: self.alloc_stmts(syn_stmt_indices, hir_eager_stmts),
                }
            }
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExpr::Sorry { token_idx } => todo!(),
            SynExpr::Todo { token_idx } => HirEagerExpr::Todo,
            SynExpr::Err(ref e) => {
                unreachable!("e = {:?}, path = {:?}", e.debug(self.db()), self.path())
            }
        };
        self.alloc_expr(syn_expr_idx, hir_eager_expr)
    }
}
