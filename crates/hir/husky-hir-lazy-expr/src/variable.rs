use husky_syn_expr::variable::{
    CurrentVariableData, CurrentVariableEntry, InheritedVariableEntry, InheritedVariableKind,
    VariableMap, VariableRegionData,
};

use super::*;

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

    fn from_inherited_syn(inherited_variable: InheritedVariableEntry) -> Option<HirLazyVariable> {
        let name = match inherited_variable.kind() {
            InheritedVariableKind::Template(_)
            | InheritedVariableKind::Parenate { .. }
            | InheritedVariableKind::SelfField { .. } => {
                VariableName::Ident(inherited_variable.ident()?)
            }
        };
        let data = HirLazyVariableData::from_inherited_syn(inherited_variable.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_variable: &CurrentVariableEntry) -> Option<Self> {
        let name = match current_variable.data() {
            CurrentVariableData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => VariableName::SelfValue,
            _ => VariableName::Ident(current_variable.ident()?),
        };
        let data = HirLazyVariableData::from_current_syn(current_variable.data())?;
        Some(Self { name, data })
    }
}

impl HirLazyVariableData {
    fn from_inherited_syn(
        inherited_variable_kind: InheritedVariableKind,
    ) -> Option<HirLazyVariableData> {
        match inherited_variable_kind {
            InheritedVariableKind::Template(_) => None,
            InheritedVariableKind::Parenate { ident: _ } => {
                Some(HirLazyVariableData::ParenateParameter)
            }
            InheritedVariableKind::SelfField { ident: _ } => {
                Some(HirLazyVariableData::FieldVariable)
            }
        }
    }

    fn from_current_syn(current_variable_data: &CurrentVariableData) -> Option<Self> {
        match current_variable_data {
            CurrentVariableData::TemplateParameter { .. } => None,
            CurrentVariableData::SelfType => todo!(),
            CurrentVariableData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => todo!(),
            CurrentVariableData::SimpleParenateParameter {
                ident: _,
                pattern_variable_idx: _,
            } => Some(HirLazyVariableData::ParenateParameter),
            CurrentVariableData::VariadicParenateParameter {
                symbol_modifier_keyword_group: _,
                ident_token: _,
            } => Some(HirLazyVariableData::ParenateParameter),
            CurrentVariableData::LetVariable {
                ident: _,
                pattern_variable_idx: _,
            } => Some(HirLazyVariableData::LetVariable),
            CurrentVariableData::BeVariable {
                ident: _,
                pattern_variable_idx: _,
            } => Some(HirLazyVariableData::BeVariable),
            CurrentVariableData::CaseVariable {
                ident: _,
                pattern_variable_idx: _,
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
                pattern_variable_idx,
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
        variable_region: &VariableRegionData,
    ) -> (Self, VariableMap<HirLazyVariableIdx>) {
        let mut arena = HirLazyVariableArena::default();
        let mut variable_to_hir_eager_runtime_symbol_map =
            VariableMap::<HirLazyVariableIdx>::new(variable_region);
        for (inherited_variable_idx, inherited_variable) in
            variable_region.indexed_inherited_variables()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirLazyVariable::from_inherited_syn(inherited_variable)
            {
                let hir_eager_runtime_symbol_idx = arena.alloc_one(hir_eager_runtime_symbol);
                variable_to_hir_eager_runtime_symbol_map
                    .insert_new_inherited(inherited_variable_idx, hir_eager_runtime_symbol_idx)
            }
        }
        for (current_variable_idx, current_variable) in variable_region.indexed_current_variables()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirLazyVariable::from_current_syn(current_variable)
            {
                let hir_eager_runtime_symbol_idx = arena.alloc_one(hir_eager_runtime_symbol);
                variable_to_hir_eager_runtime_symbol_map
                    .insert_new_current(current_variable_idx, hir_eager_runtime_symbol_idx)
            }
        }
        (Self { arena }, variable_to_hir_eager_runtime_symbol_map)
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
