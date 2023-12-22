mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::{coersion::HirEagerCoersion, symbol::runtime_symbol::HirEagerRuntimeSymbolIdx, *};
use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::{FluffyFieldSignature, MethodFluffySignature, StaticDispatch};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    instantiation::HirInstantiation, place::HirPlace, ritchie::HirEagerContract, HirConstSymbol,
    HirType,
};
use husky_print_utils::p;
use husky_sema_expr::{SemaExprData, SemaExprIdx, SemaRitchieParameterArgumentMatch};
use husky_sema_opr::{binary::SemaBinaryOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::{InheritedSynSymbolKind, InheritedTemplateParameterSynSymbol};
use vec_like::VecMap;

pub type HirEagerExprArena = Arena<HirEagerExprEntry>;
pub type HirEagerExprIdx = ArenaIdx<HirEagerExprEntry>;
pub type HirEagerExprIdxRange = ArenaIdxRange<HirEagerExprEntry>;
pub type HirEagerExprMap<V> = ArenaMap<HirEagerExprEntry, V>;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerExprEntry {
    pub data: HirEagerExprData,
    pub ty_place: HirPlace,
    pub is_ty_always_copyable: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub enum HirEagerExprData {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    AssociatedFn {
        associated_item_path: AssociatedItemPath,
    },
    ConstSymbol {
        ident: Ident,
    },
    // (HirConstSymbol),
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
    Unveil {
        unveil_associated_fn_path: TraitForTypeItemPath,
        instantiation: HirInstantiation,
        return_ty: HirType,
        opd_hir_expr_idx: HirEagerExprIdx,
    },
    Unwrap {
        opd_hir_expr_idx: HirEagerExprIdx,
    },
    As {
        opd: HirEagerExprIdx,
        ty: HirType,
    },
    TypeConstructorFnCall {
        path: TypePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerRitchieParameterArgumentMatch; 4]>,
    },
    TypeVariantConstructorCall {
        path: TypeVariantPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerRitchieParameterArgumentMatch; 4]>,
    },
    FunctionFnCall {
        path: FugitivePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerRitchieParameterArgumentMatch; 4]>,
    },
    AssociatedFunctionFnCall {
        path: AssociatedItemPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerRitchieParameterArgumentMatch; 4]>,
    },
    PropsStructField {
        owner_hir_expr_idx: HirEagerExprIdx,
        ident: Ident,
        field_ty: HirType,
    },
    MemoizedField {
        owner_hir_expr_idx: HirEagerExprIdx,
        ident: Ident,
        path: AssociatedItemPath,
    },
    MethodFnCall {
        self_argument: HirEagerExprIdx,
        self_contract: HirEagerContract,
        ident: Ident,
        path: AssociatedItemPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirEagerRitchieParameterArgumentMatch; 4]>,
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
    Unreachable,
}

impl ToHirEager for SemaExprIdx {
    type Output = HirEagerExprIdx;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let data = match *self.data(builder.sema_expr_arena_ref()) {
            SemaExprData::Literal(_, _) => {
                HirEagerExprData::Literal(match builder.expr_term(*self) {
                    EtherealTerm::Literal(lit) => lit,
                    _ => unreachable!(),
                })
            }
            SemaExprData::PrincipalEntityPath { path, .. } => {
                // ad hoc
                HirEagerExprData::PrincipalEntityPath(path)
            }
            SemaExprData::AssociatedItem {
                ref static_dispatch,
                ..
            } => match static_dispatch {
                StaticDispatch::AssociatedFn(signature) => HirEagerExprData::AssociatedFn {
                    associated_item_path: signature.path(),
                },
                StaticDispatch::AssociatedGn => unreachable!(),
            },
            SemaExprData::InheritedSynSymbol {
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
                ..
            } => match inherited_syn_symbol_kind {
                InheritedSynSymbolKind::TemplateParameter(symbol) => match symbol {
                    InheritedTemplateParameterSynSymbol::Lifetime { label } => {
                        todo!()
                    }
                    InheritedTemplateParameterSynSymbol::Place { label } => todo!(),
                    InheritedTemplateParameterSynSymbol::Type { ident } => todo!(),
                    InheritedTemplateParameterSynSymbol::Constant { ident } => {
                        HirEagerExprData::ConstSymbol { ident }
                    }
                },
                InheritedSynSymbolKind::ParenateParameter { .. }
                | InheritedSynSymbolKind::FieldVariable { .. } => HirEagerExprData::Variable(
                    builder
                        .inherited_syn_symbol_to_hir_eager_runtime_symbol(inherited_syn_symbol_idx)
                        .unwrap(),
                ),
            },
            SemaExprData::CurrentSynSymbol {
                current_syn_symbol_idx,
                ..
            } => HirEagerExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_eager_runtime_symbol(current_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::FrameVarDecl { .. } => todo!(),
            SemaExprData::SelfType(_) => {
                unreachable!()
            }
            SemaExprData::SelfValue(_) => {
                HirEagerExprData::Variable(builder.self_value_variable().unwrap())
            }
            SemaExprData::Binary {
                lopd, opr, ropd, ..
            } => match opr {
                SemaBinaryOpr::As => HirEagerExprData::As {
                    opd: lopd.to_hir_eager(builder),
                    ty: builder.expr_term_hir_ty(ropd).unwrap(),
                },
                _ => HirEagerExprData::Binary {
                    lopd: lopd.to_hir_eager(builder),
                    opr: HirBinaryOpr::from_sema(opr),
                    ropd: ropd.to_hir_eager(builder),
                },
            },
            SemaExprData::Be {
                src, ref target, ..
            } => HirEagerExprData::Be {
                src: src.to_hir_eager(builder),
                target: target.to_hir_eager(builder),
            },
            SemaExprData::Prefix {
                opr,
                opd_sema_expr_idx,
                ..
            } => HirEagerExprData::Prefix {
                opr: HirPrefixOpr::from_sema(
                    opr,
                    builder.expr_ty(opd_sema_expr_idx),
                    builder.db(),
                    builder.fluffy_terms(),
                ),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_eager(builder),
            },
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                ..
            } => match opr {
                SemaSuffixOpr::ComposeWithOption => unreachable!(),
                SemaSuffixOpr::ComposeWithNot => unreachable!(),
                SemaSuffixOpr::Incr | SemaSuffixOpr::Decr => HirEagerExprData::Suffix {
                    opr: HirSuffixOpr::from_sema(opr),
                    opd_hir_expr_idx: opd_sema_expr_idx.to_hir_eager(builder),
                },
            },
            SemaExprData::Unveil {
                return_ty,
                ref unveil_output_ty_signature,
                unveil_associated_fn_path,
                opd_sema_expr_idx,
                ..
            } => {
                let db = builder.db();
                HirEagerExprData::Unveil {
                    unveil_associated_fn_path,
                    instantiation: HirInstantiation::from_ethereal(
                        unveil_output_ty_signature.instantiation(),
                        db,
                    ),
                    opd_hir_expr_idx: opd_sema_expr_idx.to_hir_eager(builder),
                    return_ty: HirType::from_ethereal(return_ty, db).unwrap(),
                }
            }
            SemaExprData::Unwrap {
                opd_sema_expr_idx, ..
            } => HirEagerExprData::Unwrap {
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_eager(builder),
            },
            SemaExprData::FunctionApplication { .. } => unreachable!(),
            SemaExprData::FunctionRitchieCall {
                function_sema_expr_idx,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let db = builder.db();
                let template_arguments = template_arguments.as_ref().map(|_| todo!());
                let item_groups =
                    builder.new_call_list_item_groups(ritchie_parameter_argument_matches);
                match *builder.sema_expr_arena_ref()[function_sema_expr_idx].data() {
                    SemaExprData::PrincipalEntityPath {
                        path,
                        ref instantiation,
                        ..
                    } => {
                        match path {
                            PrincipalEntityPath::Module(_) => unreachable!(),
                            PrincipalEntityPath::MajorItem(path) => match path {
                                MajorItemPath::Type(path) => {
                                    HirEagerExprData::TypeConstructorFnCall {
                                        path,
                                        // ad hoc
                                        instantiation: HirInstantiation::new_empty(),
                                        item_groups,
                                    }
                                }
                                MajorItemPath::Trait(_) => unreachable!(),
                                MajorItemPath::Fugitive(path) => HirEagerExprData::FunctionFnCall {
                                    path,
                                    instantiation: HirInstantiation::from_fluffy(
                                        instantiation.as_ref().unwrap(),
                                        db,
                                        builder.fluffy_terms(),
                                    ),
                                    item_groups,
                                },
                            },
                            PrincipalEntityPath::TypeVariant(path) => {
                                HirEagerExprData::TypeVariantConstructorCall {
                                    path,
                                    // ad hoc
                                    instantiation: HirInstantiation::new_empty(),
                                    item_groups,
                                }
                            }
                        }
                    }
                    SemaExprData::AssociatedItem {
                        ref static_dispatch,
                        ..
                    } => {
                        match static_dispatch {
                            StaticDispatch::AssociatedFn(signature) => {
                                HirEagerExprData::AssociatedFunctionFnCall {
                                    path: signature.path(),
                                    // ad hoc
                                    instantiation: HirInstantiation::new_empty(),
                                    item_groups,
                                }
                            }
                            StaticDispatch::AssociatedGn => unreachable!(),
                        }
                    }
                    _ => todo!(),
                }
            }
            SemaExprData::Ritchie { .. } => todo!(),
            SemaExprData::Field {
                owner_sema_expr_idx,
                ident_token,
                ref dispatch,
                ..
            } => match *dispatch.signature() {
                FluffyFieldSignature::PropsStruct { ty } => HirEagerExprData::PropsStructField {
                    owner_hir_expr_idx: owner_sema_expr_idx.to_hir_eager(builder),
                    ident: ident_token.ident(),
                    field_ty: HirType::from_fluffy(ty, builder.db(), builder.fluffy_terms())
                        .unwrap(),
                },
                FluffyFieldSignature::Memoized {
                    ty,
                    path,
                    ref instantiation,
                } => HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx: owner_sema_expr_idx.to_hir_eager(builder),
                    ident: ident_token.ident(),
                    path,
                },
            },
            SemaExprData::MethodApplication { .. } => todo!(),
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                self_contract,
                ident_token,
                ref dispatch,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let MethodFluffySignature::MethodFn(signature) = dispatch.signature() else {
                    unreachable!()
                };
                HirEagerExprData::MethodFnCall {
                    self_argument: self_argument_sema_expr_idx.to_hir_eager(builder),
                    self_contract: HirEagerContract::from_term(self_contract),
                    ident: ident_token.ident(),
                    path: signature.path(),
                    instantiation: HirInstantiation::from_fluffy(
                        signature.instantiation(),
                        builder.db(),
                        builder.fluffy_terms(),
                    ),
                    item_groups: builder
                        .new_call_list_item_groups(ritchie_parameter_argument_matches),
                }
            }
            SemaExprData::MethodGnCall { .. } => {
                todo!()
            }
            SemaExprData::TemplateInstantiation { .. } => todo!(),
            SemaExprData::At { .. } => todo!(),
            SemaExprData::Unit { .. } => HirEagerExprData::Literal(TermLiteral::Unit),
            SemaExprData::Bracketed { item, .. } => return item.to_hir_eager(builder),
            SemaExprData::NewTuple { .. } => todo!(),
            SemaExprData::Index {
                owner_sema_expr_idx,
                lbox_regional_token_idx: _,
                ref index_sema_list_items,
                ..
            } => HirEagerExprData::Index {
                owner_hir_expr_idx: owner_sema_expr_idx.to_hir_eager(builder),
                items: index_sema_list_items
                    .iter()
                    .map(|item| item.sema_expr_idx.to_hir_eager(builder))
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
                    .map(|item| item.sema_expr_idx.to_hir_eager(builder))
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
                function_ident,
                ref arguments,
                ..
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
            SemaExprData::Sorry { .. } => todo!(),
            SemaExprData::Todo { .. } => HirEagerExprData::Todo,
            SemaExprData::Unreachable { .. } => HirEagerExprData::Unreachable,
            SemaExprData::VecFunctor { .. } => todo!(),
            SemaExprData::ArrayFunctor { .. } => todo!(),
        };
        let ty = self.ty(builder.sema_expr_arena_ref2());
        let ty_place = ty
            .place()
            .map(|place| HirPlace::from_fluffy(place))
            .unwrap_or(HirPlace::Transient);
        let entry = HirEagerExprEntry {
            data,
            ty_place,
            is_ty_always_copyable: ty
                .is_always_copyable(builder.db(), builder.fluffy_terms())
                .unwrap()
                .unwrap(),
        };
        builder.alloc_expr(*self, entry)
    }
}
