mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::StaticDispatch;
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::HirConstSymbol;
use husky_sema_expr::{SemaExprData, SemaExprIdx, SemaRitchieParameterArgumentMatch};
use husky_sema_opr::{prefix::SemaPrefixOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::{
    IdentifiableEntityPathExpr, InheritedSynSymbolKind, SynExprData, SynExprIdx, SynStmtIdx,
};
use salsa::debug::ExpectWithDb;
use vec_like::VecMap;

pub type HirEagerExprArena = Arena<HirEagerExprData>;
pub type HirEagerExprIdx = ArenaIdx<HirEagerExprData>;
pub type HirEagerExprIdxRange = ArenaIdxRange<HirEagerExprData>;
pub type HirEagerExprMap<V> = ArenaMap<HirEagerExprData, V>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub enum HirEagerExprData {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    ConstSymbol(HirConstSymbol),
    Variable(HirEagerVariableIdx),
    Binary {
        lopd: HirEagerExprIdx,
        opr: HirBinaryOpr,
        ropd: HirEagerExprIdx,
    },
    Be {
        src: HirEagerExprIdx,
        target: HirEagerBeVariablesPattern,
    },
    Prefix {
        opr: HirPrefixOpr,
        opd_hir_expr_idx: HirEagerExprIdx,
    },
    Suffix {
        opd_hir_expr_idx: HirEagerExprIdx,
        opr: HirSuffixOpr,
    },
    FnCall {
        function_hir_expr_idx: HirEagerExprIdx,
        template_arguments: Option<HirEagerTemplateArgumentList>,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    Field {
        owner_hir_expr_idx: HirEagerExprIdx,
        ident: Ident,
    },
    MethodCall {
        self_argument: HirEagerExprIdx,
        ident: Ident,
        template_arguments: Option<HirEagerTemplateArgumentList>,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    Index {
        owner_hir_expr_idx: HirEagerExprIdx,
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    NewList {
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
    AssociatedFn {
        associated_item_path: AssociatedItemPath,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerTemplateArgumentList {/*todo */}

#[cfg(feature = "rust-syn-gen")]
impl Expr {}

impl ToHirEager for SemaExprIdx {
    type Output = HirEagerExprIdx;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let hir_eager_expr = match self.data(builder.sema_expr_arena_ref()) {
            SemaExprData::Literal(_, _) => {
                HirEagerExprData::Literal(match builder.expr_term(*self) {
                    EtherealTerm::Literal(lit) => lit,
                    _ => unreachable!(),
                })
            }
            SemaExprData::PrincipalEntityPath {
                path_expr_idx,
                path,
                ty_path_disambiguation,
            } => {
                // ad hoc
                HirEagerExprData::PrincipalEntityPath(*path)
            }
            SemaExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
                static_dispatch,
            } => match static_dispatch {
                StaticDispatch::AssociatedFn(signature) => HirEagerExprData::AssociatedFn {
                    associated_item_path: signature.path(),
                },
                StaticDispatch::AssociatedGn => unreachable!(),
            },
            &SemaExprData::InheritedSynSymbol {
                ident,
                regional_token_idx,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
            } => match inherited_syn_symbol_kind {
                InheritedSynSymbolKind::TemplateParameter(_) => todo!(),
                InheritedSynSymbolKind::ParenateParameter { .. }
                | InheritedSynSymbolKind::FieldVariable { .. } => HirEagerExprData::Variable(
                    builder.inherited_syn_symbol_to_hir_eager_variable(inherited_syn_symbol_idx),
                ),
            },
            &SemaExprData::CurrentSynSymbol {
                ident,
                regional_token_idx,
                current_syn_symbol_idx,
                current_syn_symbol_kind,
            } => HirEagerExprData::Variable(
                builder.current_syn_symbol_to_hir_eager_variable(current_syn_symbol_idx),
            ),
            SemaExprData::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx,
                current_syn_symbol_kind,
            } => todo!(),
            SemaExprData::SelfType(_) => {
                unreachable!()
            }
            SemaExprData::SelfValue(_) => {
                HirEagerExprData::Variable(builder.self_value_variable().unwrap())
            }
            &SemaExprData::Binary {
                lopd,
                opr,
                opr_regional_token_idx,
                ref dispatch,
                ropd,
            } => HirEagerExprData::Binary {
                lopd: lopd.to_hir_eager(builder),
                opr: HirBinaryOpr::from_sema(opr),
                ropd: ropd.to_hir_eager(builder),
            },
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => HirEagerExprData::Be {
                src: src.to_hir_eager(builder),
                target: target.to_hir_eager(builder),
            },
            &SemaExprData::Prefix {
                opr,
                opd_sema_expr_idx,
                ..
            } => HirEagerExprData::Prefix {
                opr: HirPrefixOpr::from_sema(opr),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_eager(builder),
            },
            &SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                ..
            } => HirEagerExprData::Suffix {
                opr: HirSuffixOpr::from_sema(opr),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_eager(builder),
            },
            SemaExprData::FunctionApplication {
                function_sema_expr_idx,
                argument_sema_expr_idx,
            } => todo!(),
            SemaExprData::FunctionFnCall {
                function_sema_expr_idx,
                template_arguments,
                lpar_regional_token_idx,
                ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            } => HirEagerExprData::FnCall {
                function_hir_expr_idx: function_sema_expr_idx.to_hir_eager(builder),
                template_arguments: template_arguments.as_ref().map(|_| todo!()),
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
                dot_regional_token_idx,
                ident_token,
                field_dispatch,
            } => HirEagerExprData::Field {
                owner_hir_expr_idx: owner_sema_expr_idx.to_hir_eager(builder),
                ident: ident_token.ident(),
            },
            SemaExprData::MethodApplication { .. } => todo!(),
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                ident_token,
                template_arguments,
                lpar_regional_token_idx,
                ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
                ..
            } => HirEagerExprData::MethodCall {
                self_argument: self_argument_sema_expr_idx.to_hir_eager(builder),
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
            } => return item.to_hir_eager(builder),
            SemaExprData::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::Index {
                owner_sema_expr_idx,
                lbox_regional_token_idx,
                index_sema_list_items: indices,
                ..
            } => HirEagerExprData::Index {
                owner_hir_expr_idx: owner_sema_expr_idx.to_hir_eager(builder),
                items: indices
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_eager(builder))
                    .collect(),
            },
            SemaExprData::CompositionWithList { .. } => {
                todo!()
            }
            SemaExprData::NewList {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => HirEagerExprData::NewList {
                items: items
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_eager(builder))
                    .collect(),
            },
            SemaExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::Block { stmts } => HirEagerExprData::Block {
                stmts: stmts.to_hir_eager(builder),
            },
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => HirEagerExprData::EmptyHtmlTag {
                function_ident: function_ident.ident(),
                arguments: unsafe {
                    VecMap::from_iter_assuming_no_repetitions_unchecked(
                        arguments
                            .iter()
                            .map(|argument| argument.to_hir_eager(builder)),
                    )
                },
            },
            SemaExprData::Sorry { regional_token_idx } => todo!(),
            SemaExprData::Todo { regional_token_idx } => HirEagerExprData::Todo,
            SemaExprData::Unreachable { regional_token_idx } => todo!(),
            SemaExprData::VecFunctor {
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
        builder.alloc_expr(*self, hir_eager_expr)
    }
}
