mod html;
mod list_item;
mod template_argument;

pub use self::html::*;
pub use self::list_item::*;
pub use self::template_argument::*;

use crate::*;
use husky_coword::{Ident, IdentMap};
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_ethereal_signature::TraitForTypeAssociatedTypeEtherealSignature;
use husky_fluffy_term::{
    dispatch::dynamic_dispatch::binary_opr::SemaBinaryOprDynamicDispatch,
    instantiation::FluffyInstantiation,
};
use husky_regional_token::{
    ColonColonRegionalToken, EmptyHtmlKetRegionalToken, IdentRegionalToken,
    LightArrowRegionalToken, LparRegionalToken, PlaceLabelRegionalToken, RegionalTokenIdx,
};
use husky_sema_opr::{binary::SemaBinaryOpr, prefix::SemaPrefixOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::{
    InheritedSynSymbolIdx, InheritedSynSymbolKind, SynPrincipalEntityPathExprIdx,
};
use husky_syn_opr::{SynBinaryOpr, SynPrefixOpr, SynSuffixOpr};
use husky_term_prelude::RitchieKind;
use husky_token_data::LiteralData;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use smallvec::SmallVec;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SemaExprData {
    Literal(RegionalTokenIdx, LiteralData),
    PrincipalEntityPath {
        path_expr_idx: SynPrincipalEntityPathExprIdx,
        path: PrincipalEntityPath,
        ty_path_disambiguation: TypePathDisambiguation,
        instantiation: Option<FluffyInstantiation>,
    },
    AssociatedItem {
        parent_expr_idx: SynPrincipalEntityPathExprIdx,
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
        lopd: SemaExprIdx,
        opr: SemaBinaryOpr,
        dispatch: SemaBinaryOprDynamicDispatch,
        opr_regional_token_idx: RegionalTokenIdx,
        ropd: SemaExprIdx,
    },
    Be {
        src: SemaExprIdx,
        be_regional_token_idx: RegionalTokenIdx,
        target: BePatternSynSyndicate,
    },
    Prefix {
        opr: SemaPrefixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        opd_sema_expr_idx: SemaExprIdx,
    },
    Suffix {
        opd_sema_expr_idx: SemaExprIdx,
        opr: SemaSuffixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
    },
    Unveil {
        opd_sema_expr_idx: SemaExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
        unveil_output_ty_signature: TraitForTypeAssociatedTypeEtherealSignature,
        unveil_associated_fn_path: TraitForTypeItemPath,
        return_ty: EtherealTerm,
    },
    Unwrap {
        opd_sema_expr_idx: SemaExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
        // unwrap_method_path: TraitForTypeItemPath,
        // instantiation: FluffyInstantiation,
    },
    FunctionApplication {
        function_sema_expr_idx: SemaExprIdx,
        argument_sema_expr_idx: SemaExprIdx,
    },
    FunctionFnCall {
        function_sema_expr_idx: SemaExprIdx,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    FunctionGnCall {
        function: SemaExprIdx,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
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
        return_ty_sema_expr_idx: Option<SemaExprIdx>,
    },
    Field {
        owner_sema_expr_idx: SemaExprIdx,
        owner_ty: FluffyTerm,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        dispatch: FluffyFieldDyanmicDispatch,
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
        self_argument_sema_expr_idx: SemaExprIdx,
        self_contract: TermContract,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        // todo: change to FluffyMethodFnDynamicDispatch
        dispatch: FluffyMethodDynamicDispatch,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    MethodGnCall {
        self_argument_sema_expr_idx: SemaExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        method_dynamic_dispatch: FluffyMethodDynamicDispatch,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
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
    Unit {
        lpar_regional_token_idx: RegionalTokenIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    Bracketed {
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
        owner_sema_expr_idx: SemaExprIdx,
        lbox_regional_token_idx: RegionalTokenIdx,
        index_sema_list_items: SmallVec<[SemaCommaListItem; 2]>,
        rbox_regional_token_idx: RegionalTokenIdx,
        index_dynamic_dispatch: FluffyIndexDynamicDispatch,
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
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprEntry {
    data_result: SemaExprDataResult<SemaExprData>,
    immediate_ty_result: SemaExprTypeResult<FluffyTerm>,
    expectation_idx_and_ty: Option<(FluffyTermExpectationIdx, FluffyTerm)>,
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

    fn ty(&self) -> Option<FluffyTerm> {
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SemaExprArena(Arena<SemaExprEntry>);

impl SemaExprArena {
    pub(crate) fn alloc_one(
        &mut self,
        data_result: SemaExprDataResult<SemaExprData>,
        immediate_ty_result: SemaExprTypeResult<FluffyTerm>,
        expectation_idx_and_ty: Option<(FluffyTermExpectationIdx, FluffyTerm)>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemaExprIdx(ArenaIdx<SemaExprEntry>);

impl SemaExprIdx {
    /// panic if there is any error
    ///
    /// use it outside this crate
    #[track_caller]
    pub fn data<'a>(self, arena_ref: SemaExprArenaRef<'a>) -> &'a SemaExprData {
        arena_ref.0.index(self.0).data()
    }

    /// panic if there is any error
    pub fn data_result<'a>(
        self,
        arena: &'a SemaExprArena,
    ) -> SemaExprDataResultRef<'a, &'a SemaExprData> {
        arena[self].data_result()
    }

    pub fn ty<'a>(self, arena: &'a SemaExprArena) -> FluffyTerm {
        arena[self].ty().unwrap()
    }

    pub(crate) fn ok_ty<'a>(self, arena: &'a SemaExprArena) -> Option<FluffyTerm> {
        arena[self].ty()
    }

    pub(crate) fn index(self) -> usize {
        self.0.index()
    }
}

pub type SemaExprIdxRange = ArenaIdxRange<SemaExprEntry>;

#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprMap<V>(ArenaMap<SemaExprEntry, V>);

impl<V> SemaExprMap<V> {
    pub fn new(sema_expr_arena: SemaExprArenaRef) -> SemaExprMap<V> {
        Self(ArenaMap::new2(sema_expr_arena.0))
    }

    pub fn insert_new(&mut self, expr_idx: SemaExprIdx, v: V) {
        self.0.insert_new(expr_idx.0, v)
    }

    pub fn get(&self, sema_expr_idx: SemaExprIdx) -> Option<&V> {
        self.0.get(sema_expr_idx.0)
    }
}

impl<V> std::ops::Index<SemaExprIdx> for SemaExprMap<V> {
    type Output = V;

    fn index(&self, index: SemaExprIdx) -> &Self::Output {
        &self.0[index.0]
    }
}
