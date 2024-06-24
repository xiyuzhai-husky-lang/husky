use super::*;
use husky_coword::Ident;
use husky_eth_term::term::EthTerm;
use husky_fly_term::FlyTermBase;
use husky_hir_ty::HirTemplateVariable;
use husky_sem_expr::SemExprRegionData;
use husky_syn_expr::variable::{
    CurrentTemplateVariableData, CurrentVariableData, VariableRegionData,
};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerComptimeVariableRegionData {
    arena: HirEagerComptimeVariableArena,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerComptimeVariableEntry {
    name: HirEagerComptimeVariableName,
    data: HirEagerComptimeVariableData,
    hir_comptime_symbol: HirTemplateVariable,
}

pub type HirEagerComptimeVariableArena = Arena<HirEagerComptimeVariableEntry>;
pub type HirEagerComptimeVariableIdx = ArenaIdx<HirEagerComptimeVariableEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum HirEagerComptimeVariableName {
    SelfType,
    Ident(Ident),
    Label(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerComptimeVariableData {
    Inherited,
    Current,
}

impl HirEagerComptimeVariableEntry {
    pub fn name(&self) -> HirEagerComptimeVariableName {
        self.name
    }

    pub fn data(&self) -> &HirEagerComptimeVariableData {
        &self.data
    }

    pub fn symbol(&self) -> HirTemplateVariable {
        self.hir_comptime_symbol
    }
}

impl HirEagerComptimeVariableRegionData {
    pub(crate) fn from_sema(
        sem_expr_region_data: &SemExprRegionData,
        syn_symobl_region_data: &VariableRegionData,
        db: &::salsa::Db,
    ) -> Self {
        let mut arena = HirEagerComptimeVariableArena::default();
        let terms = sem_expr_region_data.fly_term_region().terms();
        for (inherited_variable_idx, &fly_term) in sem_expr_region_data
            .symbol_terms()
            .inherited_variable_key_values()
        {
            let FlyTermBase::Eth(term) = fly_term.base_resolved_inner(terms) else {
                unreachable!()
            };
            match term {
                EthTerm::SymbolicVariable(term_symbol) => {
                    let Some(hir_comptime_symbol) = HirTemplateVariable::from_eth(term_symbol, db)
                    else {
                        continue;
                    };
                    let name = match syn_symobl_region_data[inherited_variable_idx].ident() {
                        Some(ident) => HirEagerComptimeVariableName::Ident(ident),
                        None => todo!(),
                    };
                    arena.alloc_one(HirEagerComptimeVariableEntry {
                        name,
                        data: HirEagerComptimeVariableData::Inherited,
                        hir_comptime_symbol,
                    });
                }
                _ => todo!(),
            }
        }
        for (current_variable_idx, &fly_term) in sem_expr_region_data
            .symbol_terms()
            .current_variable_key_values()
        {
            let FlyTermBase::Eth(term) = fly_term.base_resolved_inner(terms) else {
                unreachable!()
            };
            match term {
                EthTerm::SymbolicVariable(term_symbol) => {
                    let Some(hir_comptime_symbol) = HirTemplateVariable::from_eth(term_symbol, db)
                    else {
                        continue;
                    };
                    let current_variable = &syn_symobl_region_data[current_variable_idx];
                    let name = match current_variable.ident() {
                        Some(ident) => HirEagerComptimeVariableName::Ident(ident),
                        None => match current_variable.data() {
                            CurrentVariableData::TemplateParameter {
                                syn_attrs: _,
                                annotated_variance_token: _,
                                data: template_parameter_variant,
                            } => match template_parameter_variant {
                                CurrentTemplateVariableData::Lifetime { label_token } => {
                                    HirEagerComptimeVariableName::Label(label_token.label())
                                }
                                CurrentTemplateVariableData::Place { label_token } => {
                                    HirEagerComptimeVariableName::Label(label_token.label())
                                }
                                CurrentTemplateVariableData::Type { .. } => {
                                    todo!()
                                }
                                CurrentTemplateVariableData::Constant {
                                    ident_token: _,
                                    ty_expr_idx: _,
                                } => todo!(),
                                _ => todo!(),
                            },
                            CurrentVariableData::SelfType => todo!(),
                            CurrentVariableData::SelfValue {
                                symbol_modifier_keyword_group: _,
                            } => todo!(),
                            CurrentVariableData::SimpleParenateParameter {
                                ident: _,
                                pattern_variable_idx: _,
                            } => todo!(),
                            CurrentVariableData::VariadicParenateParameter {
                                symbol_modifier_keyword_group: _,
                                ident_token: _,
                            } => todo!(),
                            CurrentVariableData::LetVariable {
                                ident: _,
                                pattern_variable_idx: _,
                            } => todo!(),
                            CurrentVariableData::BeVariable {
                                ident: _,
                                pattern_variable_idx: _,
                            } => todo!(),
                            CurrentVariableData::CaseVariable {
                                ident: _,
                                pattern_variable_idx: _,
                            } => todo!(),
                            CurrentVariableData::FieldVariable { ident_token: _ } => {
                                todo!()
                            }
                            CurrentVariableData::LoopVariable {
                                ident: _,
                                expr_idx: _,
                            } => todo!(),
                            CurrentVariableData::SimpleClosureParameter {
                                ident,
                                pattern_variable_idx,
                            } => todo!(),
                        },
                    };
                    arena.alloc_one(HirEagerComptimeVariableEntry {
                        name,
                        data: HirEagerComptimeVariableData::Current,
                        hir_comptime_symbol,
                    });
                }
                _ => todo!(),
            }
        }
        Self { arena }
    }

    pub fn symbol_name(
        &self,
        hir_comptime_symbol: HirTemplateVariable,
    ) -> Option<HirEagerComptimeVariableName> {
        self.arena.iter().find_map(|entry| {
            (entry.hir_comptime_symbol == hir_comptime_symbol).then_some(entry.name)
        })
    }
}
