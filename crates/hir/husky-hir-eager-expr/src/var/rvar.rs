use super::*;
use husky_syn_expr::{
    AllowSelfValue, CurrentSynSymbol, CurrentSynSymbolData, InheritedSynSymbol,
    InheritedSynSymbolKind, SynSymbolMap, SynSymbolRegionData,
};
use idx_arena::ArenaIdx;

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRvarRegionData {
    arena: HirEagerRvarArena,
    self_value_variable: Option<HirEagerRvarIdx>,
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerRvarEntry {
    name: HirEagerRvarName,
    data: HirEagerRvarData,
}

pub type HirEagerRvarArena = Arena<HirEagerRvarEntry>;
pub type HirEagerRvarIdx = ArenaIdx<HirEagerRvarEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum HirEagerRvarName {
    SelfValue,
    Ident(Ident),
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerRvarData {
    SelfValue,
    ParenateParameter,
    LetVariable,
    BeVariable,
    CaseVariable,
    FieldVariable,
    LoopVariable,
}

impl HirEagerRvarRegionData {
    pub(crate) fn from_syn(
        syn_symbol_region: &SynSymbolRegionData,
    ) -> (Self, SynSymbolMap<HirEagerRvarIdx>) {
        let mut arena = HirEagerRvarArena::default();
        let self_value_variable = match syn_symbol_region.allow_self_value() {
            AllowSelfValue::True => Some(arena.alloc_one(HirEagerRvarEntry {
                name: HirEagerRvarName::SelfValue,
                data: HirEagerRvarData::SelfValue,
            })),
            AllowSelfValue::False => None,
        };
        let mut syn_symbol_to_hir_eager_runtime_symbol_map =
            SynSymbolMap::<HirEagerRvarIdx>::new(syn_symbol_region);

        for (inherited_syn_symbol_idx, inherited_syn_symbol) in
            syn_symbol_region.indexed_inherited_syn_symbols()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirEagerRvarEntry::from_inherited_syn(inherited_syn_symbol)
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
                HirEagerRvarEntry::from_current_syn(current_syn_symbol)
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

impl std::ops::Index<HirEagerRvarIdx> for HirEagerRvarRegionData {
    type Output = HirEagerRvarEntry;

    fn index(&self, index: HirEagerRvarIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl HirEagerRvarEntry {
    pub fn name(&self) -> HirEagerRvarName {
        self.name
    }

    pub fn data(&self) -> &HirEagerRvarData {
        &self.data
    }

    fn from_inherited_syn(inherited_syn_symbol: InheritedSynSymbol) -> Option<HirEagerRvarEntry> {
        let name = match inherited_syn_symbol.kind() {
            InheritedSynSymbolKind::TemplateParameter(_)
            | InheritedSynSymbolKind::ParenateParameter { .. }
            | InheritedSynSymbolKind::FieldVariable { .. } => {
                HirEagerRvarName::Ident(inherited_syn_symbol.ident()?)
            }
        };
        let data = HirEagerRvarData::from_inherited_syn(inherited_syn_symbol.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_syn_symbol: &CurrentSynSymbol) -> Option<Self> {
        let name = match current_syn_symbol.data() {
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => HirEagerRvarName::SelfValue,
            _ => HirEagerRvarName::Ident(current_syn_symbol.ident()?),
        };
        let data = HirEagerRvarData::from_current_syn(current_syn_symbol.data())?;
        Some(Self { name, data })
    }
}

impl HirEagerRvarData {
    fn from_inherited_syn(
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
    ) -> Option<HirEagerRvarData> {
        match inherited_syn_symbol_kind {
            InheritedSynSymbolKind::TemplateParameter(_) => None,
            InheritedSynSymbolKind::ParenateParameter { ident: _ } => {
                Some(HirEagerRvarData::ParenateParameter)
            }
            InheritedSynSymbolKind::FieldVariable { ident: _ } => {
                Some(HirEagerRvarData::FieldVariable)
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
            } => Some(HirEagerRvarData::ParenateParameter),
            CurrentSynSymbolData::ParenateVariadicParameter {
                symbol_modifier_keyword_group: _,
                ident_token: _,
            } => Some(HirEagerRvarData::ParenateParameter),
            CurrentSynSymbolData::LetVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRvarData::LetVariable),
            CurrentSynSymbolData::BeVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRvarData::BeVariable),
            CurrentSynSymbolData::CaseVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRvarData::CaseVariable),
            CurrentSynSymbolData::FieldVariable { ident_token: _ } => {
                Some(HirEagerRvarData::FieldVariable)
            }
            CurrentSynSymbolData::LoopVariable {
                ident: _,
                expr_idx: _,
            } => Some(HirEagerRvarData::LoopVariable),
        }
    }
}
