mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::{
    be_variable::HirEagerBeVariablesPattern, closure_parameter::HirEagerClosureParameterPattern,
    variable::runtime::HirEagerRvarIdx, *,
};
use husky_eth_term::term::EthTerm;
use husky_fly_term::{
    dispatch::StaticDispatch,
    signature::{FlyFieldSignature, MethodFlySignature},
};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    instantiation::HirInstantiation,
    place_contract_site::HirPlaceContractSite,
    quary::{HirContractedQuary, HirQuary},
    ritchie::HirContract,
    HirType,
};
use husky_sem_expr::{SemaExprData, SemaExprIdx, SemaRitchieArgument};
use husky_sem_opr::{binary::SemaBinaryOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::{InheritedTemplateVariable, InheritedVariableKind};
use husky_term_prelude::literal::Literal;
use vec_like::VecMap;

pub type HirEagerExprArena = Arena<HirEagerExprEntry>;
pub type HirEagerExprIdx = ArenaIdx<HirEagerExprEntry>;
pub type HirEagerExprIdxRange = ArenaIdxRange<HirEagerExprEntry>;
pub type HirEagerExprMap<V> = ArenaMap<HirEagerExprEntry, V>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerExprEntry {
    data: HirEagerExprData,
    contracted_quary: HirContractedQuary,
    is_always_copyable: bool,
    place_contract_site: HirPlaceContractSite,
}

/// # getters
impl HirEagerExprEntry {
    pub fn data(&self) -> &HirEagerExprData {
        &self.data
    }

    pub fn contracted_quary(&self) -> HirContractedQuary {
        self.contracted_quary
    }

    pub fn quary(&self) -> HirQuary {
        self.contracted_quary.quary()
    }

    pub fn is_always_copyable(&self) -> bool {
        self.is_always_copyable
    }

    pub fn place_contract_site(&self) -> &HirPlaceContractSite {
        &self.place_contract_site
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub enum HirEagerExprData {
    Literal(Literal),
    PrincipalEntityPath(PrincipalEntityPath),
    AssocRitchie {
        assoc_item_path: AssocItemPath,
    },
    ConstVariable {
        ident: Ident,
    },
    Variable(HirEagerRvarIdx),
    Binary {
        lopd: HirEagerExprIdx,
        opr: HirBinaryOpr,
        ropd: HirEagerExprIdx,
    },
    Be {
        src: HirEagerExprIdx,
        pattern: HirEagerBeVariablesPattern,
    },
    Prefix {
        opr: HirPrefixOpr,
        opd: HirEagerExprIdx,
    },
    Suffix {
        opd: HirEagerExprIdx,
        opr: HirSuffixOpr,
    },
    Unveil {
        unveil_assoc_fn_path: TraitForTypeItemPath,
        instantiation: HirInstantiation,
        return_ty: HirType,
        opd: HirEagerExprIdx,
    },
    Unwrap {
        opd: HirEagerExprIdx,
    },
    As {
        opd: HirEagerExprIdx,
        ty: HirType,
    },
    TypeConstructorCall {
        path: TypePath,
        instantiation: HirInstantiation,
        arguments: SmallVec<[HirEagerRitchieArgument; 4]>,
    },
    TypeVariantConstructorCall {
        path: TypeVariantPath,
        instantiation: HirInstantiation,
        arguments: SmallVec<[HirEagerRitchieArgument; 4]>,
    },
    FunctionRitchieCall {
        path: MajorFormPath,
        instantiation: HirInstantiation,
        arguments: SmallVec<[HirEagerRitchieArgument; 4]>,
    },
    AssocFunctionRitchieCall {
        path: AssocItemPath,
        instantiation: HirInstantiation,
        arguments: SmallVec<[HirEagerRitchieArgument; 4]>,
    },
    PropsStructField {
        self_argument: HirEagerExprIdx,
        self_ty: HirType,
        ident: Ident,
        field_ty: HirType,
    },
    MemoizedField {
        self_argument: HirEagerExprIdx,
        self_ty: HirType,
        ident: Ident,
        path: AssocItemPath,
        instantiation: HirInstantiation,
    },
    MethodRitchieCall {
        self_argument: HirEagerExprIdx,
        self_contract: HirContract,
        ident: Ident,
        path: AssocItemPath,
        instantiation: HirInstantiation,
        arguments: SmallVec<[HirEagerRitchieArgument; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    Index {
        owner: HirEagerExprIdx,
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    NewList {
        // todo: change it to HirEagerExprIdxRange
        exprs: SmallVec<[HirEagerExprIdx; 4]>,
        element_ty: HirType,
        // todo: disambiguate Vec, SmallVec, Array, etc.
    },
    Block {
        stmts: HirEagerStmtIdxRange,
    },
    Closure {
        parameters: SmallVec<[HirEagerClosureParameterPattern; 4]>,
        return_ty: Option<HirType>,
        body: HirEagerExprIdx,
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
        let place_contract_site =
            HirPlaceContractSite::from_sema(&builder.sem_place_contract_region()[*self]);
        let data = match *self.data(builder.sem_expr_arena_ref()) {
            SemaExprData::Literal(_, _) => {
                HirEagerExprData::Literal(match builder.expr_term(*self) {
                    EthTerm::Literal(lit) => lit,
                    _ => unreachable!(),
                })
            }
            SemaExprData::PrincipalEntityPath { path, .. } => {
                // ad hoc
                HirEagerExprData::PrincipalEntityPath(path)
            }
            SemaExprData::MajorItemPathAssocItem {
                ref static_dispatch,
                ..
            }
            | SemaExprData::AssocItem {
                ref static_dispatch,
                ..
            } => match static_dispatch {
                StaticDispatch::AssocRitchie(signature) => HirEagerExprData::AssocRitchie {
                    assoc_item_path: signature.path(),
                },
                StaticDispatch::AssocGn => unreachable!(),
                StaticDispatch::TypeAsTrait {
                    trai,
                    trai_item_path,
                } => todo!(),
            },
            SemaExprData::InheritedSynSymbol {
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
                ..
            } => match inherited_syn_symbol_kind {
                InheritedVariableKind::Template(symbol) => match symbol {
                    InheritedTemplateVariable::Lifetime { label: _ } => {
                        todo!()
                    }
                    InheritedTemplateVariable::Place { label: _ } => todo!(),
                    InheritedTemplateVariable::Type { ident: _ } => todo!(),
                    InheritedTemplateVariable::Constant { ident } => {
                        HirEagerExprData::ConstVariable { ident }
                    }
                },
                InheritedVariableKind::Parenate { .. }
                | InheritedVariableKind::SelfField { .. } => HirEagerExprData::Variable(
                    builder
                        .inherited_syn_symbol_to_hir_eager_runtime_symbol(inherited_syn_symbol_idx)
                        .unwrap(),
                ),
            },
            SemaExprData::CurrentSynSymbol {
                current_variable_idx,
                ..
            } => HirEagerExprData::Variable(
                builder
                    .current_variable_to_hir_eager_runtime_symbol(current_variable_idx)
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
                pattern: target.to_hir_eager(builder),
            },
            SemaExprData::Prefix {
                opr,
                opd: opd_sem_expr_idx,
                ..
            } => HirEagerExprData::Prefix {
                opr: HirPrefixOpr::from_sema(
                    opr,
                    builder.expr_ty(opd_sem_expr_idx),
                    builder.db(),
                    builder.fly_terms(),
                ),
                opd: opd_sem_expr_idx.to_hir_eager(builder),
            },
            SemaExprData::Suffix {
                opd: opd_sem_expr_idx,
                opr,
                ..
            } => match opr {
                SemaSuffixOpr::ComposeWithOption => unreachable!(),
                SemaSuffixOpr::ComposeWithNot => unreachable!(),
                SemaSuffixOpr::Incr | SemaSuffixOpr::Decr => HirEagerExprData::Suffix {
                    opr: HirSuffixOpr::from_sema(opr),
                    opd: opd_sem_expr_idx.to_hir_eager(builder),
                },
            },
            SemaExprData::Unveil {
                return_ty,
                ref unveil_output_ty_signature,
                unveil_assoc_fn_path,
                opd_sem_expr_idx,
                ..
            } => {
                let db = builder.db();
                HirEagerExprData::Unveil {
                    unveil_assoc_fn_path,
                    instantiation: HirInstantiation::from_eth(
                        unveil_output_ty_signature.instantiation(),
                        db,
                    ),
                    opd: opd_sem_expr_idx.to_hir_eager(builder),
                    return_ty: HirType::from_eth(return_ty, db).unwrap(),
                }
            }
            SemaExprData::Unwrap {
                opd_sem_expr_idx, ..
            } => HirEagerExprData::Unwrap {
                opd: opd_sem_expr_idx.to_hir_eager(builder),
            },
            SemaExprData::FunctionApplication { .. } => {
                use husky_print_utils::p;
                p!(builder.path());
                unreachable!()
            }
            SemaExprData::FunctionRitchieCall {
                function_sem_expr_idx,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let db = builder.db();
                let _template_arguments = template_arguments.as_ref().map(|_| todo!());
                let item_groups =
                    builder.new_call_list_arguments(ritchie_parameter_argument_matches);
                match *builder.sem_expr_arena_ref()[function_sem_expr_idx].data() {
                    SemaExprData::PrincipalEntityPath {
                        path,
                        ref instantiation,
                        ..
                    } => match path {
                        PrincipalEntityPath::Module(_) => unreachable!(),
                        PrincipalEntityPath::MajorItem(path) => match path {
                            MajorItemPath::Type(path) => HirEagerExprData::TypeConstructorCall {
                                path,
                                instantiation: HirInstantiation::from_fly(
                                    instantiation.as_ref().unwrap(),
                                    &place_contract_site,
                                    db,
                                    builder.fly_terms(),
                                ),
                                arguments: item_groups,
                            },
                            MajorItemPath::Trait(_) => unreachable!(),
                            MajorItemPath::Form(path) => HirEagerExprData::FunctionRitchieCall {
                                path,
                                instantiation: HirInstantiation::from_fly(
                                    instantiation.as_ref().unwrap(),
                                    &place_contract_site,
                                    db,
                                    builder.fly_terms(),
                                ),
                                arguments: item_groups,
                            },
                        },
                        PrincipalEntityPath::TypeVariant(path) => {
                            HirEagerExprData::TypeVariantConstructorCall {
                                path,
                                instantiation: HirInstantiation::from_fly(
                                    instantiation.as_ref().unwrap(),
                                    &place_contract_site,
                                    db,
                                    builder.fly_terms(),
                                ),
                                arguments: item_groups,
                            }
                        }
                    },
                    SemaExprData::MajorItemPathAssocItem {
                        ref static_dispatch,
                        ..
                    } => match static_dispatch {
                        StaticDispatch::AssocRitchie(signature) => {
                            HirEagerExprData::AssocFunctionRitchieCall {
                                path: signature.path(),
                                instantiation: HirInstantiation::from_fly(
                                    signature.instantiation(),
                                    &place_contract_site,
                                    db,
                                    builder.fly_terms(),
                                ),
                                arguments: item_groups,
                            }
                        }
                        StaticDispatch::AssocGn => unreachable!(),
                        StaticDispatch::TypeAsTrait {
                            trai,
                            trai_item_path,
                        } => todo!(),
                    },
                    _ => todo!(),
                }
            }
            SemaExprData::Ritchie { .. } => todo!(),
            SemaExprData::Field {
                self_argument,
                self_ty,
                ident_token,
                ref dispatch,
                ..
            } => match *dispatch.signature() {
                FlyFieldSignature::PropsStruct { ty } => HirEagerExprData::PropsStructField {
                    self_argument: self_argument.to_hir_eager(builder),
                    self_ty: HirType::from_fly(self_ty, builder.db(), builder.fly_terms()).unwrap(),
                    ident: ident_token.ident(),
                    field_ty: HirType::from_fly(ty, builder.db(), builder.fly_terms()).unwrap(),
                },
                FlyFieldSignature::Memoized {
                    ty: _,
                    path,
                    ref instantiation,
                } => {
                    debug_assert!(instantiation.separator().is_some());
                    HirEagerExprData::MemoizedField {
                        self_argument: self_argument.to_hir_eager(builder),
                        self_ty: HirType::from_fly(self_ty, builder.db(), builder.fly_terms())
                            .unwrap(),
                        ident: ident_token.ident(),
                        path,
                        instantiation: HirInstantiation::from_fly(
                            instantiation,
                            &place_contract_site,
                            builder.db(),
                            builder.fly_terms(),
                        ),
                    }
                }
            },
            SemaExprData::MethodApplication { .. } => todo!(),
            SemaExprData::MethodFnCall {
                self_argument_sem_expr_idx,
                self_contract,
                ident_token,
                ref dispatch,

                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let MethodFlySignature::MethodFn(signature) = dispatch.signature() else {
                    unreachable!()
                };
                HirEagerExprData::MethodRitchieCall {
                    self_argument: self_argument_sem_expr_idx.to_hir_eager(builder),
                    self_contract: HirContract::from_contract(self_contract),
                    ident: ident_token.ident(),
                    path: signature.path(),
                    instantiation: HirInstantiation::from_fly(
                        signature.instantiation(),
                        &place_contract_site,
                        builder.db(),
                        builder.fly_terms(),
                    ),
                    arguments: builder.new_call_list_arguments(ritchie_parameter_argument_matches),
                }
            }
            SemaExprData::MethodGnCall { .. } => {
                todo!()
            }
            SemaExprData::TemplateInstantiation { .. } => todo!(),
            SemaExprData::At { .. } => todo!(),
            SemaExprData::Unit { .. } => HirEagerExprData::Literal(Literal::Unit(())),
            SemaExprData::Delimitered { item, .. } => return item.to_hir_eager(builder),
            SemaExprData::NewTuple { .. } => todo!(),
            SemaExprData::Index {
                owner: owner_sem_expr_idx,
                lbox_regional_token_idx: _,
                ref index_sem_list_items,
                ..
            } => HirEagerExprData::Index {
                owner: owner_sem_expr_idx.to_hir_eager(builder),
                items: index_sem_list_items
                    .iter()
                    .map(|item| item.sem_expr_idx.to_hir_eager(builder))
                    .collect(),
            },
            SemaExprData::CompositionWithList { .. } => {
                todo!()
            }
            SemaExprData::NewList {
                ref items,
                element_ty,
                ..
            } => HirEagerExprData::NewList {
                exprs: items
                    .iter()
                    .map(|item| item.sem_expr_idx.to_hir_eager(builder))
                    .collect(),
                element_ty: HirType::from_fly(element_ty, builder.db(), builder.fly_terms())
                    .unwrap(),
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
            SemaExprData::NestedBlock { stmts, .. } => HirEagerExprData::Block {
                stmts: stmts.to_hir_eager(builder),
            },
            SemaExprData::Closure {
                ref parameter_obelisks,
                return_ty,
                body,
                ..
            } => HirEagerExprData::Closure {
                parameters: parameter_obelisks
                    .iter()
                    .map(|param| param.to_hir_eager(builder))
                    .collect(),
                return_ty: return_ty
                    .map(|(_, return_ty, _)| builder.expr_term_hir_ty(return_ty).unwrap()),
                body: body.to_hir_eager(builder),
            },
        };
        let ty = self.ty(builder.sem_expr_arena_ref2());
        let contracted_quary = ty
            .quary()
            .map(|quary| HirContractedQuary::from_fly(quary, &place_contract_site))
            .unwrap_or_default();
        let entry = HirEagerExprEntry {
            data,
            contracted_quary,
            is_always_copyable: ty
                .is_always_copyable(builder.db(), builder.fly_terms())
                .unwrap()
                .unwrap(),
            place_contract_site,
        };
        builder.alloc_expr(*self, entry)
    }
}
