mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_entity_path::PrincipalEntityPath;
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::HirConstSymbol;
use husky_sema_expr::{SemaExprData, SemaExprIdx};
use husky_sema_opr::prefix::SemaPrefixOpr;
use husky_sema_opr::suffix::SemaSuffixOpr;
use husky_syn_expr::{IdentifiableEntityPathExpr, SynExprData, SynExprIdx};
use husky_term_prelude::{RitchieKind, TermLiteral};
use idx_arena::ArenaRef;
use salsa::debug::ExpectWithDb;

pub type HirLazyExprArena = Arena<HirLazyExprData>;
pub type HirLazyExprArenaRef<'a> = ArenaRef<'a, HirLazyExprData>;
pub type HirLazyExprIdx = ArenaIdx<HirLazyExprData>;
pub type HirLazyExprIdxRange = ArenaIdxRange<HirLazyExprData>;
pub type HirLazyExprMap<V> = ArenaMap<HirLazyExprData, V>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirLazyExprDb)]
pub enum HirLazyExprData {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    ConstSymbol(HirConstSymbol),
    Variable(HirLazyVariableIdx),
    Binary {
        lopd: HirLazyExprIdx,
        opr: HirBinaryOpr,
        ropd: HirLazyExprIdx,
    },
    Be {
        src: HirLazyExprIdx,
        target: HirLazyBeVariablesPattern,
    },
    Prefix {
        opr: HirPrefixOpr,
        opd_hir_expr_idx: HirLazyExprIdx,
    },
    Suffix {
        opd_hir_expr_idx: HirLazyExprIdx,
        opr: HirSuffixOpr,
    },
    FnCall {
        function: HirLazyExprIdx,
        generic_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    GnCall {
        function: HirLazyExprIdx,
        generic_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    Field {
        owner: HirLazyExprIdx,
        ident: Ident,
    },
    MethodFnCall {
        self_argument: HirLazyExprIdx,
        ident: Ident,
        template_arguments: Option<HirLazyTemplateArgumentList>,
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
    NewList {
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyTemplateArgumentList {/*todo */}

impl ToHirLazy for SemaExprIdx {
    type Output = HirLazyExprIdx;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let hir_lazy_expr = match *self.data(builder.sema_expr_arena_ref()) {
            SemaExprData::Literal(_, _) => {
                let EtherealTerm::Literal(lit) = builder.expr_term(*self) else {
                    unreachable!()
                };
                HirLazyExprData::Literal(lit)
            }
            SemaExprData::PrincipalEntityPath {
                path_expr_idx,
                path,
                ty_path_disambiguation,
            } => HirLazyExprData::PrincipalEntityPath(path),
            SemaExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
                ref static_dispatch,
            } => todo!(),
            SemaExprData::InheritedSynSymbol {
                ident,
                regional_token_idx,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
            } => HirLazyExprData::Variable(
                builder
                    .inherited_syn_symbol_to_hir_lazy_variable(inherited_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::CurrentSynSymbol {
                ident,
                regional_token_idx,
                current_syn_symbol_idx,
                current_syn_symbol_kind,
            } => HirLazyExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_lazy_variable(current_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx,
                current_syn_symbol_kind,
            } => HirLazyExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_lazy_variable(frame_var_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::SelfType(_) => todo!(),
            SemaExprData::SelfValue(_) => todo!(),
            SemaExprData::Binary {
                lopd, opr, ropd, ..
            } => HirLazyExprData::Binary {
                lopd: lopd.to_hir_lazy(builder),
                opr: HirBinaryOpr::from_sema(opr),
                ropd: ropd.to_hir_lazy(builder),
            },
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => HirLazyExprData::Be {
                src: src.to_hir_lazy(builder),
                target: target.to_hir_lazy(builder),
            },
            SemaExprData::Prefix {
                opr,
                opd_sema_expr_idx,
                ..
            } => HirLazyExprData::Prefix {
                opr: HirPrefixOpr::from_sema(opr),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                ..
            } => HirLazyExprData::Suffix {
                opr: HirSuffixOpr::from_sema(opr),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::FunctionApplication {
                function_sema_expr_idx,
                argument_sema_expr_idx,
            } => {
                todo!()
            }
            SemaExprData::FunctionFnCall {
                function_sema_expr_idx,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => HirLazyExprData::FnCall {
                function: function_sema_expr_idx.to_hir_lazy(builder),
                generic_arguments: template_arguments.as_ref().map(|_| todo!()),
                item_groups: builder.new_call_list_item_groups(ritchie_parameter_argument_matches),
            },
            SemaExprData::FunctionGnCall { .. } => unreachable!(),
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
            } => HirLazyExprData::Field {
                owner: owner_sema_expr_idx.to_hir_lazy(builder),
                ident: ident_token.ident(),
            },
            SemaExprData::MethodApplication {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => {
                todo!()
            }
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                ident_token,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ref dispatch,
                ..
            } => HirLazyExprData::MethodFnCall {
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
                ref index_sema_list_items,
                ref index_dynamic_dispatch,
                ..
            } => HirLazyExprData::Index {
                owner: owner_sema_expr_idx.to_hir_lazy(builder),
                items: index_sema_list_items
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_lazy(builder))
                    .collect(),
            },
            SemaExprData::CompositionWithList {
                owner, ref items, ..
            } => {
                todo!()
            }
            SemaExprData::NewList { ref items, .. } => HirLazyExprData::NewList {
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
            SemaExprData::Block { stmts } => HirLazyExprData::Block {
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
            SemaExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::NewList {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
        };
        builder.alloc_expr(*self, hir_lazy_expr)
    }
}
