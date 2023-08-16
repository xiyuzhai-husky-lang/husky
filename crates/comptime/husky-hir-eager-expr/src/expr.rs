use husky_ethereal_term::EtherealTerm;
use husky_expr_ty::{
    ApplicationOrFunctionCallExprDisambiguation, ExprDisambiguation,
    IndexOrComposeWithListExprDisambiguation, MethodCallOrApplicationDisambiguation,
    RitchieParameterArgumentMatch,
};
use husky_fluffy_term::StaticDispatch;
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
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
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
    AssociatedFn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum HirEagerCallListItemGroup {
    Regular(HirEagerExprIdx),
    Variadic,
    Keyed,
}

impl<'a> HirEagerExprBuilder<'a> {
    fn new_call_list_item_groups(
        &mut self,
        pams: &[RitchieParameterArgumentMatch],
    ) -> SmallVec<[HirEagerCallListItemGroup; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(
        &mut self,
        pam: &RitchieParameterArgumentMatch,
    ) -> HirEagerCallListItemGroup {
        match pam {
            RitchieParameterArgumentMatch::Regular(_, item) => {
                HirEagerCallListItemGroup::Regular(item.argument_expr_idx().to_hir_eager(self))
            }
            RitchieParameterArgumentMatch::Variadic(_, _) => todo!(),
            RitchieParameterArgumentMatch::Keyed(_, _) => todo!(),
        }
    }
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

impl ToHirEager for SynExprIdx {
    type Output = HirEagerExprIdx;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let hir_eager_expr = match builder.syn_expr_region_data()[*self] {
            SynExpr::Literal(_, _) => HirEagerExpr::Literal(match builder.expr_term(*self) {
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
            } => {
                let ExprDisambiguation::StaticDispatch(dispatch) =
                    builder.expr_disambiguation(*self)
                else {
                    unreachable!()
                };
                match dispatch {
                    StaticDispatch::AssociatedFn(_) => HirEagerExpr::AssociatedFn,
                }
            }
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
            } => HirEagerExpr::CurrentSymbol { ident },
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
            } => HirEagerExpr::Binary {
                lopd: lopd.to_hir_eager(builder),
                opr,
                ropd: ropd.to_hir_eager(builder),
            },
            SynExpr::Be {
                src,
                be_token_idx,
                ref target,
            } => todo!(),
            SynExpr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => HirEagerExpr::Prefix {
                opr,
                opd: opd.to_hir_eager(builder),
            },
            SynExpr::Suffix {
                opd,
                opr,
                opr_token_idx,
            } => HirEagerExpr::Suffix {
                opr,
                opd: opd.to_hir_eager(builder),
            },
            SynExpr::FunctionApplicationOrCall {
                function,
                ref generic_arguments,
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => {
                let ExprDisambiguation::ExplicitApplicationOrFunctionCall(disambiguation) =
                    builder.expr_disambiguation(*self)
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
                        function: function.to_hir_eager(builder),
                        generic_arguments: generic_arguments.as_ref().map(|_| todo!()),
                        item_groups: builder
                            .new_call_list_item_groups(ritchie_parameter_argument_matches),
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
            } => HirEagerExpr::Field {
                owner: owner.to_hir_eager(builder),
                ident: ident_token.ident(),
            },
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
                let ExprDisambiguation::MethodCallOrApplication(disambiguation) =
                    builder.expr_disambiguation(*self)
                else {
                    unreachable!()
                };
                match disambiguation {
                    MethodCallOrApplicationDisambiguation::MethodCall {
                        method_dispatch,
                        ritchie_parameter_argument_matches,
                    } => {
                        let ritchie_parameter_argument_matches = ritchie_parameter_argument_matches
                            .as_ref()
                            .expect("hir stage no errors");
                        HirEagerExpr::MethodCall {
                            self_argument: self_argument.to_hir_eager(builder),
                            ident: ident_token.ident(),
                            generic_arguments: generic_arguments.as_ref().map(|_| todo!()),
                            item_groups: builder
                                .new_call_list_item_groups(ritchie_parameter_argument_matches),
                        }
                    }
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
            } => return item.to_hir_eager(builder),
            SynExpr::NewTuple {
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                ref items,
                ..
            } => {
                let ExprDisambiguation::IndexOrComposeWithList(disambiguation) =
                    builder.expr_disambiguation(*self)
                else {
                    unreachable!()
                };
                match disambiguation {
                    IndexOrComposeWithListExprDisambiguation::Index(_) => HirEagerExpr::Index {
                        owner: owner.to_hir_eager(builder),
                        items: items
                            .iter()
                            .map(|item| item.expr_idx().to_hir_eager(builder))
                            .collect(),
                    },
                    IndexOrComposeWithListExprDisambiguation::ComposeWithList => {
                        todo!()
                    }
                }
            }
            SynExpr::List {
                lbox_token_idx,
                ref items,
                rbox_token_idx,
            } => HirEagerExpr::List {
                items: items
                    .iter()
                    .map(|item| item.expr_idx().to_hir_eager(builder))
                    .collect(),
            },
            SynExpr::BoxColonList {
                lbox_token_idx,
                colon_token_idx,
                ref items,
                rbox_token_idx,
            } => todo!(),
            SynExpr::Block { stmts } => HirEagerExpr::Block {
                stmts: stmts.to_hir_eager(builder),
            },
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExpr::Sorry { token_idx } => todo!(),
            SynExpr::Todo { token_idx } => HirEagerExpr::Todo,
            SynExpr::Err(ref e) => {
                unreachable!(
                    "e = {:?}, path = {:?}",
                    e.debug(builder.db()),
                    builder.path()
                )
            }
        };
        builder.alloc_expr(*self, hir_eager_expr)
    }
}
