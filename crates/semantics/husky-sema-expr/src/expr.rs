mod html;
mod list_item;
mod template_argument;

pub use self::html::*;
pub use self::list_item::*;
pub use self::template_argument::*;

use crate::*;
use husky_coword::{Ident, IdentMap};
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_opr::{BinaryOpr, PrefixOpr, SuffixOpr};
use husky_regional_token::{
    ColonColonRegionalToken, EmptyHtmlKetRegionalToken, IdentRegionalToken,
    LightArrowRegionalToken, LparRegionalToken, PlaceLabelRegionalToken, RegionalTokenIdx,
};
use husky_sema_opr::{prefix::SemaPrefixOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::{
    SynInheritedSymbolIdx, SynInheritedSymbolKind, SynPrincipalEntityPathExprIdx,
};
use husky_term_prelude::RitchieKind;
use husky_token_data::LiteralData;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq)]
pub enum SemaExprData {
    Literal(RegionalTokenIdx, LiteralData),
    PrincipalEntityPath {
        path_expr_idx: SynPrincipalEntityPathExprIdx,
        path: PrincipalEntityPath,
        ty_path_disambiguation: TypePathDisambiguation,
    },
    AssociatedItem {
        parent_expr_idx: SynPrincipalEntityPathExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
        static_dispatch: StaticDispatch,
    },
    InheritedSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        inherited_symbol_idx: SynInheritedSymbolIdx,
        inherited_symbol_kind: SynInheritedSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        current_symbol_idx: SynCurrentSymbolIdx,
        current_symbol_kind: SynCurrentSymbolKind,
    },
    FrameVarDecl {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        frame_var_symbol_idx: SynCurrentSymbolIdx,
        current_symbol_kind: SynCurrentSymbolKind,
    },
    SelfType(RegionalTokenIdx),
    SelfValue(RegionalTokenIdx),
    Binary {
        lopd: SemaExprIdx,
        opr: BinaryOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        ropd: SemaExprIdx,
    },
    Be {
        src: SemaExprIdx,
        be_regional_token_idx: RegionalTokenIdx,
        target: BePatternSynObelisk,
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
    // todo: implicit arguments
    Application {
        function_sema_expr_idx: SemaExprIdx,
        argument_sema_expr_idx: SemaExprIdx,
    },
    FnCall {
        function_sema_expr_idx: SemaExprIdx,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    GnCall {
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
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        field_dispatch: FluffyFieldDyanmicDispatch,
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
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        method_dynamic_dispatch: FluffyTermMethodDynamicDispatch,
        template_arguments: Option<SemaTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    MethodGnCall {
        self_argument_sema_expr_idx: SemaExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        method_dynamic_dispatch: FluffyTermMethodDynamicDispatch,
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

#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprEntry {
    data_result: SemaExprDataResult<SemaExprData>,
    ty_result: SemaExprTypeResult<FluffyTerm>,
}

impl SemaExprEntry {
    /// use this when there is no error guaranteed
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

    pub fn ty_result(&self) -> SemaExprTypeResultRef<FluffyTerm> {
        self.ty_result.as_ref().copied()
    }

    fn ok_ty(&self) -> Option<FluffyTerm> {
        self.ty_result.as_ref().ok().copied()
    }

    pub fn original_data_error(&self) -> Option<&OriginalSemaExprDataError> {
        match self.data_result {
            Err(SemaExprDataError::Original(ref e)) => Some(e),
            _ => None,
        }
    }

    pub fn original_ty_error(&self) -> Option<&OriginalSemaExprTypeError> {
        match self.ty_result {
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
        ty_result: SemaExprTypeResult<FluffyTerm>,
    ) -> SemaExprIdx {
        SemaExprIdx(self.0.alloc_one(SemaExprEntry {
            data_result,
            ty_result,
        }))
    }

    pub(crate) fn arena_ref(&self) -> SemaExprArenaRef {
        SemaExprArenaRef(self.0.arena_ref())
    }

    pub(crate) fn index_iter(&self) -> impl Iterator<Item = SemaExprIdx> {
        self.0.index_iter().map(SemaExprIdx)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemaExprIdx(ArenaIdx<SemaExprEntry>);

impl SemaExprIdx {
    /// panic if there is any error
    ///
    /// use it outside this crate
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

    pub(crate) fn ok_ty<'a>(self, arena: &'a SemaExprArena) -> Option<FluffyTerm> {
        arena[self].ok_ty()
    }
}

pub type SemaExprIdxRange = ArenaIdxRange<SemaExprEntry>;
pub type SemaExprMap<V> = ArenaMap<SemaExprEntry, V>;
