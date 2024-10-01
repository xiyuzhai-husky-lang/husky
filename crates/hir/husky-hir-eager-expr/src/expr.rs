mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::{
    be_variable::HirEagerBeVariablesPattern, closure_parameter::HirEagerClosureParameterPattern,
    variable::runtime::HirEagerRuntimeVariableIdx, *,
};
use coercion::{DedirectionHirEagerCoercion, HirEagerCoercion};
use husky_entity_path::path::{
    assoc_item::{trai_for_ty_item::TraitForTypeItemPath, AssocItemPath},
    major_item::{form::MajorFormPath, ty::TypePath, MajorItemPath},
    ty_variant::TypeVariantPath,
    PrincipalEntityPath,
};
use husky_eth_term::term::EthTerm;
use husky_fly_term::{
    dispatch::{field::FieldFlySignature, method::MethodFlySignature, OntologyDispatch},
    quary::FlyQuary,
    signature::assoc_item::ty_item::TypeItemFlySignature,
    ExpectationOutcome,
};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    indirections::HirIndirections,
    instantiation::HirInstantiation,
    place_contract_site::HirPlaceContractSite,
    quary::{HirContractedQuary, HirQuary},
    ritchie::HirContract,
    HirType,
};
use husky_place::place::EthPlace;
use husky_sem_expr::{SemExprData, SemExprIdx, SemRitchieArgument};
use husky_sem_opr::{binary::SemBinaryOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::variable::{InheritedTemplateVariable, InheritedVariableKind};
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
    /// this is the type before coercion
    base_ty: HirType,
    /// note that the contract is only told when the quary has a place, i.e., not transient
    contracted_quary: HirContractedQuary,
    always_copyable: bool,
    place_contract_site: HirPlaceContractSite,
    /// None means it's not entirely known from expectation alone,
    /// todo: remove Option
    coercion: Option<HirEagerCoercion>,
    /// note that the contract is only told when the quary has a place, i.e., not transient
    contracted_quary_after_coercion: HirContractedQuary,
}

/// # getters
impl HirEagerExprEntry {
    pub fn data(&self) -> &HirEagerExprData {
        &self.data
    }

    pub fn base_ty(&self) -> HirType {
        self.base_ty
    }

    pub fn contracted_quary(&self) -> HirContractedQuary {
        self.contracted_quary
    }

    pub fn quary(&self) -> HirQuary {
        self.contracted_quary.quary()
    }

    pub fn coercion(&self) -> Option<HirEagerCoercion> {
        self.coercion
    }

    /// this is before coercion happens, the inner type of the expression
    pub fn is_base_ty_always_copyable(&self) -> bool {
        self.always_copyable
    }

    pub fn place_contract_site(&self) -> &HirPlaceContractSite {
        &self.place_contract_site
    }

    pub fn contracted_quary_after_coercion(&self) -> HirContractedQuary {
        self.contracted_quary_after_coercion
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub enum HirEagerExprData {
    Literal(Literal),
    PrincipalEntityPath {
        path: PrincipalEntityPath,
        instantiation: HirInstantiation,
    },
    AssocRitchie {
        assoc_item_path: AssocItemPath,
    },
    ComptimeVariable {
        ident: Ident,
    },
    RuntimeVariable(HirEagerRuntimeVariableIdx),
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
        self_argument_ty: HirType,
        self_ty: HirType,
        ident: Ident,
        field_ty: HirType,
        indirections: HirIndirections,
    },
    MemoizedField {
        self_argument: HirEagerExprIdx,
        self_argument_ty: HirType,
        self_ty: HirType,
        ident: Ident,
        path: AssocItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
    },
    MethodRitchieCall {
        self_argument: HirEagerExprIdx,
        self_ty: HirType,
        self_contract: HirContract,
        ident: Ident,
        path: AssocItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
        arguments: SmallVec<[HirEagerRitchieArgument; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirEagerExprIdx; 4]>,
    },
    Index {
        self_argument: HirEagerExprIdx,
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

impl ToHirEager for SemExprIdx {
    type Output = HirEagerExprIdx;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let place_contract_site =
            HirPlaceContractSite::from_sem(&builder.sem_place_contract_region()[*self]);
        let data = match *self.data(builder.sem_expr_arena_ref()) {
            SemExprData::Literal(_, _) => {
                HirEagerExprData::Literal(match builder.expr_term(*self) {
                    EthTerm::Literal(lit) => lit,
                    _ => unreachable!(),
                })
            }
            SemExprData::PrincipalEntityPath {
                path,
                ref instantiation,
                ..
            } => HirEagerExprData::PrincipalEntityPath {
                path,
                instantiation: HirInstantiation::from_fly(
                    instantiation.as_ref().unwrap(),
                    &place_contract_site,
                    builder.db(),
                    builder.terms(),
                ),
            },
            SemExprData::MajorItemPathAssocItem {
                ref ontology_dispatch,
                ..
            }
            | SemExprData::TypeAsTraitItem {
                ref ontology_dispatch,
                ..
            }
            | SemExprData::AssocItem {
                ref ontology_dispatch,
                ..
            } => match ontology_dispatch {
                OntologyDispatch::TypeItem { signature } => todo!(),
                OntologyDispatch::TraitItem { signature, .. } => todo!(),
                OntologyDispatch::TraitForTypeItem { signature } => todo!(),
                // StaticDispatch::AssocRitchie(signature) => HirEagerExprData::AssocRitchie {
                //     assoc_item_path: signature.path(),
                // },
                // StaticDispatch::AssocGn => unreachable!(),
                // StaticDispatch::TypeAsTrait {
                //     trai,
                //     trai_item_path,
                //     ..
                // } => todo!(),
            },
            SemExprData::InheritedVariable {
                inherited_variable_idx,
                inherited_variable_kind,
                ..
            } => match inherited_variable_kind {
                InheritedVariableKind::Template(symbol) => match symbol {
                    InheritedTemplateVariable::Lifetime { label: _ } => {
                        todo!()
                    }
                    InheritedTemplateVariable::Place { label: _ } => todo!(),
                    InheritedTemplateVariable::Type { ident: _ } => todo!(),
                    InheritedTemplateVariable::Constant { ident } => {
                        HirEagerExprData::ComptimeVariable { ident }
                    }
                },
                InheritedVariableKind::Parenate { .. }
                | InheritedVariableKind::SelfField { .. } => HirEagerExprData::RuntimeVariable(
                    builder
                        .inherited_variable_to_hir_eager_runtime_variable(inherited_variable_idx)
                        .unwrap(),
                ),
                InheritedVariableKind::ReplLocal => todo!(),
            },
            SemExprData::CurrentVariable {
                current_variable_idx,
                ..
            } => HirEagerExprData::RuntimeVariable(
                builder
                    .current_variable_to_hir_eager_runtime_variable(current_variable_idx)
                    .unwrap(),
            ),
            SemExprData::FrameVarDecl { .. } => todo!(),
            SemExprData::SelfType(_) => {
                unreachable!()
            }
            SemExprData::SelfValue(_) => {
                HirEagerExprData::RuntimeVariable(builder.self_value_variable().unwrap())
            }
            SemExprData::Binary {
                lopd, opr, ropd, ..
            } => match opr {
                SemBinaryOpr::As => HirEagerExprData::As {
                    opd: lopd.to_hir_eager(builder),
                    ty: builder.expr_term_hir_ty(ropd).unwrap(),
                },
                _ => HirEagerExprData::Binary {
                    lopd: lopd.to_hir_eager(builder),
                    opr: HirBinaryOpr::from_sema(opr),
                    ropd: ropd.to_hir_eager(builder),
                },
            },
            SemExprData::Be {
                src, ref target, ..
            } => HirEagerExprData::Be {
                src: src.to_hir_eager(builder),
                pattern: target.to_hir_eager(builder),
            },
            SemExprData::Prefix {
                opr,
                opd: opd_sem_expr_idx,
                ..
            } => HirEagerExprData::Prefix {
                opr: HirPrefixOpr::from_sema(
                    opr,
                    builder.expr_ty(opd_sem_expr_idx),
                    builder.db(),
                    builder.terms(),
                ),
                opd: opd_sem_expr_idx.to_hir_eager(builder),
            },
            SemExprData::Suffix {
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
            SemExprData::Unveil {
                return_ty,
                ref unveil_output_ty_signature,
                ref unveil_assoc_fn_signature,
                unveil_assoc_fn_path,
                opd,
                ..
            } => {
                let db = builder.db();
                HirEagerExprData::Unveil {
                    unveil_assoc_fn_path,
                    instantiation: HirInstantiation::from_eth(
                        unveil_assoc_fn_signature.instantiation(),
                        db,
                    ),
                    opd: opd.to_hir_eager(builder),
                    return_ty: HirType::from_eth(return_ty, db).unwrap(),
                }
            }
            SemExprData::Unwrap {
                opd: opd_sem_expr_idx,
                ..
            } => HirEagerExprData::Unwrap {
                opd: opd_sem_expr_idx.to_hir_eager(builder),
            },
            SemExprData::FunctionApplication { .. } => {
                use husky_print_utils::p;
                p!(builder.path());
                unreachable!()
            }
            SemExprData::FunctionRitchieCall {
                function: function_sem_expr_idx,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let db = builder.db();
                let _template_arguments = template_arguments.as_ref().map(|_| todo!());
                let item_groups =
                    builder.new_call_list_arguments(ritchie_parameter_argument_matches);
                match *builder.sem_expr_arena_ref()[function_sem_expr_idx].data() {
                    SemExprData::PrincipalEntityPath {
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
                                    builder.terms(),
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
                                    builder.terms(),
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
                                    builder.terms(),
                                ),
                                arguments: item_groups,
                            }
                        }
                    },
                    SemExprData::MajorItemPathAssocItem {
                        ref ontology_dispatch,
                        ..
                    } => match ontology_dispatch {
                        OntologyDispatch::TypeItem { signature } => match signature {
                            TypeItemFlySignature::AssocRitchie(signature) => {
                                HirEagerExprData::AssocFunctionRitchieCall {
                                    path: signature.path().into(),
                                    instantiation: HirInstantiation::from_fly(
                                        signature.instantiation(),
                                        &place_contract_site,
                                        db,
                                        builder.terms(),
                                    ),
                                    arguments: item_groups,
                                }
                            }
                        },
                        OntologyDispatch::TraitItem { signature, .. } => todo!(),
                        OntologyDispatch::TraitForTypeItem { signature } => todo!(),
                        // StaticDispatch::AssocRitchie(signature) => {
                        // }
                        // StaticDispatch::AssocGn => unreachable!(),
                        // StaticDispatch::TypeAsTrait {
                        //     trai,
                        //     trai_item_path,
                        //     ..
                        // } => todo!(),
                    },
                    _ => todo!(),
                }
            }
            SemExprData::Ritchie { .. } => todo!(),
            SemExprData::Field {
                self_argument,
                self_argument_ty,
                ident_token,
                ref dispatch,
                ..
            } => match *dispatch.signature() {
                FieldFlySignature::PropsStruct { self_ty, ty } => {
                    HirEagerExprData::PropsStructField {
                        self_argument: self_argument.to_hir_eager(builder),
                        self_argument_ty: HirType::from_fly_base(
                            self_argument_ty,
                            builder.db(),
                            builder.terms(),
                        )
                        .unwrap(),
                        self_ty: HirType::from_fly_base(self_ty, builder.db(), builder.terms())
                            .unwrap(),
                        ident: ident_token.ident(),
                        field_ty: HirType::from_fly_base(ty, builder.db(), builder.terms())
                            .unwrap(),
                        indirections: HirIndirections::from_fly(dispatch.indirections()),
                    }
                }
                FieldFlySignature::Memoized {
                    path,
                    self_ty,
                    ref instantiation,
                    ..
                } => {
                    debug_assert!(instantiation.separator().is_some());
                    let db = builder.db();
                    let terms = builder.terms();
                    HirEagerExprData::MemoizedField {
                        self_ty: HirType::from_fly(self_ty, db, terms).unwrap(),
                        self_argument: self_argument.to_hir_eager(builder),
                        self_argument_ty: HirType::from_fly_base(
                            self_argument_ty,
                            builder.db(),
                            terms,
                        )
                        .unwrap(),
                        ident: ident_token.ident(),
                        path,
                        indirections: HirIndirections::from_fly(dispatch.indirections()),
                        instantiation: HirInstantiation::from_fly(
                            instantiation,
                            &place_contract_site,
                            builder.db(),
                            builder.terms(),
                        ),
                    }
                }
            },
            SemExprData::MethodApplication { .. } => todo!(),
            SemExprData::MethodRitchieCall {
                self_argument,
                self_ty,
                self_contract,
                ident_token,
                ref dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let signature = dispatch.signature();
                HirEagerExprData::MethodRitchieCall {
                    self_argument: self_argument.to_hir_eager(builder),
                    self_ty: HirType::from_fly(
                        self_ty,
                        builder.db(),
                        builder.sem_expr_region_data.fly_term_region().terms(),
                    )
                    .unwrap(),
                    self_contract: HirContract::from_contract(self_contract),
                    ident: ident_token.ident(),
                    path: signature.path(),
                    indirections: HirIndirections::from_fly(dispatch.indirections()),
                    instantiation: HirInstantiation::from_fly(
                        signature.instantiation(),
                        &place_contract_site,
                        builder.db(),
                        builder.terms(),
                    ),
                    arguments: builder.new_call_list_arguments(ritchie_parameter_argument_matches),
                }
            }
            SemExprData::TemplateInstantiation { .. } => todo!(),
            SemExprData::At { .. } => todo!(),
            SemExprData::Unit { .. } => HirEagerExprData::Literal(Literal::Unit(())),
            SemExprData::Delimitered { item, .. } => return item.to_hir_eager(builder),
            SemExprData::NewTuple { .. } => todo!(),
            SemExprData::Index {
                self_argument,
                ref items,
                ..
            } => HirEagerExprData::Index {
                self_argument: self_argument.to_hir_eager(builder),
                items: items
                    .iter()
                    .map(|item| item.expr.to_hir_eager(builder))
                    .collect(),
            },
            SemExprData::CompositionWithList { .. } => {
                todo!()
            }
            SemExprData::NewList {
                ref items,
                element_ty,
                ..
            } => HirEagerExprData::NewList {
                exprs: items
                    .iter()
                    .map(|item| item.expr.to_hir_eager(builder))
                    .collect(),
                element_ty: HirType::from_fly_base(element_ty, builder.db(), builder.terms())
                    .unwrap(),
            },
            SemExprData::BoxColonList {
                lbox_regional_token_idx: _,
                colon_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemExprData::Block { stmts } => HirEagerExprData::Block {
                stmts: stmts.to_hir_eager(builder),
            },
            SemExprData::EmptyHtmxTag {
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
            SemExprData::Sorry { .. } => todo!(),
            SemExprData::Todo { .. } => HirEagerExprData::Todo,
            SemExprData::Unreachable { .. } => HirEagerExprData::Unreachable,
            SemExprData::VecFunctor { .. } => todo!(),
            SemExprData::ArrayFunctor { .. } => todo!(),
            SemExprData::NestedBlock { stmts, .. } => HirEagerExprData::Block {
                stmts: stmts.to_hir_eager(builder),
            },
            SemExprData::Closure {
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
        let base_ty = HirType::from_fly_base(
            ty,
            builder.db(),
            builder.sem_expr_region_data.fly_term_region().terms(),
        )
        .unwrap();
        let quary = ty.quary();
        let contracted_quary = quary
            .map(|quary| HirContractedQuary::from_fly(quary, &place_contract_site))
            .unwrap_or(HirContractedQuary::new_contractless_transient());
        let coercion = match self.expectation_outcome(builder.sem_expr_region_data) {
            Some(ref outcome) => match outcome {
                ExpectationOutcome::Coercion(coersion_outcome) => {
                    Some(coersion_outcome.coercion().to_hir_eager(builder))
                }
                _ => None,
            },
            None => None,
        };
        let quary_after_coercion = match coercion {
            Some(coercion) => match coercion {
                HirEagerCoercion::Trivial(_) => quary,
                HirEagerCoercion::Never => None,
                HirEagerCoercion::WrapInSome => Some(FlyQuary::Transient),
                HirEagerCoercion::Redirection(_) => Some(FlyQuary::Transient),
                HirEagerCoercion::Dedirection(dedirection) => match dedirection {
                    DedirectionHirEagerCoercion::Deleash => {
                        let place = match quary {
                            Some(quary) => match quary {
                                FlyQuary::Compterm => todo!(),
                                FlyQuary::StackPure { place } => match place {
                                    EthPlace::Idx(place_idx) => Some(place_idx),
                                    EthPlace::SymbolicVariable(_) => todo!(),
                                    EthPlace::Field(_) => todo!(),
                                },
                                FlyQuary::ImmutableOnStack { place } => todo!(),
                                FlyQuary::MutableOnStack { place } => todo!(),
                                FlyQuary::Transient => None,
                                FlyQuary::Ref { guard } => todo!(),
                                FlyQuary::RefMut { place, lifetime } => todo!(),
                                FlyQuary::Leashed { place } => todo!(),
                                FlyQuary::Todo => todo!(),
                                FlyQuary::EtherealSymbol(_) => todo!(),
                            },
                            None => None,
                        };
                        Some(FlyQuary::Leashed { place })
                    }
                    DedirectionHirEagerCoercion::Deref { lifetime } => todo!(),
                    DedirectionHirEagerCoercion::DerefMut => todo!(),
                },
            },
            None => quary,
        };
        let contracted_quary_after_coercion = quary_after_coercion
            .map(|quary_after_coercion| {
                HirContractedQuary::from_fly(quary_after_coercion, &place_contract_site)
            })
            .unwrap_or(HirContractedQuary::new_contractless_transient());
        let entry = HirEagerExprEntry {
            data,
            base_ty,
            contracted_quary,
            always_copyable: ty
                .always_copyable(builder.db(), builder.terms())
                .unwrap()
                .unwrap(),
            place_contract_site,
            coercion,
            contracted_quary_after_coercion,
        };
        builder.alloc_expr(*self, entry)
    }
}
