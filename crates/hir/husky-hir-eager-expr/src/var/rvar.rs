use super::*;
use husky_syn_expr::{
    AllowSelfValue, CurrentSynSymbol, CurrentSynSymbolData, InheritedSynSymbol,
    InheritedSynSymbolKind, SynSymbolMap, VariableRegionData,
};
use idx_arena::ArenaIdx;

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRuntimeSvarRegionData {
    arena: HirEagerRvarArena,
    self_value_variable: Option<HirEagerRvarIdx>,
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRuntimeSvarEntry {
    name: HirEagerRuntimeSvarName,
    data: HirEagerRuntimeSvarData,
}

pub type HirEagerRvarArena = Arena<HirEagerRuntimeSvarEntry>;
pub type HirEagerRvarIdx = ArenaIdx<HirEagerRuntimeSvarEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum HirEagerRuntimeSvarName {
    SelfValue,
    Ident(Ident),
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerRuntimeSvarData {
    SelfValue,
    ParenateParameter,
    LetVariable,
    BeVariable,
    CaseVariable,
    FieldVariable,
    LoopVariable,
}

impl HirEagerRuntimeSvarRegionData {
    pub(crate) fn from_syn(
        syn_symbol_region: &VariableRegionData,
    ) -> (Self, SynSymbolMap<HirEagerRvarIdx>) {
        let mut arena = HirEagerRvarArena::default();
        let self_value_variable = match syn_symbol_region.allow_self_value() {
            AllowSelfValue::True => Some(arena.alloc_one(HirEagerRuntimeSvarEntry {
                name: HirEagerRuntimeSvarName::SelfValue,
                data: HirEagerRuntimeSvarData::SelfValue,
            })),
            AllowSelfValue::False => None,
        };
        let mut syn_symbol_to_hir_eager_runtime_symbol_map =
            SynSymbolMap::<HirEagerRvarIdx>::new(syn_symbol_region);

        for (inherited_syn_symbol_idx, inherited_syn_symbol) in
            syn_symbol_region.indexed_inherited_syn_symbols()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirEagerRuntimeSvarEntry::from_inherited_syn(inherited_syn_symbol)
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
                HirEagerRuntimeSvarEntry::from_current_syn(current_syn_symbol)
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

impl std::ops::Index<HirEagerRvarIdx> for HirEagerRuntimeSvarRegionData {
    type Output = HirEagerRuntimeSvarEntry;

    fn index(&self, index: HirEagerRvarIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl HirEagerRuntimeSvarEntry {
    pub fn name(&self) -> HirEagerRuntimeSvarName {
        self.name
    }

    pub fn data(&self) -> &HirEagerRuntimeSvarData {
        &self.data
    }

    fn from_inherited_syn(
        inherited_syn_symbol: InheritedSynSymbol,
    ) -> Option<HirEagerRuntimeSvarEntry> {
        let name = match inherited_syn_symbol.kind() {
            InheritedSynSymbolKind::TemplateParameter(_)
            | InheritedSynSymbolKind::ParenateParameter { .. }
            | InheritedSynSymbolKind::FieldVariable { .. } => {
                HirEagerRuntimeSvarName::Ident(inherited_syn_symbol.ident()?)
            }
        };
        let data = HirEagerRuntimeSvarData::from_inherited_syn(inherited_syn_symbol.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_syn_symbol: &CurrentSynSymbol) -> Option<Self> {
        let name = match current_syn_symbol.data() {
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => HirEagerRuntimeSvarName::SelfValue,
            _ => HirEagerRuntimeSvarName::Ident(current_syn_symbol.ident()?),
        };
        let data = HirEagerRuntimeSvarData::from_current_syn(current_syn_symbol.data())?;
        Some(Self { name, data })
    }
}

impl HirEagerRuntimeSvarData {
    fn from_inherited_syn(
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
    ) -> Option<HirEagerRuntimeSvarData> {
        match inherited_syn_symbol_kind {
            InheritedSynSymbolKind::TemplateParameter(_) => None,
            InheritedSynSymbolKind::ParenateParameter { ident: _ } => {
                Some(HirEagerRuntimeSvarData::ParenateParameter)
            }
            InheritedSynSymbolKind::FieldVariable { ident: _ } => {
                Some(HirEagerRuntimeSvarData::FieldVariable)
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
            CurrentSynSymbolData::SimpleParenateParameter {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRuntimeSvarData::ParenateParameter),
            CurrentSynSymbolData::VariadicParenateParameter {
                symbol_modifier_keyword_group: _,
                ident_token: _,
            } => Some(HirEagerRuntimeSvarData::ParenateParameter),
            CurrentSynSymbolData::LetVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRuntimeSvarData::LetVariable),
            CurrentSynSymbolData::BeVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRuntimeSvarData::BeVariable),
            CurrentSynSymbolData::CaseVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRuntimeSvarData::CaseVariable),
            CurrentSynSymbolData::FieldVariable { ident_token: _ } => {
                Some(HirEagerRuntimeSvarData::FieldVariable)
            }
            CurrentSynSymbolData::LoopVariable {
                ident: _,
                expr_idx: _,
            } => Some(HirEagerRuntimeSvarData::LoopVariable),
            CurrentSynSymbolData::SimpleLambdaParameter {
                ident,
                pattern_symbol_idx,
            } => todo!(),
        }
    }
}
