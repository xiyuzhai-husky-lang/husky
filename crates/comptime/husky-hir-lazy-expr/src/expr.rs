mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_entity_path::PrincipalEntityPath;
use husky_opr::{BinaryOpr, PrefixOpr, SuffixOpr};
use husky_sema_expr::{SemaExprData, SemaExprIdx};
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
        opd_hir_expr_idx: HirLazyExprIdx,
    },
    Suffix {
        opd_hir_expr_idx: HirLazyExprIdx,
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
    MethodFnCall {
        self_argument: HirLazyExprIdx,
        ident: Ident,
        template_arguments: Option<HirLazyGenericArgumentList>,
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
                path,
                ty_path_disambiguation,
            } => HirLazyExpr::PrincipalEntityPath(*path),
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
            } => HirLazyExpr::CurrentSymbol { ident: *ident },
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
                opr: todo!(),
                ropd: ropd.to_hir_lazy(builder),
            },
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => HirLazyExpr::Be {
                src: src.to_hir_lazy(builder),
                target: target.to_hir_lazy(builder),
            },
            SemaExprData::Prefix {
                opr,
                opd_sema_expr_idx,
                ..
            } => HirLazyExpr::Prefix {
                opr: todo!(),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                ..
            } => HirLazyExpr::Suffix {
                opr: todo!(),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::Application {
                function_sema_expr_idx,
                argument_sema_expr_idx,
            } => {
                todo!()
            }
            SemaExprData::FnCall {
                function_sema_expr_idx,
                template_arguments,
                ritchie_parameter_argument_matches,
                ..
            } => HirLazyExpr::FnCall {
                function: function_sema_expr_idx.to_hir_lazy(builder),
                generic_arguments: template_arguments.as_ref().map(|_| todo!()),
                item_groups: builder.new_call_list_item_groups(ritchie_parameter_argument_matches),
            },
            SemaExprData::GnCall { .. } => unreachable!(),
            SemaExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                ritchie_kind,
                lpar_token,
                ref parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty_sema_expr_idx,
            } => todo!(),
            SemaExprData::Field {
                owner_sema_expr_idx,
                ident_token,
                ..
            } => HirLazyExpr::Field {
                owner: owner_sema_expr_idx.to_hir_lazy(builder),
                ident: ident_token.ident(),
            },
            SemaExprData::MethodApplication {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                template_arguments: ref generic_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => {
                todo!()
            }
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                ident_token,
                template_arguments,
                ritchie_parameter_argument_matches,
                method_dynamic_dispatch,
                ..
            } => HirLazyExpr::MethodFnCall {
                self_argument: self_argument_sema_expr_idx.to_hir_lazy(builder),
                ident: ident_token.ident(),
                template_arguments: template_arguments.as_ref().map(|_| todo!()),
                item_groups: builder.new_call_list_item_groups(ritchie_parameter_argument_matches),
            },
            SemaExprData::MethodGnCall { .. } => {
                todo!()
            }
            SemaExprData::TemplateInstantiation {
                template,
                ref template_arguments,
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
            SemaExprData::Index {
                owner_sema_expr_idx,
                indices,
                index_dynamic_dispatch,
                ..
            } => HirLazyExpr::Index {
                owner: owner_sema_expr_idx.to_hir_lazy(builder),
                items: indices
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_lazy(builder))
                    .collect(),
            },
            SemaExprData::CompositionWithList { owner, items, .. } => {
                todo!()
            }
            SemaExprData::List { items, .. } => HirLazyExpr::List {
                items: items
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_lazy(builder))
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
            SemaExprData::ListFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::NewList {
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
        };
        builder.alloc_expr(*self, hir_lazy_expr)
    }
}
