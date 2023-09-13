mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_entity_path::PrincipalEntityPath;
use husky_expr_ty::{
    ApplicationOrFunctionCallExprDisambiguation, IndexOrComposeWithListExprDisambiguation,
    MethodCallOrApplicationDisambiguation, SynExprDisambiguation,
};
use husky_opr::{BinaryOpr, PrefixOpr, SuffixOpr};
use husky_syn_expr::{SynExpr, SynExprIdx};
use husky_term_prelude::{RitchieKind, TermLiteral};
use salsa::debug::ExpectWithDb;

pub type HirLazyExprArena = Arena<HirLazyExpr>;
pub type HirLazyExprIdx = ArenaIdx<HirLazyExpr>;
pub type HirLazyExprIdxRange = ArenaIdxRange<HirLazyExpr>;
pub type HirLazyExprMap<V> = ArenaMap<HirLazyExpr, V>;

#[derive(Debug, PartialEq, Eq)]
pub enum HirLazyExpr {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    InheritedSymbol {
        ident: Ident,
        // inherited_symbol_idx: InheritedHirLazySymbolIdx,
        // inherited_symbol_kind: InheritedHirLazySymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        // current_symbol_idx: CurrentHirLazySymbolIdx,
        // current_symbol_kind: CurrentHirLazySymbolKind,
    },
    FrameVarDecl {
        ident: Ident,
        // frame_var_symbol_idx: CurrentHirLazySymbolIdx,
        // current_symbol_kind: CurrentHirLazySymbolKind,
    },
    SelfType,
    SelfValue,
    Binary {
        lopd: HirLazyExprIdx,
        opr: BinaryOpr,
        ropd: HirLazyExprIdx,
    },
    Be {
        src: HirLazyExprIdx,
        target: HirLazyBeVariablesPattern,
    },
    Prefix {
        opr: PrefixOpr,
        opd: HirLazyExprIdx,
    },
    Suffix {
        opd: HirLazyExprIdx,
        opr: SuffixOpr,
    },
    FnCall {
        function: HirLazyExprIdx,
        generic_arguments: Option<HirLazyGenericArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    GnCall {
        function: HirLazyExprIdx,
        generic_arguments: Option<HirLazyGenericArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    Field {
        owner: HirLazyExprIdx,
        ident: Ident,
    },
    MethodCall {
        self_argument: HirLazyExprIdx,
        ident: Ident,
        generic_arguments: Option<HirLazyGenericArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    Index {
        owner: HirLazyExprIdx,
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    List {
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    Block {
        stmts: HirLazyStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        function_ident: Ident,
        arguments: IdentMap<HirLazyHtmlArgumentExpr>,
    },
    Todo,
    AssociatedFn,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyGenericArgumentList {/*todo */}

impl ToHirLazy for SynExprIdx {
    type Output = HirLazyExprIdx;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let hir_lazy_expr = match builder.syn_expr_region_data()[*self] {
            SynExpr::Literal(_, _) => {
                let EtherealTerm::Literal(lit) = builder.expr_term(*self) else {
                    unreachable!()
                };
                HirLazyExpr::Literal(lit)
            }
            SynExpr::PrincipalEntityPath {
                item_path_expr,
                opt_path,
            } => {
                let path = opt_path.expect("whatever");
                // ad hoc
                HirLazyExpr::PrincipalEntityPath(path)
            }
            SynExpr::ScopeResolution {
                parent_expr_idx,
                colon_colon_regional_token,
                ident_token,
            } => todo!(),
            SynExpr::InheritedSymbol {
                ident,
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            SynExpr::CurrentSymbol {
                ident,
                regional_token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => HirLazyExpr::CurrentSymbol { ident },
            SynExpr::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SynExpr::SelfType(_) => todo!(),
            SynExpr::SelfValue(_) => todo!(),
            SynExpr::Binary {
                lopd, opr, ropd, ..
            } => HirLazyExpr::Binary {
                lopd: lopd.to_hir_lazy(builder),
                opr,
                ropd: ropd.to_hir_lazy(builder),
            },
            SynExpr::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => HirLazyExpr::Be {
                src: src.to_hir_lazy(builder),
                target: target
                    .as_ref()
                    .expect_with_db(builder.db(), "hir stage no errors")
                    .to_hir_lazy(builder),
            },
            SynExpr::Prefix { opr, opd, .. } => HirLazyExpr::Prefix {
                opr,
                opd: opd.to_hir_lazy(builder),
            },
            SynExpr::Suffix { opd, opr, .. } => HirLazyExpr::Suffix {
                opr,
                opd: opd.to_hir_lazy(builder),
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
                    } => HirLazyExpr::FnCall {
                        function: function.to_hir_lazy(builder),
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
            } => {
                let SynExprDisambiguation::FunctionCall {
                    ritchie_kind,
                    ritchie_parameter_argument_matches,
                } = builder.expr_disambiguation(*self)
                else {
                    unreachable!()
                };
                let ritchie_parameter_argument_matches = ritchie_parameter_argument_matches
                    .as_ref()
                    .expect("hir stage no errors");
                match ritchie_kind {
                    RitchieKind::FnType => HirLazyExpr::FnCall {
                        function: function.to_hir_lazy(builder),
                        generic_arguments: generic_arguments.as_ref().map(|_| todo!()),
                        item_groups: builder
                            .new_call_list_item_groups(ritchie_parameter_argument_matches),
                    },
                    RitchieKind::FnTrait => todo!(),
                    RitchieKind::FnMutTrait => todo!(),
                    RitchieKind::GnType => todo!(),
                }
            }
            SynExpr::Field {
                owner, ident_token, ..
            } => HirLazyExpr::Field {
                owner: owner.to_hir_lazy(builder),
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
                        HirLazyExpr::MethodCall {
                            self_argument: self_argument.to_hir_lazy(builder),
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
            } => todo!(),
            SynExpr::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => {
                let SynExprDisambiguation::IndexOrComposeWithList(disambiguation) =
                    builder.expr_disambiguation(*self)
                else {
                    unreachable!()
                };
                match disambiguation {
                    IndexOrComposeWithListExprDisambiguation::Index(_) => HirLazyExpr::Index {
                        owner: owner.to_hir_lazy(builder),
                        items: items
                            .iter()
                            .map(|item| item.expr_idx().to_hir_lazy(builder))
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
            } => HirLazyExpr::List {
                items: items
                    .iter()
                    .map(|item| item.expr_idx().to_hir_lazy(builder))
                    .collect(),
            },
            SynExpr::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SynExpr::Block { stmts } => HirLazyExpr::Block {
                stmts: stmts.to_hir_lazy(builder),
            },
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExpr::Sorry { regional_token_idx } => todo!(),
            SynExpr::Todo { regional_token_idx } => todo!(),
            SynExpr::Err(_) => todo!(),
        };
        builder.alloc_expr(*self, hir_lazy_expr)
    }
}
