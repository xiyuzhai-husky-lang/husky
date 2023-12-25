mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{
    AssociatedItemPath, FugitivePath, MajorItemPath, PrincipalEntityPath, TraitForTypeItemPath,
    TypePath, TypeVariantPath,
};
use husky_fluffy_term::{FluffyFieldSignature, MethodFluffySignature};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    indirections::HirIndirections, instantiation::HirInstantiation, HirConstSymbol,
    HirTemplateArguments, HirType,
};
use husky_sema_expr::{SemaExprData, SemaExprIdx};
use husky_sema_opr::{binary::SemaBinaryOpr, suffix::SemaSuffixOpr};
use husky_term_prelude::TermLiteral;
use idx_arena::ArenaRef;

pub type HirLazyExprArena = Arena<HirLazyExprData>;
pub type HirLazyExprArenaRef<'a> = ArenaRef<'a, HirLazyExprData>;
pub type HirLazyExprIdx = ArenaIdx<HirLazyExprData>;
pub type HirLazyExprIdxRange = ArenaIdxRange<HirLazyExprData>;
pub type HirLazyExprMap<V> = ArenaMap<HirLazyExprData, V>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirLazyExprDb, jar = HirLazyExprJar)]
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
    As {
        opd: HirLazyExprIdx,
        ty: HirType,
    },
    TypeConstructorFnCall {
        path: TypePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    TypeVariantConstructorFnCall {
        path: TypeVariantPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    FunctionFnItemCall {
        path: FugitivePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    FunctionGnItemCall {
        path: FugitivePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    AssociatedFunctionFnCall {
        path: AssociatedItemPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    PropsStructField {
        owner: HirLazyExprIdx,
        owner_base_ty: HirType,
        ident: Ident,
    },
    MemoizedField {
        owner: HirLazyExprIdx,
        ident: Ident,
        path: AssociatedItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
    },
    MethodFnCall {
        self_argument: HirLazyExprIdx,
        ident: Ident,
        path: AssociatedItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
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
    Unreachable,
    AssociatedFn {
        // ad hoc, needs more
        path: AssociatedItemPath,
    },
    Unveil {
        unveil_associated_fn_path: TraitForTypeItemPath,
        instantiation: HirInstantiation,
        opd_hir_expr_idx: HirLazyExprIdx,
    },
    Unwrap {
        opd_hir_expr_idx: HirLazyExprIdx,
    },
}

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
            SemaExprData::PrincipalEntityPath { path, .. } => {
                HirLazyExprData::PrincipalEntityPath(path)
            }
            SemaExprData::AssociatedItem { .. } => todo!(),
            SemaExprData::InheritedSynSymbol {
                inherited_syn_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .inherited_syn_symbol_to_hir_lazy_variable(inherited_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::CurrentSynSymbol {
                current_syn_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_lazy_variable(current_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::FrameVarDecl {
                frame_var_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_lazy_variable(frame_var_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::SelfType(_) => todo!(),
            SemaExprData::SelfValue(_) => todo!(),
            SemaExprData::Binary {
                lopd, opr, ropd, ..
            } => match opr {
                SemaBinaryOpr::As => HirLazyExprData::As {
                    opd: lopd.to_hir_lazy(builder),
                    ty: todo!(),
                },
                _ => HirLazyExprData::Binary {
                    lopd: lopd.to_hir_lazy(builder),
                    opr: HirBinaryOpr::from_sema(opr),
                    ropd: ropd.to_hir_lazy(builder),
                },
            },
            SemaExprData::Be {
                src,
                be_regional_token_idx: _,
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
                opr: HirPrefixOpr::from_sema(
                    opr,
                    builder.expr_ty(opd_sema_expr_idx),
                    builder.db(),
                    builder.fluffy_terms(),
                ),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                ..
            } => match opr {
                _ => HirLazyExprData::Suffix {
                    opr: HirSuffixOpr::from_sema(opr),
                    opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
                },
            },
            SemaExprData::Unveil {
                ref unveil_output_ty_signature,
                unveil_associated_fn_path,
                opd_sema_expr_idx,
                ..
            } => HirLazyExprData::Unveil {
                unveil_associated_fn_path,
                instantiation: HirInstantiation::from_ethereal(
                    unveil_output_ty_signature.instantiation(),
                    builder.db(),
                ),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::Unwrap {
                opd_sema_expr_idx, ..
            } => HirLazyExprData::Unwrap {
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::FunctionApplication {
                function_sema_expr_idx: _,
                argument_sema_expr_idx: _,
            } => {
                todo!()
            }
            SemaExprData::FunctionRitchieCall {
                function_sema_expr_idx,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let function_hir_lazy_expr_idx = function_sema_expr_idx.to_hir_lazy(builder);
                let item_groups =
                    builder.new_call_list_item_groups(ritchie_parameter_argument_matches);
                match *builder.sema_expr_arena_ref()[function_sema_expr_idx].data() {
                    SemaExprData::PrincipalEntityPath {
                        path,
                        ref instantiation,
                        ..
                    } => match path {
                        PrincipalEntityPath::Module(_) => unreachable!(),
                        PrincipalEntityPath::MajorItem(path) => match path {
                            MajorItemPath::Type(path) => HirLazyExprData::TypeConstructorFnCall {
                                path,
                                // ad hoc
                                instantiation: HirInstantiation::new_empty(false),
                                item_groups,
                            },
                            MajorItemPath::Trait(_) => unreachable!(),
                            MajorItemPath::Fugitive(path) => match path.fugitive_kind(builder.db())
                            {
                                FugitiveKind::FunctionFn => HirLazyExprData::FunctionFnItemCall {
                                    path,
                                    instantiation: HirInstantiation::from_fluffy(
                                        instantiation.as_ref().unwrap(),
                                        builder.db(),
                                        builder.fluffy_terms(),
                                    ),
                                    item_groups,
                                },
                                FugitiveKind::FunctionGn => HirLazyExprData::FunctionGnItemCall {
                                    path,
                                    instantiation: HirInstantiation::from_fluffy(
                                        instantiation.as_ref().unwrap(),
                                        builder.db(),
                                        builder.fluffy_terms(),
                                    ),
                                    item_groups,
                                },
                                FugitiveKind::AliasType => todo!(),
                                FugitiveKind::Val => todo!(),
                            },
                        },
                        PrincipalEntityPath::TypeVariant(path) => {
                            HirLazyExprData::TypeVariantConstructorFnCall {
                                path,
                                instantiation: todo!(),
                                item_groups,
                            }
                        }
                    },
                    SemaExprData::AssociatedItem {
                        ref static_dispatch,
                        ..
                    } => todo!(),
                    _ => todo!(),
                }
            }
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
                owner_ty,
                ident_token,
                ref dispatch,
                ..
            } => match *dispatch.signature() {
                FluffyFieldSignature::PropsStruct { ty } => HirLazyExprData::PropsStructField {
                    owner: owner_sema_expr_idx.to_hir_lazy(builder),
                    owner_base_ty: HirType::from_fluffy(
                        owner_ty,
                        builder.db(),
                        builder.fluffy_terms(),
                    )
                    .unwrap(),
                    ident: ident_token.ident(),
                },
                FluffyFieldSignature::Memoized {
                    ty,
                    path,
                    ref instantiation,
                } => {
                    debug_assert!(instantiation.separator().is_some());
                    HirLazyExprData::MemoizedField {
                        owner: owner_sema_expr_idx.to_hir_lazy(builder),
                        ident: ident_token.ident(),
                        path,
                        indirections: HirIndirections::from_fluffy(dispatch.indirections()),
                        instantiation: HirInstantiation::from_fluffy(
                            instantiation,
                            builder.db(),
                            builder.fluffy_terms(),
                        ),
                    }
                }
            },
            SemaExprData::MethodApplication {
                self_argument: _,
                dot_regional_token_idx: _,
                ident_token: _,
                template_arguments: _,
                lpar_regional_token_idx: _,
                items: _,
                rpar_regional_token_idx: _,
            } => {
                todo!()
            }
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                ident_token,
                ref dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let MethodFluffySignature::MethodFn(signature) = dispatch.signature() else {
                    unreachable!()
                };
                HirLazyExprData::MethodFnCall {
                    self_argument: self_argument_sema_expr_idx.to_hir_lazy(builder),
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
                item: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemaExprData::NewTuple {
                lpar_regional_token_idx: _,
                items: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Index {
                owner_sema_expr_idx,
                ref index_sema_list_items,
                ..
            } => HirLazyExprData::Index {
                owner: owner_sema_expr_idx.to_hir_lazy(builder),
                items: index_sema_list_items
                    .iter()
                    .map(|item| item.sema_expr_idx.to_hir_lazy(builder))
                    .collect(),
            },
            SemaExprData::CompositionWithList { .. } => {
                todo!()
            }
            SemaExprData::NewList { ref items, .. } => HirLazyExprData::NewList {
                items: items
                    .iter()
                    .map(|item| item.sema_expr_idx.to_hir_lazy(builder))
                    .collect(),
            },
            SemaExprData::BoxColonList {
                lbox_regional_token_idx: _,
                colon_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Block { stmts } => HirLazyExprData::Block {
                stmts: stmts.to_hir_lazy(builder),
            },
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx: _,
                function_ident: _,
                arguments: _,
                empty_html_ket: _,
            } => todo!(),
            SemaExprData::Sorry {
                regional_token_idx: _,
            } => todo!(),
            SemaExprData::Todo {
                regional_token_idx: _,
            } => todo!(),
            SemaExprData::Unreachable {
                regional_token_idx: _,
            } => todo!(),
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
        builder.alloc_expr(*self, hir_lazy_expr)
    }
}
