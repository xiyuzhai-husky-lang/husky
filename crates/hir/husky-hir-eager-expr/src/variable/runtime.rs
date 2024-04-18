use super::*;
use husky_syn_expr::{
    AllowSelfValue, CurrentVariableData, CurrentVariableEntry, InheritedVariable,
    InheritedVariableKind, VariableMap, VariableRegionData,
};
use idx_arena::ArenaIdx;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRuntimeVariableRegionData {
    arena: HirEagerRvarArena,
    self_value_variable: Option<HirEagerRvarIdx>,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRuntimeVariableEntry {
    name: HirEagerRuntimeVariableName,
    data: HirEagerRuntimeVariableData,
}

pub type HirEagerRvarArena = Arena<HirEagerRuntimeVariableEntry>;
pub type HirEagerRvarIdx = ArenaIdx<HirEagerRuntimeVariableEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum HirEagerRuntimeVariableName {
    SelfValue,
    Ident(Ident),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerRuntimeVariableData {
    SelfValue,
    ParenateParameter,
    ClosureParameter,
    LetVariable,
    BeVariable,
    CaseVariable,
    FieldVariable,
    LoopVariable,
}

impl HirEagerRuntimeVariableRegionData {
    pub(crate) fn from_syn(
        syn_symbol_region: &VariableRegionData,
    ) -> (Self, VariableMap<HirEagerRvarIdx>) {
        let mut arena = HirEagerRvarArena::default();
        let self_value_variable = match syn_symbol_region.allow_self_value() {
            AllowSelfValue::True => Some(arena.alloc_one(HirEagerRuntimeVariableEntry {
                name: HirEagerRuntimeVariableName::SelfValue,
                data: HirEagerRuntimeVariableData::SelfValue,
            })),
            AllowSelfValue::False => None,
        };
        let mut syn_symbol_to_hir_eager_runtime_symbol_map =
            VariableMap::<HirEagerRvarIdx>::new(syn_symbol_region);

        for (inherited_syn_symbol_idx, inherited_syn_symbol) in
            syn_symbol_region.indexed_inherited_syn_symbols()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirEagerRuntimeVariableEntry::from_inherited_syn(inherited_syn_symbol)
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
                HirEagerRuntimeVariableEntry::from_current_syn(current_syn_symbol)
            {
                let hir_eager_runtime_symbol_idx = arena.alloc_one(hir_eager_runtime_symbol);
                syn_symbol_to_hir_eager_runtime_symbol_map
                    .push_current(current_syn_symbol_idx, hir_eager_runtime_symbol_idx)
            }
        }
        (
            Self {
                arena,
                self_value_variable,
            },
            syn_symbol_to_hir_eager_runtime_symbol_map,
        )
    }

    pub fn self_value_variable(&self) -> Option<HirEagerRvarIdx> {
        self.self_value_variable
    }
}

impl std::ops::Index<HirEagerRvarIdx> for HirEagerRuntimeVariableRegionData {
    type Output = HirEagerRuntimeVariableEntry;

    fn index(&self, index: HirEagerRvarIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl HirEagerRuntimeVariableEntry {
    pub fn name(&self) -> HirEagerRuntimeVariableName {
        self.name
    }

    pub fn data(&self) -> &HirEagerRuntimeVariableData {
        &self.data
    }

    fn from_inherited_syn(
        inherited_syn_symbol: InheritedVariable,
    ) -> Option<HirEagerRuntimeVariableEntry> {
        let name = match inherited_syn_symbol.kind() {
            InheritedVariableKind::Template(_)
            | InheritedVariableKind::Parenate { .. }
            | InheritedVariableKind::SelfField { .. } => {
                HirEagerRuntimeVariableName::Ident(inherited_syn_symbol.ident()?)
            }
        };
        let data = HirEagerRuntimeVariableData::from_inherited_syn(inherited_syn_symbol.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_syn_symbol: &CurrentVariableEntry) -> Option<Self> {
        let name = match current_syn_symbol.data() {
            CurrentVariableData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => HirEagerRuntimeVariableName::SelfValue,
            _ => HirEagerRuntimeVariableName::Ident(current_syn_symbol.ident()?),
        };
        let data = HirEagerRuntimeVariableData::from_current_syn(current_syn_symbol.data())?;
        Some(Self { name, data })
    }
}

impl HirEagerRuntimeVariableData {
    fn from_inherited_syn(
        inherited_syn_symbol_kind: InheritedVariableKind,
    ) -> Option<HirEagerRuntimeVariableData> {
        match inherited_syn_symbol_kind {
            InheritedVariableKind::Template(_) => None,
            InheritedVariableKind::Parenate { ident: _ } => {
                Some(HirEagerRuntimeVariableData::ParenateParameter)
            }
            InheritedVariableKind::SelfField { ident: _ } => {
                Some(HirEagerRuntimeVariableData::FieldVariable)
            }
        }
    }

    fn from_current_syn(current_syn_symbol_data: &CurrentVariableData) -> Option<Self> {
        match current_syn_symbol_data {
            CurrentVariableData::TemplateParameter { .. } => None,
            CurrentVariableData::SelfType => todo!(),
            CurrentVariableData::SelfValue { .. } => todo!(),
            CurrentVariableData::SimpleParenateParameter { .. } => {
                Some(HirEagerRuntimeVariableData::ParenateParameter)
            }
            CurrentVariableData::VariadicParenateParameter { .. } => {
                Some(HirEagerRuntimeVariableData::ParenateParameter)
            }
            CurrentVariableData::SimpleClosureParameter { .. } => {
                Some(HirEagerRuntimeVariableData::ClosureParameter)
            }
            CurrentVariableData::LetVariable { .. } => {
                Some(HirEagerRuntimeVariableData::LetVariable)
            }
            CurrentVariableData::BeVariable { .. } => Some(HirEagerRuntimeVariableData::BeVariable),
            CurrentVariableData::CaseVariable { .. } => {
                Some(HirEagerRuntimeVariableData::CaseVariable)
            }
            CurrentVariableData::FieldVariable { .. } => {
                Some(HirEagerRuntimeVariableData::FieldVariable)
            }
            CurrentVariableData::LoopVariable { .. } => {
                Some(HirEagerRuntimeVariableData::LoopVariable)
            }
        }
    }
}
