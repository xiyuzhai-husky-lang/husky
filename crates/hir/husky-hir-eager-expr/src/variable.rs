use super::*;
use husky_syn_expr::{
    CurrentSynSymbol, CurrentSynSymbolKind, CurrentSynSymbolVariant,
    CurrentTemplateParameterSynSymbolVariant, InheritedSynSymbol, InheritedSynSymbolKind,
    SynSymbolMap, SynSymbolOrderedMap, SynSymbolRegion,
};
use idx_arena::ArenaIdx;

pub enum HirEagerVariable {
    ParenateParameter,
    LetVariable,
    BeVariable,
    CaseVariable,
    FieldVariable,
    LoopVariable,
}

impl HirEagerVariable {
    fn from_inherited_syn(inherited_syn_symbol: InheritedSynSymbol) -> Option<HirEagerVariable> {
        match inherited_syn_symbol.kind() {
            InheritedSynSymbolKind::TemplateParameter(_) => None,
            InheritedSynSymbolKind::ParenateParameter { ident } => {
                Some(HirEagerVariable::ParenateParameter)
            }
            InheritedSynSymbolKind::FieldVariable { ident } => {
                Some(HirEagerVariable::FieldVariable)
            }
        }
    }

    fn from_current_syn(current_syn_symbol: &CurrentSynSymbol) -> Option<Self> {
        match *current_syn_symbol.variant() {
            CurrentSynSymbolVariant::TemplateParameter { .. } => None,
            CurrentSynSymbolVariant::SelfType => todo!(),
            CurrentSynSymbolVariant::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            CurrentSynSymbolVariant::ParenateRegularParameter {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariable::ParenateParameter),
            CurrentSynSymbolVariant::ParenateVariadicParameter {
                symbol_modifier_keyword_group,
                ident_token,
            } => Some(HirEagerVariable::ParenateParameter),
            CurrentSynSymbolVariant::LetVariable {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariable::LetVariable),
            CurrentSynSymbolVariant::BeVariable {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariable::BeVariable),
            CurrentSynSymbolVariant::CaseVariable {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariable::CaseVariable),
            CurrentSynSymbolVariant::FieldVariable { ident_token } => {
                Some(HirEagerVariable::FieldVariable)
            }
            CurrentSynSymbolVariant::LoopVariable { ident, expr_idx } => {
                Some(HirEagerVariable::LoopVariable)
            }
        }
    }
}

pub type HirEagerVariableArena = Arena<HirEagerVariable>;
pub type HirEagerVariableIdx = ArenaIdx<HirEagerVariable>;

pub struct HirEagerVariableRegion {
    arena: HirEagerVariableArena,
}

impl HirEagerVariableRegion {
    pub(crate) fn from_syn(
        syn_symbol_region: &SynSymbolRegion,
    ) -> (Self, SynSymbolMap<HirEagerVariableIdx>) {
        let mut arena = HirEagerVariableArena::default();
        let mut syn_symbol_to_hir_eager_variable_map =
            SynSymbolMap::<HirEagerVariableIdx>::new(syn_symbol_region);
        for (inherited_syn_symbol_idx, inherited_syn_symbol) in
            syn_symbol_region.indexed_inherited_syn_symbols()
        {
            if let Some(hir_eager_variable) =
                HirEagerVariable::from_inherited_syn(inherited_syn_symbol)
            {
                let hir_eager_variable_idx = arena.alloc_one(hir_eager_variable);
                syn_symbol_to_hir_eager_variable_map
                    .push_inherited(inherited_syn_symbol_idx, hir_eager_variable_idx)
            }
        }
        for (current_syn_symbol_idx, current_syn_symbol) in
            syn_symbol_region.indexed_current_syn_symbols()
        {
            if let Some(hir_eager_variable) = HirEagerVariable::from_current_syn(current_syn_symbol)
            {
                let hir_eager_variable_idx = arena.alloc_one(hir_eager_variable);
                syn_symbol_to_hir_eager_variable_map
                    .push_current(current_syn_symbol_idx, hir_eager_variable_idx)
            }
        }
        (Self { arena }, syn_symbol_to_hir_eager_variable_map)
    }
}
