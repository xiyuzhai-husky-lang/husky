use super::*;
use husky_syn_expr::variable::{
    AllowSelfValue, CurrentVariableData, CurrentVariableEntry, CurrentVariableIdx,
    InheritedVariableEntry, InheritedVariableKind, VariableMap, VariableRegionData,
};
use idx_arena::ArenaIdx;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRuntimeVariableRegionData {
    arena: HirEagerRuntimeVariableArena,
    self_value_variable: Option<HirEagerRuntimeVariableIdx>,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRuntimeVariableEntry {
    name: HirEagerRuntimeVariableName,
    data: HirEagerRuntimeVariableData,
}

pub type HirEagerRuntimeVariableArena = Arena<HirEagerRuntimeVariableEntry>;
pub type HirEagerRuntimeVariableIdx = ArenaIdx<HirEagerRuntimeVariableEntry>;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        variable_region: &VariableRegionData,
    ) -> (Self, VariableMap<HirEagerRuntimeVariableIdx>) {
        let mut arena = HirEagerRuntimeVariableArena::default();
        let self_value_variable = match variable_region.allow_self_value() {
            AllowSelfValue::True => Some(arena.alloc_one(HirEagerRuntimeVariableEntry {
                name: HirEagerRuntimeVariableName::SelfValue,
                data: HirEagerRuntimeVariableData::SelfValue,
            })),
            AllowSelfValue::False => None,
        };
        let mut variable_to_hir_eager_runtime_symbol_map =
            VariableMap::<HirEagerRuntimeVariableIdx>::new(variable_region);

        for (inherited_variable_idx, inherited_variable) in
            variable_region.indexed_inherited_variables()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirEagerRuntimeVariableEntry::from_inherited_syn(inherited_variable)
            {
                let hir_eager_runtime_symbol_idx = arena.alloc_one(hir_eager_runtime_symbol);
                variable_to_hir_eager_runtime_symbol_map
                    .insert_new_inherited(inherited_variable_idx, hir_eager_runtime_symbol_idx)
            }
        }
        for (current_variable_idx, current_variable) in variable_region.indexed_current_variables()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirEagerRuntimeVariableEntry::from_current_syn(current_variable)
            {
                let hir_eager_runtime_symbol_idx = arena.alloc_one(hir_eager_runtime_symbol);
                variable_to_hir_eager_runtime_symbol_map
                    .insert_new_current(current_variable_idx, hir_eager_runtime_symbol_idx)
            }
        }
        (
            Self {
                arena,
                self_value_variable,
            },
            variable_to_hir_eager_runtime_symbol_map,
        )
    }

    pub fn self_value_variable(&self) -> Option<HirEagerRuntimeVariableIdx> {
        self.self_value_variable
    }

    pub fn arena(&self) -> &HirEagerRuntimeVariableArena {
        &self.arena
    }
}

impl std::ops::Index<HirEagerRuntimeVariableIdx> for HirEagerRuntimeVariableRegionData {
    type Output = HirEagerRuntimeVariableEntry;

    fn index(&self, index: HirEagerRuntimeVariableIdx) -> &Self::Output {
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
        inherited_variable: InheritedVariableEntry,
    ) -> Option<HirEagerRuntimeVariableEntry> {
        let name = match inherited_variable.kind() {
            InheritedVariableKind::Template(_)
            | InheritedVariableKind::Parenate { .. }
            | InheritedVariableKind::SelfField { .. } => {
                HirEagerRuntimeVariableName::Ident(inherited_variable.ident()?)
            }
            InheritedVariableKind::ReplLocal => todo!(),
        };
        let data = HirEagerRuntimeVariableData::from_inherited_syn(inherited_variable.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_variable: &CurrentVariableEntry) -> Option<Self> {
        let name = match current_variable.data() {
            CurrentVariableData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => HirEagerRuntimeVariableName::SelfValue,
            _ => HirEagerRuntimeVariableName::Ident(current_variable.ident()?),
        };
        let data = HirEagerRuntimeVariableData::from_current_syn(current_variable.data())?;
        Some(Self { name, data })
    }
}

impl HirEagerRuntimeVariableData {
    fn from_inherited_syn(
        inherited_variable_kind: InheritedVariableKind,
    ) -> Option<HirEagerRuntimeVariableData> {
        match inherited_variable_kind {
            InheritedVariableKind::Template(_) => None,
            InheritedVariableKind::Parenate { ident: _ } => {
                Some(HirEagerRuntimeVariableData::ParenateParameter)
            }
            InheritedVariableKind::SelfField { ident: _ } => {
                Some(HirEagerRuntimeVariableData::FieldVariable)
            }
            InheritedVariableKind::ReplLocal => todo!(),
        }
    }

    fn from_current_syn(current_variable_data: &CurrentVariableData) -> Option<Self> {
        match current_variable_data {
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

impl ToHirEager for CurrentVariableIdx {
    type Output = HirEagerRuntimeVariableIdx;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        builder
            .current_variable_to_hir_eager_runtime_symbol(*self)
            .unwrap()
    }
}
