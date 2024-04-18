use super::*;
use husky_syn_expr::{
    CurrentVariableData, CurrentVariableEntry, InheritedVariable, InheritedVariableKind,
    VariableMap, VariableRegionData,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirLazyVariable {
    name: VariableName,
    data: HirLazyVariableData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
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

    fn from_inherited_syn(inherited_syn_symbol: InheritedVariable) -> Option<HirLazyVariable> {
        let name = match inherited_syn_symbol.kind() {
            InheritedVariableKind::Template(_)
            | InheritedVariableKind::Parenate { .. }
            | InheritedVariableKind::SelfField { .. } => {
                VariableName::Ident(inherited_syn_symbol.ident()?)
            }
        };
        let data = HirLazyVariableData::from_inherited_syn(inherited_syn_symbol.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_syn_symbol: &CurrentVariableEntry) -> Option<Self> {
        let name = match current_syn_symbol.data() {
            CurrentVariableData::SelfValue {
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
        inherited_syn_symbol_kind: InheritedVariableKind,
    ) -> Option<HirLazyVariableData> {
        match inherited_syn_symbol_kind {
            InheritedVariableKind::Template(_) => None,
            InheritedVariableKind::Parenate { ident: _ } => {
                Some(HirLazyVariableData::ParenateParameter)
            }
            InheritedVariableKind::SelfField { ident: _ } => {
                Some(HirLazyVariableData::FieldVariable)
            }
        }
    }

    fn from_current_syn(current_syn_symbol_data: &CurrentVariableData) -> Option<Self> {
        match current_syn_symbol_data {
            CurrentVariableData::TemplateParameter { .. } => None,
            CurrentVariableData::SelfType => todo!(),
            CurrentVariableData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => todo!(),
            CurrentVariableData::SimpleParenateParameter {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::ParenateParameter),
            CurrentVariableData::VariadicParenateParameter {
                symbol_modifier_keyword_group: _,
                ident_token: _,
            } => Some(HirLazyVariableData::ParenateParameter),
            CurrentVariableData::LetVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::LetVariable),
            CurrentVariableData::BeVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::BeVariable),
            CurrentVariableData::CaseVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirLazyVariableData::CaseVariable),
            CurrentVariableData::FieldVariable { ident_token: _ } => {
                Some(HirLazyVariableData::FieldVariable)
            }
            CurrentVariableData::LoopVariable {
                ident: _,
                expr_idx: _,
            } => Some(HirLazyVariableData::LoopVariable),
            CurrentVariableData::SimpleClosureParameter {
                ident,
                pattern_symbol_idx,
            } => todo!(),
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
        syn_symbol_region: &VariableRegionData,
    ) -> (Self, VariableMap<HirLazyVariableIdx>) {
        let mut arena = HirLazyVariableArena::default();
        let mut syn_symbol_to_hir_eager_runtime_symbol_map =
            VariableMap::<HirLazyVariableIdx>::new(syn_symbol_region);
        for (inherited_syn_symbol_idx, inherited_syn_symbol) in
            syn_symbol_region.indexed_inherited_syn_symbols()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirLazyVariable::from_inherited_syn(inherited_syn_symbol)
            {
                let hir_eager_runtime_symbol_idx = arena.alloc_one(hir_eager_runtime_symbol);
                syn_symbol_to_hir_eager_runtime_symbol_map
                    .push_inherited(inherited_syn_symbol_idx, hir_eager_runtime_symbol_idx)
            }
        }
        for (current_syn_symbol_idx, current_syn_symbol) in
            syn_symbol_region.indexed_current_syn_symbols()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirLazyVariable::from_current_syn(current_syn_symbol)
            {
                let hir_eager_runtime_symbol_idx = arena.alloc_one(hir_eager_runtime_symbol);
                syn_symbol_to_hir_eager_runtime_symbol_map
                    .push_current(current_syn_symbol_idx, hir_eager_runtime_symbol_idx)
            }
        }
        (Self { arena }, syn_symbol_to_hir_eager_runtime_symbol_map)
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
