mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_ethereal_term::EtherealTerm;
use husky_expr_ty::{
    ApplicationOrFunctionCallExprDisambiguation, IndexOrComposeWithListExprDisambiguation,
    MethodCallOrApplicationDisambiguation, RitchieParameterArgumentMatch, SynExprDisambiguation,
};
use husky_fluffy_term::StaticDispatch;
use husky_syn_expr::{SynExpr, SynExprIdx, SynStmtIdx};
use salsa::debug::ExpectWithDb;
use vec_like::VecMap;

pub type HirEagerExprArena = Arena<HirEagerExpr>;
pub type HirEagerExprIdx = ArenaIdx<HirEagerExpr>;
pub type HirEagerExprIdxRange = ArenaIdxRange<HirEagerExpr>;
pub type HirEagerExprMap<V> = ArenaMap<HirEagerExpr, V>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    FnCall {
        function: HirEagerExprIdx,
        generic_arguments: Option<HirEagerGenericArgumentList>,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    Field {
        owner: HirEagerExprIdx,
        ident: Ident,
    },
    MethodCall {
        self_argument: HirEagerExprIdx,
        ident: Ident,
        generic_arguments: Option<HirEagerGenericArgumentList>,
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
        arguments: IdentMap<HirEagerHtmlArgumentExpr>,
    },
    Todo,
    AssociatedFn,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerGenericArgumentList {/*todo */}

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
                colon_colon_regional_token,
                ident_token,
            } => {
                let SynExprDisambiguation::StaticDispatch(dispatch) =
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
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => HirEagerExpr::InheritedSymbol { ident },
            SynExpr::CurrentSymbol {
                ident,
                regional_token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => HirEagerExpr::CurrentSymbol { ident },
            SynExpr::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SynExpr::SelfType(_) => HirEagerExpr::SelfType,
            SynExpr::SelfValue(_) => HirEagerExpr::SelfValue,
            SynExpr::Binary {
                lopd,
                opr,
                opr_regional_token_idx,
                ropd,
            } => HirEagerExpr::Binary {
                lopd: lopd.to_hir_eager(builder),
                opr,
                ropd: ropd.to_hir_eager(builder),
            },
            SynExpr::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => HirEagerExpr::Be {
                src: src.to_hir_eager(builder),
                target: target
                    .as_ref()
                    .expect_with_db(builder.db(), "hir stage no errors")
                    .to_hir_eager(builder),
            },
            SynExpr::Prefix {
                opr,
                opr_regional_token_idx,
                opd,
            } => HirEagerExpr::Prefix {
                opr,
                opd: opd.to_hir_eager(builder),
            },
            SynExpr::Suffix {
                opd,
                opr,
                opr_regional_token_idx,
            } => HirEagerExpr::Suffix {
                opr,
                opd: opd.to_hir_eager(builder),
            },
            SynExpr::FunctionApplicationOrCall {
                function,
                ref generic_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => {
                let SynExprDisambiguation::ApplicationOrFunctionCall(disambiguation) =
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
                    } => HirEagerExpr::FnCall {
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
                ritchie_kind_regional_token_idx,
                ritchie_kind,
                lpar_token,
                ref parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty_expr,
            } => todo!(),
            SynExpr::FunctionCall {
                function,
                ref generic_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SynExpr::Field {
                owner,
                dot_regional_token_idx,
                ident_token,
            } => HirEagerExpr::Field {
                owner: owner.to_hir_eager(builder),
                ident: ident_token.ident(),
            },
            SynExpr::MethodApplicationOrCall {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                ref generic_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => {
                // todo: method application should be ignored
                let SynExprDisambiguation::MethodCallOrApplication(disambiguation) =
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
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => todo!(),
            SynExpr::Bracketed {
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => return item.to_hir_eager(builder),
            SynExpr::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                ..
            } => {
                let SynExprDisambiguation::IndexOrComposeWithList(disambiguation) =
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
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => HirEagerExpr::List {
                items: items
                    .iter()
                    .map(|item| item.expr_idx().to_hir_eager(builder))
                    .collect(),
            },
            SynExpr::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SynExpr::Block { stmts } => HirEagerExpr::Block {
                stmts: stmts.to_hir_eager(builder),
            },
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => HirEagerExpr::EmptyHtmlTag {
                function_ident: function_ident.ident(),
                arguments: unsafe {
                    VecMap::from_iter_assuming_no_repetitions_unchecked(
                        arguments
                            .iter()
                            .map(|argument| argument.to_hir_eager(builder)),
                    )
                },
            },
            SynExpr::Sorry { regional_token_idx } => todo!(),
            SynExpr::Todo { regional_token_idx } => HirEagerExpr::Todo,
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
