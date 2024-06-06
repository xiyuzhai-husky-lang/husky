mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{
    assoc_item::{trai_for_ty_item::TraitForTypeItemPath, AssocItemPath},
    major_item::{form::MajorFormPath, ty::TypePath, MajorItemPath},
    ty_variant::TypeVariantPath,
    PrincipalEntityPath,
};
use husky_fly_term::signature::{FlyFieldSignature, MethodFlySignature};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    indirections::HirIndirections, instantiation::HirInstantiation,
    place_contract_site::HirPlaceContractSite, HirComptermTemplateVariable, HirType,
};
use husky_sem_expr::{SemExprData, SemExprIdx};
use husky_sem_opr::binary::SemaBinaryOpr;
use husky_term_prelude::literal::Literal;
use idx_arena::ArenaRef;

pub type HirLazyExprArena = Arena<HirLazyExprData>;
pub type HirLazyExprArenaRef<'a> = ArenaRef<'a, HirLazyExprData>;
pub type HirLazyExprIdx = ArenaIdx<HirLazyExprData>;
pub type HirLazyExprIdxRange = ArenaIdxRange<HirLazyExprData>;
pub type HirLazyExprMap<V> = ArenaMap<HirLazyExprData, V>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub enum HirLazyExprData {
    Literal(Literal),
    PrincipalEntityPath(PrincipalEntityPath),
    ConstSymbol(HirComptermTemplateVariable),
    Variable(HirLazyVariableIdx),
    Binary {
        lopd: HirLazyExprIdx,
        opr: HirBinaryOpr,
        ropd: HirLazyExprIdx,
    },
    Be {
        src: HirLazyExprIdx,
        pattern: HirLazyBeVariablesPattern,
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
    TypeConstructorCall {
        path: TypePath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListArgument; 4]>,
    },
    TypeVariantConstructorCall {
        path: TypeVariantPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListArgument; 4]>,
    },
    FunctionRitchieItemCall {
        path: MajorFormPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListArgument; 4]>,
    },
    AssocFunctionRitchieCall {
        path: AssocItemPath,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListArgument; 4]>,
    },
    PropsStructField {
        owner: HirLazyExprIdx,
        owner_base_ty: HirType,
        ident: Ident,
    },
    MemoizedField {
        owner: HirLazyExprIdx,
        ident: Ident,
        path: AssocItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
    },
    MethodRitchieCall {
        self_argument: HirLazyExprIdx,
        ident: Ident,
        path: AssocItemPath,
        indirections: HirIndirections,
        instantiation: HirInstantiation,
        item_groups: SmallVec<[HirLazyCallListArgument; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    Index {
        owner: HirLazyExprIdx,
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    ConstructList {
        items: SmallVec<[HirLazyExprIdx; 4]>,
        element_ty: HirType,
        // todo: disambiguate Vec, SmallVec, Array, etc.
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
    AssocRitchie {
        // ad hoc, needs more
        path: AssocItemPath,
    },
    Unveil {
        unveil_assoc_fn_path: TraitForTypeItemPath,
        instantiation: HirInstantiation,
        opd_hir_expr_idx: HirLazyExprIdx,
    },
    Unwrap {
        opd_hir_expr_idx: HirLazyExprIdx,
    },
}

impl ToHirLazy for SemExprIdx {
    type Output = HirLazyExprIdx;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let place_contract_site =
            HirPlaceContractSite::from_sema(&builder.sem_place_contract_region()[*self]);
        let hir_lazy_expr = match *self.data(builder.sem_expr_arena_ref()) {
            SemExprData::Literal(_, _) => {
                let EthTerm::Literal(lit) = builder.expr_term(*self) else {
                    unreachable!()
                };
                HirLazyExprData::Literal(lit)
            }
            SemExprData::PrincipalEntityPath { path, .. } => {
                HirLazyExprData::PrincipalEntityPath(path)
            }
            SemExprData::MajorItemPathAssocItem { .. } => todo!(),
            SemExprData::AssocItem { .. } => todo!(),
            SemExprData::InheritedSynSymbol {
                inherited_syn_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .inherited_syn_symbol_to_hir_lazy_variable(inherited_syn_symbol_idx)
                    .unwrap(),
            ),
            SemExprData::CurrentSynSymbol {
                current_variable_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .current_variable_to_hir_lazy_variable(current_variable_idx)
                    .unwrap(),
            ),
            SemExprData::FrameVarDecl {
                frame_var_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .current_variable_to_hir_lazy_variable(frame_var_symbol_idx)
                    .unwrap(),
            ),
            SemExprData::SelfType(_) => todo!(),
            SemExprData::SelfValue(_) => todo!(),
            SemExprData::Binary {
                lopd, opr, ropd, ..
            } => match opr {
                SemaBinaryOpr::As => HirLazyExprData::As {
                    opd: lopd.to_hir_lazy(builder),
                    ty: builder.expr_term_to_hir_ty(ropd).expect("valid"),
                },
                _ => HirLazyExprData::Binary {
                    lopd: lopd.to_hir_lazy(builder),
                    opr: HirBinaryOpr::from_sema(opr),
                    ropd: ropd.to_hir_lazy(builder),
                },
            },
            SemExprData::Be {
                src,
                be_regional_token_idx: _,
                ref target,
            } => HirLazyExprData::Be {
                src: src.to_hir_lazy(builder),
                pattern: target.to_hir_lazy(builder),
            },
            SemExprData::Prefix {
                opr,
                opd: opd_sem_expr_idx,
                ..
            } => HirLazyExprData::Prefix {
                opr: HirPrefixOpr::from_sema(
                    opr,
                    builder.expr_ty(opd_sem_expr_idx),
                    builder.db(),
                    builder.fly_terms(),
                ),
                opd_hir_expr_idx: opd_sem_expr_idx.to_hir_lazy(builder),
            },
            SemExprData::Suffix {
                opd: opd_sem_expr_idx,
                opr,
                ..
            } => match opr {
                _ => HirLazyExprData::Suffix {
                    opr: HirSuffixOpr::from_sema(opr),
                    opd_hir_expr_idx: opd_sem_expr_idx.to_hir_lazy(builder),
                },
            },
            SemExprData::Unveil {
                ref unveil_output_ty_signature,
                unveil_assoc_fn_path,
                opd_sem_expr_idx,
                ..
            } => HirLazyExprData::Unveil {
                unveil_assoc_fn_path,
                instantiation: HirInstantiation::from_eth(
                    unveil_output_ty_signature.instantiation(),
                    builder.db(),
                ),
                opd_hir_expr_idx: opd_sem_expr_idx.to_hir_lazy(builder),
            },
            SemExprData::Unwrap {
                opd_sem_expr_idx, ..
            } => HirLazyExprData::Unwrap {
                opd_hir_expr_idx: opd_sem_expr_idx.to_hir_lazy(builder),
            },
            SemExprData::FunctionApplication {
                function_sem_expr_idx: _,
                argument_sem_expr_idx: _,
            } => {
                todo!()
            }
            SemExprData::FunctionRitchieCall {
                function_sem_expr_idx,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let _function_hir_lazy_expr_idx = function_sem_expr_idx.to_hir_lazy(builder);
                let item_groups =
                    builder.new_call_list_item_groups(ritchie_parameter_argument_matches);
                match *builder.sem_expr_arena_ref()[function_sem_expr_idx].data() {
                    SemExprData::PrincipalEntityPath {
                        path,
                        // only None if `path` is an ontology constructor
                        instantiation: Some(ref instantiation),
                        ..
                    } => {
                        let instantiation = HirInstantiation::from_fly(
                            instantiation,
                            &place_contract_site,
                            builder.db(),
                            builder.fly_terms(),
                        );
                        match path {
                            PrincipalEntityPath::Module(_) => unreachable!(),
                            PrincipalEntityPath::MajorItem(path) => match path {
                                MajorItemPath::Type(path) => HirLazyExprData::TypeConstructorCall {
                                    path,
                                    instantiation,
                                    item_groups,
                                },
                                MajorItemPath::Trait(_) => unreachable!(),
                                MajorItemPath::Form(path) => match path.kind(builder.db()) {
                                    MajorFormKind::FN => HirLazyExprData::FunctionRitchieItemCall {
                                        path,
                                        instantiation,
                                        item_groups,
                                    },
                                    MajorFormKind::GN => HirLazyExprData::FunctionRitchieItemCall {
                                        path,
                                        instantiation,
                                        item_groups,
                                    },
                                    MajorFormKind::Ritchie(_) => todo!(),
                                    MajorFormKind::TypeAlias
                                    | MajorFormKind::TypeVar
                                    | MajorFormKind::Val
                                    | MajorFormKind::Conceptual => unreachable!(),
                                    MajorFormKind::Static => todo!(),
                                    MajorFormKind::Compterm => todo!(),
                                },
                            },
                            PrincipalEntityPath::TypeVariant(path) => {
                                HirLazyExprData::TypeVariantConstructorCall {
                                    path,
                                    instantiation,
                                    item_groups,
                                }
                            }
                        }
                    }
                    SemExprData::MajorItemPathAssocItem { .. } => todo!(),
                    _ => todo!(),
                }
            }
            SemExprData::Ritchie {
                ritchie_kind_regional_token_idx: _,
                ritchie_kind: _,
                lpar_token: _,
                parameter_ty_items: _,
                rpar_regional_token_idx: _,
                light_arrow_token: _,
                return_ty_sem_expr_idx: _,
            } => todo!(),
            SemExprData::Field {
                self_argument: owner,
                self_ty: owner_ty,
                ident_token,
                ref dispatch,
                ..
            } => match *dispatch.signature() {
                FlyFieldSignature::PropsStruct { ty: _ } => HirLazyExprData::PropsStructField {
                    owner: owner.to_hir_lazy(builder),
                    owner_base_ty: HirType::from_fly(owner_ty, builder.db(), builder.fly_terms())
                        .unwrap(),
                    ident: ident_token.ident(),
                },
                FlyFieldSignature::Memoized {
                    ty: _,
                    path,
                    ref instantiation,
                } => {
                    debug_assert!(instantiation.separator().is_some());
                    HirLazyExprData::MemoizedField {
                        owner: owner.to_hir_lazy(builder),
                        ident: ident_token.ident(),
                        path,
                        indirections: HirIndirections::from_fly(dispatch.indirections()),
                        instantiation: HirInstantiation::from_fly(
                            instantiation,
                            &place_contract_site,
                            builder.db(),
                            builder.fly_terms(),
                        ),
                    }
                }
            },
            SemExprData::MethodApplication {
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
            SemExprData::MethodFnCall {
                self_argument_sem_expr_idx,
                ident_token,
                ref dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let MethodFlySignature::MethodFn(signature) = dispatch.signature() else {
                    unreachable!()
                };
                HirLazyExprData::MethodRitchieCall {
                    self_argument: self_argument_sem_expr_idx.to_hir_lazy(builder),
                    ident: ident_token.ident(),
                    path: signature.path(),
                    item_groups: builder
                        .new_call_list_item_groups(ritchie_parameter_argument_matches),
                    instantiation: HirInstantiation::from_fly(
                        signature.instantiation(),
                        &place_contract_site,
                        builder.db(),
                        builder.fly_terms(),
                    ),
                    indirections: HirIndirections::from_fly(dispatch.indirections()),
                }
            }
            SemExprData::MethodGnCall { .. } => {
                todo!()
            }
            SemExprData::TemplateInstantiation {
                template: _,
                template_arguments: _,
            } => todo!(),
            SemExprData::At { .. } => unreachable!("this is a type"),
            SemExprData::Unit { .. } => HirLazyExprData::Literal(().into()),
            SemExprData::Delimitered { item, .. } => return item.to_hir_lazy(builder),
            SemExprData::NewTuple {
                lpar_regional_token_idx: _,
                items: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemExprData::Index {
                owner: owner_sem_expr_idx,
                ref index_sem_list_items,
                ..
            } => HirLazyExprData::Index {
                owner: owner_sem_expr_idx.to_hir_lazy(builder),
                items: index_sem_list_items
                    .iter()
                    .map(|item| item.sem_expr_idx.to_hir_lazy(builder))
                    .collect(),
            },
            SemExprData::CompositionWithList { .. } => {
                todo!()
            }
            SemExprData::NewList {
                ref items,
                element_ty,
                ..
            } => HirLazyExprData::ConstructList {
                items: items
                    .iter()
                    .map(|item| item.sem_expr_idx.to_hir_lazy(builder))
                    .collect(),
                element_ty: HirType::from_fly(element_ty, builder.db(), builder.fly_terms())
                    .unwrap(),
            },
            SemExprData::BoxColonList {
                lbox_regional_token_idx: _,
                colon_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemExprData::Block { stmts } => HirLazyExprData::Block {
                stmts: stmts.to_hir_lazy(builder),
            },
            SemExprData::EmptyHtmlTag {
                empty_html_bra_idx: _,
                function_ident: _,
                arguments: _,
                empty_html_ket: _,
            } => todo!(),
            SemExprData::Sorry {
                regional_token_idx: _,
            } => todo!(),
            SemExprData::Todo {
                regional_token_idx: _,
            } => HirLazyExprData::Todo,
            SemExprData::Unreachable {
                regional_token_idx: _,
            } => todo!(),
            SemExprData::VecFunctor {
                lbox_regional_token_idx: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemExprData::ArrayFunctor {
                lbox_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => todo!(),
            SemExprData::Closure { .. } => todo!(),
        };
        builder.alloc_expr(*self, hir_lazy_expr)
    }
}
