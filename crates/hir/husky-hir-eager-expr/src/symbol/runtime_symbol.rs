use super::*;
use husky_syn_expr::{
    AllowSelfValue, CurrentSynSymbol, CurrentSynSymbolData, InheritedSynSymbol,
    InheritedSynSymbolKind, SynSymbolMap, SynSymbolRegionData,
};
use idx_arena::ArenaIdx;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerRuntimeSymbolRegionData {
    arena: HirEagerRuntimeSymbolArena,
    self_value_variable: Option<HirEagerRuntimeSymbolIdx>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerRuntimeSymbolEntry {
    name: HirEagerRuntimeSymbolName,
    data: HirEagerRuntimeSymbolData,
}

pub type HirEagerRuntimeSymbolArena = Arena<HirEagerRuntimeSymbolEntry>;
pub type HirEagerRuntimeSymbolIdx = ArenaIdx<HirEagerRuntimeSymbolEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub enum HirEagerRuntimeSymbolName {
    SelfValue,
    Ident(Ident),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub enum HirEagerRuntimeSymbolData {
    SelfValue,
    ParenateParameter,
    LetVariable,
    BeVariable,
    CaseVariable,
    FieldVariable,
    LoopVariable,
}

impl HirEagerRuntimeSymbolRegionData {
    pub(crate) fn from_syn(
        syn_symbol_region: &SynSymbolRegionData,
    ) -> (Self, SynSymbolMap<HirEagerRuntimeSymbolIdx>) {
        let mut arena = HirEagerRuntimeSymbolArena::default();
        let self_value_variable = match syn_symbol_region.allow_self_value() {
            AllowSelfValue::True => Some(arena.alloc_one(HirEagerRuntimeSymbolEntry {
                name: HirEagerRuntimeSymbolName::SelfValue,
                data: HirEagerRuntimeSymbolData::SelfValue,
            })),
            AllowSelfValue::False => None,
        };
        let mut syn_symbol_to_hir_eager_runtime_symbol_map =
            SynSymbolMap::<HirEagerRuntimeSymbolIdx>::new(syn_symbol_region);

        for (inherited_syn_symbol_idx, inherited_syn_symbol) in
            syn_symbol_region.indexed_inherited_syn_symbols()
        {
            if let Some(hir_eager_runtime_symbol) =
                HirEagerRuntimeSymbolEntry::from_inherited_syn(inherited_syn_symbol)
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
                HirEagerRuntimeSymbolEntry::from_current_syn(current_syn_symbol)
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

    pub fn self_value_variable(&self) -> Option<HirEagerRuntimeSymbolIdx> {
        self.self_value_variable
    }
}

impl std::ops::Index<HirEagerRuntimeSymbolIdx> for HirEagerRuntimeSymbolRegionData {
    type Output = HirEagerRuntimeSymbolEntry;

    fn index(&self, index: HirEagerRuntimeSymbolIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl HirEagerRuntimeSymbolEntry {
    pub fn name(&self) -> HirEagerRuntimeSymbolName {
        self.name
    }

    pub fn data(&self) -> &HirEagerRuntimeSymbolData {
        &self.data
    }

    fn from_inherited_syn(
        inherited_syn_symbol: InheritedSynSymbol,
    ) -> Option<HirEagerRuntimeSymbolEntry> {
        let name = match inherited_syn_symbol.kind() {
            InheritedSynSymbolKind::TemplateParameter(_)
            | InheritedSynSymbolKind::ParenateParameter { .. }
            | InheritedSynSymbolKind::FieldVariable { .. } => {
                HirEagerRuntimeSymbolName::Ident(inherited_syn_symbol.ident()?)
            }
        };
        let data = HirEagerRuntimeSymbolData::from_inherited_syn(inherited_syn_symbol.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_syn_symbol: &CurrentSynSymbol) -> Option<Self> {
        let name = match current_syn_symbol.data() {
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group: _,
            } => HirEagerRuntimeSymbolName::SelfValue,
            _ => HirEagerRuntimeSymbolName::Ident(current_syn_symbol.ident()?),
        };
        let data = HirEagerRuntimeSymbolData::from_current_syn(current_syn_symbol.data())?;
        Some(Self { name, data })
    }
}

impl HirEagerRuntimeSymbolData {
    fn from_inherited_syn(
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
    ) -> Option<HirEagerRuntimeSymbolData> {
        match inherited_syn_symbol_kind {
            InheritedSynSymbolKind::TemplateParameter(_) => None,
            InheritedSynSymbolKind::ParenateParameter { ident: _ } => {
                Some(HirEagerRuntimeSymbolData::ParenateParameter)
            }
            InheritedSynSymbolKind::FieldVariable { ident: _ } => {
                Some(HirEagerRuntimeSymbolData::FieldVariable)
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
            } => Some(HirEagerRuntimeSymbolData::ParenateParameter),
            CurrentSynSymbolData::ParenateVariadicParameter {
                symbol_modifier_keyword_group: _,
                ident_token: _,
            } => Some(HirEagerRuntimeSymbolData::ParenateParameter),
            CurrentSynSymbolData::LetVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRuntimeSymbolData::LetVariable),
            CurrentSynSymbolData::BeVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRuntimeSymbolData::BeVariable),
            CurrentSynSymbolData::CaseVariable {
                ident: _,
                pattern_symbol_idx: _,
            } => Some(HirEagerRuntimeSymbolData::CaseVariable),
            CurrentSynSymbolData::FieldVariable { ident_token: _ } => {
                Some(HirEagerRuntimeSymbolData::FieldVariable)
            }
            CurrentSynSymbolData::LoopVariable {
                ident: _,
                expr_idx: _,
            } => Some(HirEagerRuntimeSymbolData::LoopVariable),
        }
    }
}
