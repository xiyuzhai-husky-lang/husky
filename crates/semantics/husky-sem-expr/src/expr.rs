pub mod assoc_item;
pub mod binary;
pub mod box_list;
pub mod closure;
pub mod current_syn_symbol;
pub mod field;
pub mod function_application;
pub mod function_call;
pub mod html;
pub mod index_or_compose_with_list;
pub mod list_item;
pub mod literal;
pub mod method;
pub mod prefix;
pub mod principal_entity_path;
pub mod ritchie_call_arguments_ty;
pub mod suffix;
pub mod template_argument;
pub mod utils;

use std::ops::Index;

pub use self::html::*;
pub use self::list_item::*;
pub use self::ritchie_call_arguments_ty::*;
pub(crate) use self::suffix::*;
pub use self::template_argument::*;

use crate::{obelisks::closure_parameter::ClosureParameterObelisk, *};
use husky_coword::{Ident, IdentMap};
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_eth_signature::TraitForTypeAssocTypeEtherealSignature;
use husky_eth_term::term::EthTerm;
use husky_fly_term::{
    dispatch::{
        dynamic_dispatch::binary_opr::SemaBinaryOprDynamicDispatch, FlyFieldDyanmicDispatch,
        FlyIndexDynamicDispatch, FlyMethodDynamicDispatch, StaticDispatch,
    },
    instantiation::FlyInstantiation,
};
use husky_opr::*;
use husky_regional_token::{
    ColonColonRegionalToken, EmptyHtmlKetRegionalToken, EqRegionalToken, IdentRegionalToken,
    LightArrowRegionalToken, LparRegionalToken, NestedRcurlRegionalToken, PlaceLabelRegionalToken,
    RegionalTokenIdx, RvertRegionalToken,
};
use husky_sem_opr::{binary::SemaBinaryOpr, prefix::SemaPrefixOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::{
    entity_path::SynPrincipalEntityPathSynExprIdx, InheritedSynSymbolIdx, InheritedSynSymbolKind,
};
use husky_syn_opr::{SynBinaryOpr, SynPrefixOpr, SynSuffixOpr};
use husky_term_prelude::ritchie::{RitchieKind, RitchieTypeKind};
use husky_token_data::{IntegerLikeLiteralTokenData, LiteralTokenData, TokenData};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use smallvec::SmallVec;
use vec_like::{AsVecMapEntry, VecMap};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SemaExprData {
    Literal(RegionalTokenIdx, LiteralTokenData),
    /// unit is basically a literal.
    ///
    /// However, due to the complexity of the language, it's not recognized in the lexing stage,
    /// so we need to record it as the combination of two tokens, `(` and `)`.
    Unit {
        lpar_regional_token_idx: RegionalTokenIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    PrincipalEntityPath {
        path_expr_idx: SynPrincipalEntityPathSynExprIdx,
        path: PrincipalEntityPath,
        ty_path_disambiguation: TypePathDisambiguation,
        /// only None if `path` is an ontology constructor
        instantiation: Option<FlyInstantiation>,
    },
    AssocItem {
        parent_expr_idx: SynPrincipalEntityPathSynExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
        static_dispatch: StaticDispatch,
    },
    InheritedSynSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        inherited_syn_symbol_idx: InheritedSynSymbolIdx,
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
    },
    CurrentSynSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        current_syn_symbol_kind: CurrentSynSymbolKind,
    },
    FrameVarDecl {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        frame_var_symbol_idx: CurrentSynSymbolIdx,
        current_syn_symbol_kind: CurrentSynSymbolKind,
    },
    SelfType(RegionalTokenIdx),
    SelfValue(RegionalTokenIdx),
    Binary {
        // todo: coersion?
        lopd: SemaExprIdx,
        opr: SemaBinaryOpr,
        dispatch: SemaBinaryOprDynamicDispatch,
        opr_regional_token_idx: RegionalTokenIdx,
        ropd: SemaExprIdx,
    },
    Be {
        // todo: coersion?
        src: SemaExprIdx,
        be_regional_token_idx: RegionalTokenIdx,
        target: BePatternSyndicate,
    },
    Prefix {
        // todo: coersion?
        opr: SemaPrefixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        opd: SemaExprIdx,
    },
    Suffix {
        // todo: coersion?
        opd: SemaExprIdx,
        opr: SemaSuffixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
    },
    Unveil {
        opd_sem_expr_idx: SemaExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
        unveil_output_ty_signature: TraitForTypeAssocTypeEtherealSignature,
        unveil_assoc_fn_path: TraitForTypeItemPath,
        return_ty: EthTerm,
    },
    Unwrap {
        opd_sem_expr_idx: SemaExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
        // unwrap_method_path: TraitForTypeItemPath,
        // instantiation: FlyInstantiation,
    },
    FunctionApplication {
        function_sem_expr_idx: SemaExprIdx,
        argument_sem_expr_idx: SemaExprIdx,
    },
    FunctionRitchieCall {
        function_sem_expr_idx: SemaExprIdx,
        ritchie_ty_kind: RitchieTypeKind,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieArgumentes,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    /// function type or trait
    Ritchie {
        ritchie_kind_regional_token_idx: RegionalTokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LparRegionalToken,
        parameter_ty_items: SmallVec<[SemaCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
        light_arrow_token: Option<LightArrowRegionalToken>,
        /// it's guaranteed that `return_ty_expr` is some if and only if
        /// `light_arrow_token` is some
        return_ty_sem_expr_idx: Option<SemaExprIdx>,
    },
    Field {
        self_argument: SemaExprIdx,
        self_ty: FlyTerm,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        dispatch: FlyFieldDyanmicDispatch,
    },
    MethodApplication {
        self_argument: SemaExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SemaCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    MethodFnCall {
        self_argument_sem_expr_idx: SemaExprIdx,
        self_contract: Contract,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        // todo: change to FlyMethodFnDynamicDispatch
        dispatch: FlyMethodDynamicDispatch,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieArgumentes,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    MethodGnCall {
        self_argument_sem_expr_idx: SemaExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        method_dynamic_dispatch: FlyMethodDynamicDispatch,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieArgumentes,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    TemplateInstantiation {
        template: SemaExprIdx,
        template_arguments: SemaTemplateArgumentList,
    },
    At {
        at_regional_token_idx: RegionalTokenIdx,
        place_label_regional_token: Option<PlaceLabelRegionalToken>,
    },
    Delimitered {
        lpar_regional_token_idx: RegionalTokenIdx,
        item: SemaExprIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    NewTuple {
        lpar_regional_token_idx: RegionalTokenIdx,
        /// guaranteed that items.len() > 0
        items: SmallVec<[SemaCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    /// there are two cases
    /// - index `$owner[$items]` where `$owner` can be indexed
    /// - application `$owner [$items]` where `$owner` is of type `List _ -> S`
    /// the cases are determined by whether `$owner` is of curry type
    Index {
        owner: SemaExprIdx,
        lbox_regional_token_idx: RegionalTokenIdx,
        index_sem_list_items: SmallVec<[SemaCommaListItem; 2]>,
        rbox_regional_token_idx: RegionalTokenIdx,
        index_dynamic_dispatch: FlyIndexDynamicDispatch,
    },
    CompositionWithList {
        owner: SemaExprIdx,
        lbox_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SemaCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    NewList {
        lbox_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SemaCommaListItem; 4]>,
        element_ty: FlyTerm,
        // todo: disambiguate Vec, SmallList, Array, etc.
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    /// [:] means Slice
    /// [:n] means array as `[_;n]` in Rust
    /// [:n1, n2, ...] means multidimensional array
    BoxColonList {
        lbox_regional_token_idx: RegionalTokenIdx,
        colon_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SemaCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    VecFunctor {
        lbox_regional_token_idx: RegionalTokenIdx,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    ArrayFunctor {
        lbox_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SemaCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    Block {
        stmts: SemaStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        empty_html_bra_idx: RegionalTokenIdx,
        function_ident: IdentRegionalToken,
        arguments: IdentMap<SemaHtmlArgumentExpr>,
        empty_html_ket: EmptyHtmlKetRegionalToken,
    },
    Closure {
        closure_kind_regional_token_idx: Option<RegionalTokenIdx>,
        lvert_regional_token_idx: RegionalTokenIdx,
        parameter_obelisks: Vec<ClosureParameterObelisk>,
        rvert_regional_token: RvertRegionalToken,
        return_ty: Option<(LightArrowRegionalToken, SemaExprIdx, EqRegionalToken)>,
        body: SemaExprIdx,
    },
    /// sorry is for comptime (say proof) terms
    Sorry {
        regional_token_idx: RegionalTokenIdx,
    },
    /// todo is for runtime terms
    Todo {
        regional_token_idx: RegionalTokenIdx,
    },
    Unreachable {
        regional_token_idx: RegionalTokenIdx,
    },
    NestedBlock {
        lcurl_regional_token_idx: RegionalTokenIdx,
        stmts: SemaStmtIdxRange,
        rcurl_regional_token: NestedRcurlRegionalToken,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprEntry {
    data_result: SemaExprDataResult<SemaExprData>,
    immediate_ty_result: SemaExprTypeResult<FlyTerm>,
    expectation_idx_and_ty: Option<(FlyTermExpectationIdx, FlyTerm)>,
}

impl SemaExprEntry {
    /// use this when there is no error guaranteed
    #[track_caller]
    pub fn data(&self) -> &SemaExprData {
        self.data_result
            .as_ref()
            .expect("use this when there is no error guaranteed")
    }

    pub fn data_ok(&self) -> Option<&SemaExprData> {
        self.data_result.as_ref().ok()
    }

    pub fn data_result<'a>(&'a self) -> SemaExprDataResultRef<'a, &'a SemaExprData> {
        self.data_result.as_ref()
    }

    fn ty(&self) -> Option<FlyTerm> {
        self.expectation_idx_and_ty.map(|(_, ty)| ty)
    }

    pub fn original_data_error(&self) -> Option<&OriginalSemaExprDataError> {
        match self.data_result {
            Err(SemaExprDataError::Original(ref e)) => Some(e),
            _ => None,
        }
    }

    pub fn original_ty_error(&self) -> Option<&OriginalSemaExprTypeError> {
        match self.immediate_ty_result {
            Err(SemaExprTypeError::Original(ref e)) => Some(e),
            _ => None,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SemaExprArena(Arena<SemaExprEntry>);

impl SemaExprArena {
    pub(crate) fn alloc_one(
        &mut self,
        data_result: SemaExprDataResult<SemaExprData>,
        immediate_ty_result: SemaExprTypeResult<FlyTerm>,
        expectation_idx_and_ty: Option<(FlyTermExpectationIdx, FlyTerm)>,
    ) -> SemaExprIdx {
        SemaExprIdx(self.0.alloc_one(SemaExprEntry {
            data_result,
            immediate_ty_result,
            expectation_idx_and_ty,
        }))
    }

    pub(crate) fn arena_ref(&self) -> SemaExprArenaRef {
        SemaExprArenaRef(self.0.to_ref())
    }

    pub(crate) fn index_iter(&self) -> impl Iterator<Item = SemaExprIdx> {
        self.0.indices().map(SemaExprIdx)
    }
}

impl std::ops::Index<SemaExprIdx> for SemaExprArena {
    type Output = SemaExprEntry;

    fn index(&self, idx: SemaExprIdx) -> &Self::Output {
        &self.0[idx.0]
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy)]
pub struct SemaExprArenaRef<'a>(ArenaRef<'a, SemaExprEntry>);

impl<'a> SemaExprArenaRef<'a> {
    pub fn len(self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn indexed_iter(self) -> impl Iterator<Item = (SemaExprIdx, &'a SemaExprEntry)> {
        self.0
            .indexed_iter()
            .map(|(idx, entry)| (SemaExprIdx(idx), entry))
    }

    #[inline]
    pub fn iter(self) -> impl Iterator<Item = &'a SemaExprEntry> + 'a {
        self.0.iter()
    }
}

impl<'a> std::ops::Index<SemaExprIdx> for SemaExprArenaRef<'a> {
    type Output = SemaExprEntry;

    fn index(&self, idx: SemaExprIdx) -> &Self::Output {
        &self.0[idx.0]
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemaExprIdx(ArenaIdx<SemaExprEntry>);

impl Into<HoleSource> for SemaExprIdx {
    fn into(self) -> HoleSource {
        HoleSource::SemaExpr(unsafe { std::mem::transmute(self) })
    }
}

impl SemaExprIdx {
    /// panic if there is any error
    ///
    /// use it outside this crate
    #[track_caller]
    pub fn data<'a>(self, arena_ref: SemaExprArenaRef<'a>) -> &'a SemaExprData {
        arena_ref.0.index(self.0).data()
    }

    /// None means something is wrong
    pub fn expectation_outcome<'a>(
        self,
        sem_expr_region: &'a SemaExprRegionData,
    ) -> Option<&'a ExpectationOutcome> {
        let (expectation_idx, _) = sem_expr_region
            .sem_expr_arena()
            .index(self)
            .expectation_idx_and_ty?;
        sem_expr_region.fly_term_region()[expectation_idx]
            .resolve_progress()
            .outcome2()
    }

    /// panic if there is any error
    pub fn data_result<'a>(
        self,
        arena: &'a SemaExprArena,
    ) -> SemaExprDataResultRef<'a, &'a SemaExprData> {
        arena[self].data_result()
    }

    pub fn ty<'a>(self, arena: &'a SemaExprArena) -> FlyTerm {
        arena[self].ty().unwrap()
    }

    /// outside crate wouldn't need to access this
    ///
    /// for downstream crates, it's assumed that there are no semantic errors otherwise the analysis stops at semantic
    pub(crate) fn ok_ty<'a>(self, arena: &'a SemaExprArena) -> Option<FlyTerm> {
        arena[self].ty()
    }

    pub(crate) fn index(self) -> usize {
        self.0.index()
    }
}

pub type SemaExprIdxRange = ArenaIdxRange<SemaExprEntry>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprMap<V>(ArenaMap<SemaExprEntry, V>);

impl<V> SemaExprMap<V> {
    pub fn new(sem_expr_arena: SemaExprArenaRef) -> SemaExprMap<V> {
        Self(ArenaMap::new2(sem_expr_arena.0))
    }

    pub fn insert_new(&mut self, expr: SemaExprIdx, v: V) {
        self.0.insert_new(expr.0, v)
    }

    pub fn get(&self, expr: SemaExprIdx) -> Option<&V> {
        self.0.get(expr.0)
    }

    pub fn get_expr_by_value_copied(&self, v: V) -> SemaExprIdx
    where
        V: PartialEq + Copy,
    {
        SemaExprIdx(
            self.0
                .key_value_iter()
                .find_map(|(expr, &v1)| (v == v1).then_some(expr))
                .unwrap(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = (SemaExprIdx, &V)> {
        self.0
            .key_value_iter()
            .map(|(idx, v)| (SemaExprIdx(idx), v))
    }
}

impl<V> std::ops::Index<SemaExprIdx> for SemaExprMap<V> {
    type Output = V;

    fn index(&self, index: SemaExprIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<'a> SemaExprBuilder<'a> {
    pub(crate) fn build_all_exprs(&mut self) {
        for root in self.syn_expr_region_data().syn_expr_roots() {
            let sem_expr_idx = match root.kind() {
                SynExprRootKind::SelfType
                | SynExprRootKind::ReturnType
                | SynExprRootKind::ReturnType
                | SynExprRootKind::PropsStructFieldType { .. }
                | SynExprRootKind::TupleStructFieldType
                | SynExprRootKind::ConstantImplicitParameterType
                | SynExprRootKind::ExplicitParameterType
                | SynExprRootKind::TypeAliasTypeTerm
                | SynExprRootKind::AssocTypeTerm => {
                    let sem_expr_idx = self.build_sem_expr(root.syn_expr_idx(), ExpectSort::TYPE);
                    self.infer_expr_term(sem_expr_idx);
                    sem_expr_idx
                }
                SynExprRootKind::Trait => {
                    let sem_expr_idx = self.build_sem_expr(root.syn_expr_idx(), ExpectAnyOriginal);
                    self.infer_expr_term(sem_expr_idx);
                    sem_expr_idx
                }
                SynExprRootKind::BlockExpr => match self.return_ty() {
                    Some(return_ty) => self.build_sem_expr(
                        root.syn_expr_idx(),
                        ExpectCoersion::new_move(return_ty.into()),
                    ),
                    None => self.build_sem_expr(root.syn_expr_idx(), ExpectAnyDerived),
                },
                SynExprRootKind::FieldBindInitialValue { ty_syn_expr_idx }
                | SynExprRootKind::ParenateParameterDefaultValue { ty_syn_expr_idx } => {
                    let (ty_sem_expr_idx, _) = self.sem_expr_roots[ty_syn_expr_idx].1;
                    match self.infer_expr_term(ty_sem_expr_idx) {
                        Some(ty) => {
                            self.build_sem_expr(root.syn_expr_idx(), ExpectCoersion::new_move(ty))
                        }
                        _ => todo!(),
                    }
                }
                SynExprRootKind::ReturnExpr
                | SynExprRootKind::Condition
                | SynExprRootKind::HtmlArgumentExpr
                | SynExprRootKind::LetStmtType
                | SynExprRootKind::LetStmtInitialValue
                | SynExprRootKind::EvalExpr => continue,
                SynExprRootKind::Snippet => todo!(),
                SynExprRootKind::ValExpr => todo!(),
            };
            self.sem_expr_roots
                .insert_new((root.syn_expr_idx(), (sem_expr_idx, root.kind())))
                .expect("impossible")
        }
    }

    pub(crate) fn build_sem_expr_with_ty<E: ExpectFlyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<FlyTerm>) {
        let (sem_expr_idx, expectation_idx_and_ty) =
            self.build_sem_expr_aux(expr_idx, expr_ty_expectation);
        (sem_expr_idx, expectation_idx_and_ty.map(|(_, ty)| ty))
    }

    pub(crate) fn build_sem_expr_with_ty_and_outcome<E: ExpectFlyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<FlyTerm>, Option<ExpectationOutcome>) {
        let (sem_expr_idx, expectation_idx_and_ty) =
            self.build_sem_expr_aux(expr_idx, expr_ty_expectation);
        let (ty, outcome) = match expectation_idx_and_ty {
            Some((expectation_idx, ty)) => (
                Some(ty),
                self.fly_term_region()[expectation_idx]
                    .resolve_progress()
                    .outcome::<E>()
                    .cloned(),
            ),
            None => (None, None),
        };
        (sem_expr_idx, ty, outcome.map(Into::into))
    }

    /// infer the type of a new expression but don't need the result for now
    pub(crate) fn build_sem_expr<E: ExpectFlyTerm>(
        &mut self,
        syn_expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> SemaExprIdx {
        let (sem_expr_idx, _) = self.build_sem_expr_aux(syn_expr_idx, expr_ty_expectation);
        sem_expr_idx
    }

    #[inline(always)]
    pub(crate) fn build_sem_expr_with_outcome<E: ExpectFlyTerm>(
        &mut self,
        syn_expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<E::Outcome>)
    where
        E::Outcome: Clone,
    {
        let (sem_expr_idx, expectation_idx) =
            self.build_sem_expr_aux(syn_expr_idx, expr_ty_expectation);
        let outcome = match expectation_idx {
            Some((expectation_idx, _)) => self.fly_term_region()[expectation_idx]
                .resolve_progress()
                .outcome::<E>()
                .cloned(),
            None => None,
        };
        (sem_expr_idx, outcome)
    }

    #[inline(always)]
    fn build_sem_expr_aux<E: ExpectFlyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<(FlyTermExpectationIdx, FlyTerm)>) {
        let (data_result, immediate_ty_result) =
            self.build_sem_expr_data_and_ty_result(expr_idx, &expr_ty_expectation);
        let expectation_idx_and_ty = match immediate_ty_result {
            Ok(ty) => Some(self.add_expectation(
                ExpectationSource::new_expr(expr_idx),
                ty,
                expr_ty_expectation,
            )),
            _ => None,
        };
        let sem_expr_idx =
            self.alloc_expr(data_result, immediate_ty_result, expectation_idx_and_ty);
        (sem_expr_idx, expectation_idx_and_ty)
    }

    pub(crate) fn build_unit_sem_expr<E: ExpectFlyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        lpar_regional_token_idx: RegionalTokenIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
        expr_ty_expectation: E,
    ) -> SemaExprIdx {
        let ty = match expr_ty_expectation.final_destination(self) {
            FinalDestination::Sort => self.eth_term_menu().ty0().into(),
            FinalDestination::TypeOntology => self.eth_term_menu().unit_ty_ontology(),
            FinalDestination::AnyOriginal => todo!(),
            FinalDestination::AnyDerived => todo!(),
            FinalDestination::Ritchie(_) => todo!(),
        };
        let expectation_idx_and_ty = self.add_expectation(
            ExpectationSource::new_expr(expr_idx),
            ty.into(),
            expr_ty_expectation,
        );
        let sem_expr_idx = self.alloc_expr(
            Ok(SemaExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            }),
            Ok(ty.into()),
            Some(expectation_idx_and_ty),
        );
        sem_expr_idx
    }

    fn build_sem_expr_data_and_ty_result(
        &mut self,
        syn_expr_idx: SynExprIdx,
        expr_ty_expectation: &impl ExpectFlyTerm,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        match self.syn_expr_region_data()[syn_expr_idx] {
            SynExprData::Literal(literal_token_idx, literal_data) => (
                Ok(SemaExprData::Literal(literal_token_idx, literal_data)),
                self.calc_literal_expr_ty(syn_expr_idx, literal_token_idx, expr_ty_expectation),
            ),
            SynExprData::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            } => match opt_path {
                Some(path) => {
                    let ty_path_disambiguation = expr_ty_expectation.disambiguate_ty_path(self);
                    let (instantiation_result, ty_result) = self.calc_principal_item_path_expr_ty(
                        syn_expr_idx,
                        path,
                        expr_ty_expectation,
                        ty_path_disambiguation,
                    );
                    (
                        instantiation_result.map(|instantiation| {
                            SemaExprData::PrincipalEntityPath {
                                path_expr_idx,
                                path,
                                ty_path_disambiguation,
                                instantiation,
                            }
                        }),
                        ty_result,
                    )
                }
                None => {
                    // let Some(path) = path else {
                    //     Err(DerivedSemaExprTypeError::EntityPathError)?
                    // };
                    todo!()
                }
            },
            SynExprData::PrincipalEntityPathAssocItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => {
                let (static_dispatch_result, ty_result) =
                    self.calc_assoc_item_ty(syn_expr_idx, parent_path, ident_token);
                let data_result =
                    static_dispatch_result.map(|static_dispatch| SemaExprData::AssocItem {
                        parent_expr_idx,
                        parent_path,
                        colon_colon_regional_token,
                        ident_token,
                        static_dispatch,
                    });
                (data_result, ty_result)
            }
            SynExprData::AssocItem {
                parent_expr_idx,
                colon_colon_regional_token_idx,
                ident,
                ident_regional_token_idx,
            } => todo!(),
            SynExprData::InheritedSynSymbol {
                ident,
                regional_token_idx,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
            } => (
                Ok(SemaExprData::InheritedSynSymbol {
                    ident,
                    regional_token_idx,
                    inherited_syn_symbol_idx,
                    inherited_syn_symbol_kind,
                }),
                match self
                    .symbol_tys()
                    .inherited_syn_symbol_map()
                    .get(inherited_syn_symbol_idx)
                {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedSemaExprTypeError::InheritedSynSymbolTypeError.into()),
                },
            ),
            SynExprData::CurrentSynSymbol {
                ident,
                regional_token_idx,
                current_syn_symbol_idx,
                current_syn_symbol_kind,
            } => (
                Ok(SemaExprData::CurrentSynSymbol {
                    ident,
                    regional_token_idx,
                    current_syn_symbol_idx,
                    current_syn_symbol_kind,
                }),
                self.get_current_syn_symbol_ty(syn_expr_idx, current_syn_symbol_idx),
            ),
            SynExprData::FrameVarDecl {
                ident,
                regional_token_idx,
                frame_var_symbol_idx,
                current_syn_symbol_kind,
            } => (
                Ok(SemaExprData::FrameVarDecl {
                    ident,
                    regional_token_idx,
                    frame_var_symbol_idx,
                    current_syn_symbol_kind,
                }),
                self.get_current_syn_symbol_ty(syn_expr_idx, frame_var_symbol_idx),
            ),
            SynExprData::SelfType(regional_token_idx) => (
                Ok(SemaExprData::SelfType(regional_token_idx)),
                match self.self_ty() {
                    Some(self_ty) => match self_ty.ty_unchecked(self.db()) {
                        Ok(Left(self_ty_ty)) => Ok(self_ty_ty.into()),
                        Err(e) => Err(e.into()),
                        Ok(Right(_)) => unreachable!(),
                    }, // todo: impl binding
                    None => Err(DerivedSemaExprTypeError::SelfTypeNotInferred.into()),
                },
            ),
            SynExprData::SelfValue(regional_token_idx) => (
                Ok(SemaExprData::SelfValue(regional_token_idx)),
                match self.self_value_ty() {
                    Some(self_value_ty) => Ok(self_value_ty),
                    None => Err(DerivedSemaExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            ),
            SynExprData::Binary {
                lopd,
                opr,
                opr_regional_token_idx,
                ropd,
            } => {
                let (lopd, opr, ropd, dispatch_result, ty_result) =
                    self.calc_binary_expr_ty(syn_expr_idx, lopd, opr, ropd);
                (
                    dispatch_result.map(|dispatch| SemaExprData::Binary {
                        lopd,
                        opr,
                        opr_regional_token_idx,
                        ropd,
                        dispatch,
                    }),
                    ty_result,
                )
            }
            SynExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => {
                let (src, src_ty) = self.build_sem_expr_with_ty(src, ExpectAnyOriginal);
                match src_ty {
                    Some(src_ty) => match target {
                        Ok(target) => self.infer_variable_pattern_root_and_symbols_ty(
                            target.syn_pattern_root(),
                            src_ty,
                            target.variables(),
                        ),
                        Err(_) => (),
                    },
                    None => (),
                };
                let data_result = target
                    .as_ref()
                    .map(|&target| SemaExprData::Be {
                        src,
                        be_regional_token_idx,
                        target,
                    })
                    .map_err(|e| e.into());
                (data_result, Ok(self.term_menu().bool_ty_ontology().into()))
            }
            SynExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd,
            } => {
                let (opd_sem_expr_idx_and_opr_result, ty_result) = self.build_prefix_sem_expr(
                    syn_expr_idx,
                    opr,
                    opd,
                    expr_ty_expectation.final_destination(self),
                );
                match opd_sem_expr_idx_and_opr_result {
                    Ok((opd_sem_expr_idx, opr)) => (
                        Ok(SemaExprData::Prefix {
                            opr,
                            opr_regional_token_idx,
                            opd: opd_sem_expr_idx,
                        }),
                        ty_result,
                    ),
                    Err(_) => todo!(),
                }
            }
            SynExprData::Suffix {
                opd,
                opr,
                opr_regional_token_idx,
            } => {
                let (data_result, ty_result) = self.calc_suffix_expr_ty(
                    syn_expr_idx,
                    opd,
                    opr,
                    opr_regional_token_idx,
                    expr_ty_expectation.final_destination(self),
                );
                (data_result, ty_result)
            }
            SynExprData::FunctionApplicationOrCall {
                function,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => self.build_function_application_or_call_sem_expr(
                syn_expr_idx,
                function,
                expr_ty_expectation,
                template_arguments.as_ref(),
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
            ),
            SynExprData::FunctionCall {
                function,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => self.build_function_call_sem_expr(
                syn_expr_idx,
                function,
                expr_ty_expectation.final_destination(self),
                template_arguments.as_ref(),
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
            ),
            SynExprData::Field {
                owner,
                dot_regional_token_idx,
                ident_token,
            } => self.calc_field_expr_ty(owner, dot_regional_token_idx, ident_token),
            SynExprData::MethodApplicationOrCall {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                lpar_regional_token_idx,
                ref template_arguments,
                ref items,
                rpar_regional_token_idx,
            } => self.calc_method_application_or_call_ty(
                syn_expr_idx,
                self_argument,
                dot_regional_token_idx,
                ident_token,
                template_arguments.as_ref(),
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
            ),
            SynExprData::TemplateInstantiation {
                template,
                template_arguments: ref generic_arguments,
            } => todo!(),
            SynExprData::ExplicitApplication {
                function_expr_idx,
                argument_expr_idx,
            } => self.build_explicit_application_sem_expr(
                syn_expr_idx,
                function_expr_idx,
                argument_expr_idx,
                expr_ty_expectation.final_destination(self),
            ),
            SynExprData::Delimitered {
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => {
                let (item, infer_new_expr_ty) =
                    self.build_sem_expr_with_ty(item, expr_ty_expectation.clone());
                (
                    Ok(SemaExprData::Delimitered {
                        lpar_regional_token_idx,
                        item,
                        rpar_regional_token_idx,
                    }),
                    infer_new_expr_ty
                        .ok_or(DerivedSemaExprTypeError::DelimiteredItemTypeError.into()),
                )
            }
            SynExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => (
                Ok(SemaExprData::At {
                    at_regional_token_idx,
                    place_label_regional_token,
                }),
                Ok(self.term_menu().ex_inv_ty0_to_ty0().into()),
            ),
            SynExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => (
                Ok(SemaExprData::Unit {
                    lpar_regional_token_idx,
                    rpar_regional_token_idx,
                }),
                match expr_ty_expectation.final_destination(self) {
                    FinalDestination::Sort => Ok(self.term_menu().ty0().into()),
                    FinalDestination::TypeOntology
                    | FinalDestination::AnyOriginal
                    | FinalDestination::AnyDerived => {
                        Ok(self.term_menu().unit_ty_ontology().into())
                    }
                    FinalDestination::Ritchie(_) => todo!(),
                },
            ),
            SynExprData::NewTuple { ref items, .. } => todo!(),
            SynExprData::IndexOrCompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => self.calc_index_or_compose_with_list_expr_ty(
                syn_expr_idx,
                owner,
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            ),
            SynExprData::List {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => {
                match expr_ty_expectation.disambiguate_ty_path(self) {
                    TypePathDisambiguation::OntologyConstructor => {
                        // ad hoc, assume universe is 1
                        match items.len() {
                            0 => (
                                Ok(SemaExprData::VecFunctor {
                                    lbox_regional_token_idx,
                                    rbox_regional_token_idx,
                                }
                                .into()),
                                Ok(self.term_menu().ex_co_ty0_to_ty0().into()),
                            ),
                            1 => (
                                Ok(SemaExprData::ArrayFunctor {
                                    lbox_regional_token_idx,
                                    items: items
                                        .iter()
                                        .map(|&syn_comma_list_item| {
                                            self.build_sem_comma_list_item(
                                                syn_comma_list_item,
                                                ExpectCoersion::new_const(
                                                    self.term_menu().usize_ty_ontology().into(),
                                                ),
                                            )
                                        })
                                        .collect(),
                                    rbox_regional_token_idx,
                                }
                                .into()),
                                Ok(self.term_menu().ex_co_ty0_to_ty0().into()),
                            ),
                            _ => {
                                print_debug_expr!(self, syn_expr_idx);
                                todo!()
                            }
                        }
                    }
                    TypePathDisambiguation::InstanceConstructor => {
                        let element_ty: FlyTerm = match expr_ty_expectation.destination() {
                            FlyTermDestination::Specific(ty_pattern) => match ty_pattern
                                .data_inner(self.db(), self.fly_term_region().terms())
                            {
                                FlyTermData::Literal(_) => todo!(),
                                FlyTermData::TypeOntology {
                                    refined_ty_path,
                                    ty_arguments,
                                    ..
                                } => match refined_ty_path {
                                    Left(PreludeTypePath::List) => {
                                        assert_eq!(ty_arguments.len(), 1);
                                        ty_arguments[0]
                                    }
                                    Left(PreludeTypePath::Container(_)) => {
                                        assert_eq!(ty_arguments.len(), 1);
                                        ty_arguments[0]
                                    }
                                    _ => todo!(),
                                },
                                FlyTermData::Curry {
                                    toolchain,
                                    curry_kind,
                                    variance,
                                    parameter_hvar,
                                    parameter_ty,
                                    return_ty,
                                    ty_ethereal_term,
                                } => todo!(),
                                FlyTermData::Hole(_, _) => todo!(),
                                FlyTermData::Sort(_) => todo!(),
                                FlyTermData::Ritchie {
                                    ritchie_kind,
                                    parameter_contracted_tys,
                                    return_ty,
                                    ..
                                } => todo!(),
                                FlyTermData::Symbol { .. } => todo!(),
                                FlyTermData::Hvar { .. } => todo!(),
                                FlyTermData::TypeVariant { path } => todo!(),
                            },
                            FlyTermDestination::AnyOriginal => {
                                self.new_hole(syn_expr_idx, HoleKind::AnyOriginal).into()
                            }
                            FlyTermDestination::AnyDerived => {
                                self.new_hole(syn_expr_idx, HoleKind::AnyDerived).into()
                            }
                        };
                        (
                            Ok(SemaExprData::NewList {
                                lbox_regional_token_idx,
                                items: items
                                    .iter()
                                    .map(|&syn_comma_list_item| {
                                        self.build_sem_comma_list_item(
                                            syn_comma_list_item,
                                            ExpectCoersion::new_move(element_ty),
                                        )
                                    })
                                    .collect(),
                                element_ty,
                                rbox_regional_token_idx,
                            }),
                            FlyTerm::new_application(
                                self,
                                self.term_menu().list_ty_ontology(),
                                element_ty,
                            )
                            .map_err(|_| todo!()),
                        )
                    }
                }
            }
            SynExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                rbox_regional_token_idx,
                ref items,
            } => match items.len() {
                0 => (
                    Ok(SemaExprData::BoxColonList {
                        lbox_regional_token_idx,
                        colon_regional_token_idx,
                        items: items.iter().map(|item| todo!()).collect(),
                        rbox_regional_token_idx,
                    }),
                    Ok(self.term_menu().ex_co_ty0_to_ty0().into()),
                ),
                _ => todo!(),
            },
            SynExprData::Block { stmts } => {
                let (stmts, block_ty) =
                    self.build_sem_stmts_with_its_ty_returned(stmts, expr_ty_expectation.clone());
                (
                    Ok(SemaExprData::Block { stmts }),
                    block_ty.ok_or(DerivedSemaExprTypeError::BlockTypeError.into()),
                )
            }
            SynExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => {
                let (stmts, block_ty) =
                    self.build_sem_stmts_with_its_ty_returned(stmts, expr_ty_expectation.clone());
                (
                    Ok(SemaExprData::NestedBlock {
                        lcurl_regional_token_idx,
                        stmts,
                        rcurl_regional_token,
                    }),
                    block_ty.ok_or(DerivedSemaExprTypeError::BlockTypeError.into()),
                )
            }
            SynExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => (
                Ok(SemaExprData::EmptyHtmlTag {
                    empty_html_bra_idx,
                    function_ident,
                    arguments: unsafe {
                        VecMap::from_iter_assuming_no_repetitions_unchecked(
                            arguments
                                .iter()
                                .map(|&argument| self.build_sem_html_argument_expr(argument)),
                        )
                    },
                    empty_html_ket,
                }),
                Ok(self.term_menu().html_ty_ontology().into()),
            ),
            SynExprData::Ritchie {
                ritchie_kind,
                ritchie_kind_regional_token_idx,
                lpar_token,
                ref parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty_syn_expr_idx,
            } => {
                let parameter_ty_items = parameter_ty_items
                    .iter()
                    .map(|&syn_comma_list_item| {
                        self.build_sem_comma_list_item(
                            syn_comma_list_item,
                            self.expect_ty0_subtype(),
                        )
                    })
                    .collect();
                let return_ty_sem_expr_idx = return_ty_syn_expr_idx.map(|return_ty_syn_expr_idx| {
                    self.build_sem_expr(return_ty_syn_expr_idx, self.expect_ty0_subtype())
                });
                (
                    Ok(SemaExprData::Ritchie {
                        ritchie_kind_regional_token_idx,
                        ritchie_kind,
                        lpar_token,
                        parameter_ty_items,
                        rpar_regional_token_idx,
                        light_arrow_token,
                        return_ty_sem_expr_idx,
                    }),
                    Ok(self.term_menu().ty0().into()),
                )
            }
            SynExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                ref parameters,
                rvert_regional_token,
                return_ty,
                body,
            } => self.build_closure_expr(
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                parameters.elements(),
                rvert_regional_token,
                return_ty.map(|(light_arrow, expr, eq)| (light_arrow, expr, eq)),
                body,
                expr_ty_expectation,
            ),
            SynExprData::Sorry { regional_token_idx } => (
                Ok(SemaExprData::Sorry { regional_token_idx }),
                Ok(self.term_menu().never().into()),
            ),
            SynExprData::Todo { regional_token_idx } => (
                Ok(SemaExprData::Todo { regional_token_idx }),
                Ok(self.term_menu().never().into()),
            ),
            SynExprData::Unreachable { regional_token_idx } => (
                Ok(SemaExprData::Unreachable { regional_token_idx }),
                Ok(self.term_menu().never().into()),
            ),
            SynExprData::Err(ref e) => (
                Err(DerivedSemaExprDataError::SynExpr.into()),
                Err(DerivedSemaExprTypeError::SynExprError.into()),
            ),
        }
    }

    fn build_explicit_application_sem_expr(
        &mut self,
        expr_idx: SynExprIdx,
        function_expr_idx: SynExprIdx,
        argument_expr_idx: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        self.calc_function_application_expr_ty(
            expr_idx,
            function_expr_idx,
            argument_expr_idx,
            final_destination,
        )
    }
}
