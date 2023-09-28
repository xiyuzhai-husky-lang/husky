mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_entity_path::PrincipalEntityPath;
use husky_opr::{BinaryOpr, PrefixOpr, SuffixOpr};
use husky_sema_expr::{
    ApplicationOrFunctionCallExprDisambiguation, IndexOrComposeWithListExprDisambiguation,
    MethodCallOrApplicationDisambiguation, SemaExprData, SemaExprIdx, SynExprDisambiguation,
};
use husky_syn_expr::{IdentifiableEntityPathExpr, SynExprData, SynExprIdx};
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

impl ToHirLazy for SemaExprIdx {
    type Output = HirLazyExprIdx;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let hir_lazy_expr = match self.data(todo!()) {
            SemaExprData::Literal(_, _) => {
                let EtherealTerm::Literal(lit) = builder.expr_term(*self) else {
                    unreachable!()
                };
                HirLazyExpr::Literal(lit)
            }
            SemaExprData::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            } => {
                let path = opt_path.expect("whatever");
                // ad hoc
                HirLazyExpr::PrincipalEntityPath(path)
            }
            SemaExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
                static_dispatch,
            } => todo!(),
            SemaExprData::InheritedSymbol {
                ident,
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            SemaExprData::CurrentSymbol {
                ident,
                regional_token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => HirLazyExpr::CurrentSymbol { ident },
            SemaExprData::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SemaExprData::SelfType(_) => todo!(),
            SemaExprData::SelfValue(_) => todo!(),
            SemaExprData::Binary {
                lopd, opr, ropd, ..
            } => HirLazyExpr::Binary {
                lopd: lopd.to_hir_lazy(builder),
                opr,
                ropd: ropd.to_hir_lazy(builder),
            },
            SemaExprData::Be {
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
            SemaExprData::Prefix { opr, opd, .. } => HirLazyExpr::Prefix {
                opr,
                opd: opd.to_hir_lazy(builder),
            },
            SemaExprData::Suffix { opd, opr, .. } => HirLazyExpr::Suffix {
                opr,
                opd: opd.to_hir_lazy(builder),
            },
            SemaExprData::FunctionApplicationOrCall {
                function,
                template_arguments: ref generic_arguments,
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
            SemaExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                ritchie_kind,
                lpar_token,
                ref parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty_sema_expr_idx,
            } => todo!(),
            SemaExprData::FunctionCall {
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
            SemaExprData::Field {
                owner, ident_token, ..
            } => HirLazyExpr::Field {
                owner: owner.to_hir_lazy(builder),
                ident: ident_token.ident(),
            },
            SemaExprData::MethodApplicationOrCall {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                template_arguments: ref generic_arguments,
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
            SemaExprData::TemplateInstantiation {
                template,
                ref generic_arguments,
            } => todo!(),
            SemaExprData::ExplicitApplication {
                function_expr_idx,
                argument_expr_idx,
            } => todo!(),
            SemaExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SemaExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::Bracketed {
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::IndexOrCompositionWithList {
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
            SemaExprData::List {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => HirLazyExpr::List {
                items: items
                    .iter()
                    .map(|item| item.expr_idx().to_hir_lazy(builder))
                    .collect(),
            },
            SemaExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::Block { stmts } => HirLazyExpr::Block {
                stmts: stmts.to_hir_lazy(builder),
            },
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SemaExprData::Sorry { regional_token_idx } => todo!(),
            SemaExprData::Todo { regional_token_idx } => todo!(),
            SemaExprData::Unreachable { regional_token_idx } => todo!(),
            SemaExprData::Err(_) => todo!(),
        };
        builder.alloc_expr(*self, hir_lazy_expr)
    }
}
