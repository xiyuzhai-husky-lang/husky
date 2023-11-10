use super::*;
use husky_syn_expr::{
    AllowSelfValue, CurrentSynSymbol, CurrentSynSymbolData, CurrentSynSymbolKind,
    CurrentTemplateParameterSynSymbolVariant, InheritedSynSymbol, InheritedSynSymbolKind,
    SynSymbolMap, SynSymbolOrderedMap, SynSymbolRegion,
};
use idx_arena::ArenaIdx;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerVariable {
    name: VariableName,
    data: HirEagerVariableData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub enum VariableName {
    SelfValue,
    Ident(Ident),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerVariableData {
    SelfValue,
    ParenateParameter,
    LetVariable,
    BeVariable,
    CaseVariable,
    FieldVariable,
    LoopVariable,
}

impl HirEagerVariable {
    pub fn name(&self) -> VariableName {
        self.name
    }

    pub fn data(&self) -> &HirEagerVariableData {
        &self.data
    }

    fn from_inherited_syn(inherited_syn_symbol: InheritedSynSymbol) -> Option<HirEagerVariable> {
        let name = match inherited_syn_symbol.kind() {
            InheritedSynSymbolKind::TemplateParameter(_)
            | InheritedSynSymbolKind::ParenateParameter { .. }
            | InheritedSynSymbolKind::FieldVariable { .. } => {
                VariableName::Ident(inherited_syn_symbol.ident()?)
            }
        };
        let data = HirEagerVariableData::from_inherited_syn(inherited_syn_symbol.kind())?;
        Some(Self { name, data })
    }

    fn from_current_syn(current_syn_symbol: &CurrentSynSymbol) -> Option<Self> {
        let name = match current_syn_symbol.data() {
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group,
            } => VariableName::SelfValue,
            _ => VariableName::Ident(current_syn_symbol.ident()?),
        };
        let data = HirEagerVariableData::from_current_syn(current_syn_symbol.data())?;
        Some(Self { name, data })
    }
}

impl HirEagerVariableData {
    fn from_inherited_syn(
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
    ) -> Option<HirEagerVariableData> {
        match inherited_syn_symbol_kind {
            InheritedSynSymbolKind::TemplateParameter(_) => None,
            InheritedSynSymbolKind::ParenateParameter { ident } => {
                Some(HirEagerVariableData::ParenateParameter)
            }
            InheritedSynSymbolKind::FieldVariable { ident } => {
                Some(HirEagerVariableData::FieldVariable)
            }
        }
    }

    fn from_current_syn(current_syn_symbol_data: &CurrentSynSymbolData) -> Option<Self> {
        match current_syn_symbol_data {
            CurrentSynSymbolData::TemplateParameter { .. } => None,
            CurrentSynSymbolData::SelfType => todo!(),
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            CurrentSynSymbolData::ParenateRegularParameter {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariableData::ParenateParameter),
            CurrentSynSymbolData::ParenateVariadicParameter {
                symbol_modifier_keyword_group,
                ident_token,
            } => Some(HirEagerVariableData::ParenateParameter),
            CurrentSynSymbolData::LetVariable {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariableData::LetVariable),
            CurrentSynSymbolData::BeVariable {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariableData::BeVariable),
            CurrentSynSymbolData::CaseVariable {
                ident,
                pattern_symbol_idx,
            } => Some(HirEagerVariableData::CaseVariable),
            CurrentSynSymbolData::FieldVariable { ident_token } => {
                Some(HirEagerVariableData::FieldVariable)
            }
            CurrentSynSymbolData::LoopVariable { ident, expr_idx } => {
                Some(HirEagerVariableData::LoopVariable)
            }
        }
    }
}

pub type HirEagerVariableArena = Arena<HirEagerVariable>;
pub type HirEagerVariableIdx = ArenaIdx<HirEagerVariable>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerVariableRegion {
    arena: HirEagerVariableArena,
    self_value_variable: Option<HirEagerVariableIdx>,
}

impl HirEagerVariableRegion {
    pub(crate) fn from_syn(
        syn_symbol_region: &SynSymbolRegion,
    ) -> (Self, SynSymbolMap<HirEagerVariableIdx>) {
        let mut arena = HirEagerVariableArena::default();
        let self_value_variable = match syn_symbol_region.allow_self_value() {
            AllowSelfValue::True => Some(arena.alloc_one(HirEagerVariable {
                name: VariableName::SelfValue,
                data: HirEagerVariableData::SelfValue,
            })),
            AllowSelfValue::False => None,
        };
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
        (
            Self {
                arena,
                self_value_variable,
            },
            syn_symbol_to_hir_eager_variable_map,
        )
    }

    pub fn self_value_variable(&self) -> Option<HirEagerVariableIdx> {
        self.self_value_variable
    }
}

impl std::ops::Index<HirEagerVariableIdx> for HirEagerVariableRegion {
    type Output = HirEagerVariable;

    fn index(&self, index: HirEagerVariableIdx) -> &Self::Output {
        &self.arena[index]
    }
}
