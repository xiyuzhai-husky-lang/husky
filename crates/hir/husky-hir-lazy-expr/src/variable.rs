use super::*;
use husky_syn_expr::{
    CurrentSynSymbol, CurrentSynSymbolData, InheritedSynSymbol, InheritedSynSymbolKind,
    SynSymbolMap, SynSymbolRegion,
};
use idx_arena::ArenaIdx;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirLazyVariable {
    name: VariableName,
    data: HirLazyVariableData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirLazyExprDb)]
pub enum VariableName {
    SelfValue,
    Ident(Ident),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirLazyVariableData {
    ParenateParameter,
    LetVariable,
    BeVariable,
    CaseVariable,
    FieldVariable,
    LoopVariable,
}

impl HirLazyVariable {
    pub fn name(&self) -> VariableName {
        self.name
    }

    pub fn data(&self) -> &HirLazyVariableData {
        &self.data
    }

    fn from_inherited_syn(inherited_syn_symbol: InheritedSynSymbol) -> Option<HirLazyVariable> {
        let name = match inherited_syn_symbol.kind() {
            InheritedSynSymbolKind::TemplateParameter(_)
            | InheritedSynSymbolKind::ParenateParameter { .. }
            | InheritedSynSymbolKind::FieldVariable { .. } => {
                VariableName::Ident(inherited_syn_symbol.ident()?)
            }
        };
        let data = HirLazyVariableData::from_inherited_syn(inherited_syn_symbol.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_syn_symbol: &CurrentSynSymbol) -> Option<Self> {
        let name = match current_syn_symbol.data() {
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => VariableName::SelfValue,
            _ => VariableName::Ident(current_syn_symbol.ident()?),
        };
        let data = HirLazyVariableData::from_current_syn(current_syn_symbol.data())?;
        Some(Self { name, data })
    }
}

impl HirLazyVariableData {
    fn from_inherited_syn(
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
    ) -> Option<HirLazyVariableData> {
        match inherited_syn_symbol_kind {
            InheritedSynSymbolKind::TemplateParameter(_) => None,
            InheritedSynSymbolKind::ParenateParameter { ident: _ } => {
                Some(HirLazyVariableData::ParenateParameter)
            }
            InheritedSynSymbolKind::FieldVariable { ident: _ } => {
                Some(HirLazyVariableData::FieldVariable)
            }
        }
    }

    fn from_current_syn(current_syn_symbol_data: &CurrentSynSymbolData) -> Option<Self> {
        match current_syn_symbol_data {
            CurrentSynSymbolData::TemplateParameter { .. } => None,
            CurrentSynSymbolData::SelfType => todo!(),
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => todo!(),
            CurrentSynSymbolData::ParenateRegularParameter {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::ParenateParameter),
            CurrentSynSymbolData::ParenateVariadicParameter {
                symbol_modifier_keyword_group: _,
                ident_token: _,
            } => Some(HirLazyVariableData::ParenateParameter),
            CurrentSynSymbolData::LetVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::LetVariable),
            CurrentSynSymbolData::BeVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::BeVariable),
            CurrentSynSymbolData::CaseVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::CaseVariable),
            CurrentSynSymbolData::FieldVariable { ident_token: _ } => {
                Some(HirLazyVariableData::FieldVariable)
            }
            CurrentSynSymbolData::LoopVariable { ident: _, expr_idx: _ } => {
                Some(HirLazyVariableData::LoopVariable)
            }
        }
    }
}

pub type HirLazyVariableArena = Arena<HirLazyVariable>;
pub type HirLazyVariableIdx = ArenaIdx<HirLazyVariable>;
pub type HirLazyVariableMap<V> = ArenaMap<HirLazyVariable, V>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirLazyVariableRegion {
    arena: HirLazyVariableArena,
}

impl HirLazyVariableRegion {
    pub(crate) fn from_syn(
        syn_symbol_region: &SynSymbolRegion,
    ) -> (Self, SynSymbolMap<HirLazyVariableIdx>) {
        let mut arena = HirLazyVariableArena::default();
        let mut syn_symbol_to_hir_eager_variable_map =
            SynSymbolMap::<HirLazyVariableIdx>::new(syn_symbol_region);
        for (inherited_syn_symbol_idx, inherited_syn_symbol) in
            syn_symbol_region.indexed_inherited_syn_symbols()
        {
            if let Some(hir_eager_variable) =
                HirLazyVariable::from_inherited_syn(inherited_syn_symbol)
            {
                let hir_eager_variable_idx = arena.alloc_one(hir_eager_variable);
                syn_symbol_to_hir_eager_variable_map
                    .push_inherited(inherited_syn_symbol_idx, hir_eager_variable_idx)
            }
        }
        for (current_syn_symbol_idx, current_syn_symbol) in
            syn_symbol_region.indexed_current_syn_symbols()
        {
            if let Some(hir_eager_variable) = HirLazyVariable::from_current_syn(current_syn_symbol)
            {
                let hir_eager_variable_idx = arena.alloc_one(hir_eager_variable);
                syn_symbol_to_hir_eager_variable_map
                    .push_current(current_syn_symbol_idx, hir_eager_variable_idx)
            }
        }
        (Self { arena }, syn_symbol_to_hir_eager_variable_map)
    }

    pub fn arena(&self) -> &HirLazyVariableArena {
        &self.arena
    }
}

impl std::ops::Index<HirLazyVariableIdx> for HirLazyVariableRegion {
    type Output = HirLazyVariable;

    fn index(&self, index: HirLazyVariableIdx) -> &Self::Output {
        &self.arena[index]
    }
}
