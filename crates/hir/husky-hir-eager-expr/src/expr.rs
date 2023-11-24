mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::{symbol::runtime_symbol::HirEagerRuntimeSymbolIdx, *};
use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::{FluffyFieldSignature, MethodFluffySignature, StaticDispatch};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    indirections::HirIndirections, instantiation::HirInstantiation, HirConstSymbol,
};
use husky_sema_expr::{SemaExprData, SemaExprIdx, SemaRitchieParameterArgumentMatch};
use husky_syn_expr::InheritedSynSymbolKind;
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
    AssociatedFn {
        associated_item_path: AssociatedItemPath,
    },
    ConstSymbol(HirConstSymbol),
    Variable(HirEagerRuntimeSymbolIdx),
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
    TypeConstructorFnCall {
        path: TypePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    TypeVariantConstructorFnCall {
        path: TypeVariantPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    FunctionFnItemCall {
        path: FugitivePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    AssociatedFunctionFnCall {
        path: AssociatedItemPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    PropsStructField {
        owner_hir_expr_idx: HirEagerExprIdx,
        ident: Ident,
    },
    MemoizedField {
        owner_hir_expr_idx: HirEagerExprIdx,
        ident: Ident,
        path: AssociatedItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
    },
    MethodFnCall {
        self_argument: HirEagerExprIdx,
        ident: Ident,
        path: AssociatedItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerCallListItemGroup; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    Index {
        owner_hir_expr_idx: HirEagerExprIdx,
        // todo: indirections,
        // instantiation
        // path
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
    Unreachable,
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
                path_expr_idx: _,
                path,
                ty_path_disambiguation: _,
            } => {
                // ad hoc
                HirEagerExprData::PrincipalEntityPath(*path)
            }
            SemaExprData::AssociatedItem {
                parent_expr_idx: _,
                parent_path: _,
                colon_colon_regional_token: _,
                ident_token: _,
                static_dispatch,
            } => match static_dispatch {
                StaticDispatch::AssociatedFn(signature) => HirEagerExprData::AssociatedFn {
                    associated_item_path: signature.path(),
                },
                StaticDispatch::AssociatedGn => unreachable!(),
            },
            &SemaExprData::InheritedSynSymbol {
                ident: _,
                regional_token_idx: _,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
            } => match inherited_syn_symbol_kind {
                InheritedSynSymbolKind::TemplateParameter(_) => todo!(),
                InheritedSynSymbolKind::ParenateParameter { .. }
                | InheritedSynSymbolKind::FieldVariable { .. } => HirEagerExprData::Variable(
                    builder
                        .inherited_syn_symbol_to_hir_eager_runtime_symbol(inherited_syn_symbol_idx)
                        .unwrap(),
                ),
            },
            &SemaExprData::CurrentSynSymbol {
                ident: _,
                regional_token_idx: _,
                current_syn_symbol_idx,
                current_syn_symbol_kind: _,
            } => HirEagerExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_eager_runtime_symbol(current_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::FrameVarDecl {
                regional_token_idx: _,
                ident: _,
                frame_var_symbol_idx: _,
                current_syn_symbol_kind: _,
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
                opr_regional_token_idx: _,
                dispatch: _,
                ropd,
            } => HirEagerExprData::Binary {
                lopd: lopd.to_hir_eager(builder),
                opr: HirBinaryOpr::from_sema(opr),
                ropd: ropd.to_hir_eager(builder),
            },
            SemaExprData::Be {
                src,
                be_regional_token_idx: _,
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
                function_sema_expr_idx: _,
                argument_sema_expr_idx: _,
            } => todo!(),
            SemaExprData::FunctionFnCall {
                function_sema_expr_idx,
                template_arguments,
                lpar_regional_token_idx: _,
                ritchie_parameter_argument_matches,
                rpar_regional_token_idx: _,
            } => {
                let function_hir_eager_expr_idx = function_sema_expr_idx.to_hir_eager(builder);
                let template_arguments = template_arguments.as_ref().map(|_| todo!());
                let item_groups =
                    builder.new_call_list_item_groups(ritchie_parameter_argument_matches);
                match builder.hir_eager_expr_arena()[function_hir_eager_expr_idx] {
                    HirEagerExprData::PrincipalEntityPath(path) => match path {
                        PrincipalEntityPath::Module(_) => unreachable!(),
                        PrincipalEntityPath::MajorItem(path) => match path {
                            MajorItemPath::Type(path) => HirEagerExprData::TypeConstructorFnCall {
                                path,
                                instantiation: HirInstantiation::new_empty(),
                                item_groups,
                            },
                            MajorItemPath::Trait(_) => unreachable!(),
                            MajorItemPath::Fugitive(path) => HirEagerExprData::FunctionFnItemCall {
                                path,
                                instantiation: HirInstantiation::new_empty(),
                                item_groups,
                            },
                        },
                        PrincipalEntityPath::TypeVariant(path) => {
                            HirEagerExprData::TypeVariantConstructorFnCall {
                                path,
                                instantiation: HirInstantiation::new_empty(),
                                item_groups,
                            }
                        }
                    },
                    HirEagerExprData::AssociatedFn {
                        associated_item_path,
                    } => HirEagerExprData::AssociatedFunctionFnCall {
                        path: associated_item_path,
                        instantiation: HirInstantiation::new_empty(),
                        item_groups,
                    },
                    _ => todo!(),
                }
            }
            SemaExprData::FunctionGnCall { .. } => unreachable!(),
            SemaExprData::Ritchie {
                ritchie_kind_regional_token_idx: _,
                ritchie_kind: _,
                lpar_token: _,
                parameter_ty_items: _,
                rpar_regional_token_idx: _,
                light_arrow_token: _,
                return_ty_sema_expr_idx: _,
            } => todo!(),
            SemaExprData::Field {
                owner_sema_expr_idx,
                ident_token,
                dispatch,
                ..
            } => match *dispatch.signature() {
                FluffyFieldSignature::PropsStruct { ty: ty2 } => {
                    HirEagerExprData::PropsStructField {
                        owner_hir_expr_idx: owner_sema_expr_idx.to_hir_eager(builder),
                        ident: ident_token.ident(),
                    }
                }
                FluffyFieldSignature::Memoized {
                    ty,
                    path,
                    ref instantiation,
                } => HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx: owner_sema_expr_idx.to_hir_eager(builder),
                    ident: ident_token.ident(),
                    path,
                    indirections: HirIndirections::from_fluffy(dispatch.indirections()),
                    instantiation: HirInstantiation::from_fluffy(
                        instantiation,
                        builder.db(),
                        builder.fluffy_terms(),
                    ),
                },
            },
            SemaExprData::MethodApplication { .. } => todo!(),
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                ident_token,
                dispatch,
                template_arguments,
                ritchie_parameter_argument_matches,
                ..
            } => {
                let MethodFluffySignature::MethodFn(signature) = dispatch.signature() else {
                    unreachable!()
                };
                HirEagerExprData::MethodFnCall {
                    self_argument: self_argument_sema_expr_idx.to_hir_eager(builder),
                    ident: ident_token.ident(),
                    path: signature.path(),
                    item_groups: builder
                        .new_call_list_item_groups(ritchie_parameter_argument_matches),
                    instantiation: HirInstantiation::from_fluffy(
                        signature.instantiation(),
                        builder.db(),
                        builder.fluffy_terms(),
                    ),
                    indirections: HirIndirections::from_fluffy(dispatch.indirections()),
                }
            }
            SemaExprData::MethodGnCall { .. } => {
                todo!()
            }
            SemaExprData::TemplateInstantiation {
                template: _,
                template_arguments: _,
            } => todo!(),
            SemaExprData::At {
                at_regional_token_idx: _,
                place_label_regional_token: _,
            } => todo!(),
            SemaExprData::Unit {
                lpar_regional_token_idx: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Bracketed {
                lpar_regional_token_idx: _,
                item,
                rpar_regional_token_idx: _,
            } => return item.to_hir_eager(builder),
            SemaExprData::NewTuple {
                lpar_regional_token_idx: _,
                items: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Index {
                owner_sema_expr_idx,
                lbox_regional_token_idx: _,
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
                lbox_regional_token_idx: _,
                ref items,
                rbox_regional_token_idx: _,
            } => HirEagerExprData::NewList {
                items: items
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_eager(builder))
                    .collect(),
            },
            SemaExprData::BoxColonList {
                lbox_regional_token_idx: _,
                colon_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Block { stmts } => HirEagerExprData::Block {
                stmts: stmts.to_hir_eager(builder),
            },
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx: _,
                function_ident,
                ref arguments,
                empty_html_ket: _,
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
            SemaExprData::Sorry {
                regional_token_idx: _,
            } => todo!(),
            SemaExprData::Todo {
                regional_token_idx: _,
            } => HirEagerExprData::Todo,
            SemaExprData::Unreachable {
                regional_token_idx: _,
            } => HirEagerExprData::Unreachable,
            SemaExprData::VecFunctor {
                lbox_regional_token_idx: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
        };
        builder.alloc_expr(*self, hir_eager_expr)
    }
}
