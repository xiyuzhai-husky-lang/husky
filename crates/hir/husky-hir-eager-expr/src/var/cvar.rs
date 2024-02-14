use super::*;
use husky_coword::Ident;
use husky_eth_term::term::EthTerm;
use husky_fly_term::FlyTermBase;
use husky_hir_ty::HirTemplateVar;
use husky_sema_expr::SemaExprRegionData;
use husky_syn_expr::{
    CurrentSynSymbolData, CurrentTemplateParameterSynSymbolVariant, VariableRegionData,
};

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerComptimeSvarRegionData {
    arena: HirEagerComptimeSvarArena,
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerComptimeSvarEntry {
    name: HirEagerComptimeSvarName,
    data: HirEagerComptimeSvarData,
    hir_comptime_symbol: HirTemplateVar,
}

pub type HirEagerComptimeSvarArena = Arena<HirEagerComptimeSvarEntry>;
pub type HirEagerComptimeSvarIdx = ArenaIdx<HirEagerComptimeSvarEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum HirEagerComptimeSvarName {
    SelfType,
    Ident(Ident),
    Label(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerComptimeSvarData {
    Inherited,
    Current,
}

impl HirEagerComptimeSvarEntry {
    pub fn name(&self) -> HirEagerComptimeSvarName {
        self.name
    }

    pub fn data(&self) -> &HirEagerComptimeSvarData {
        &self.data
    }

    pub fn symbol(&self) -> HirTemplateVar {
        self.hir_comptime_symbol
    }
}

impl HirEagerComptimeSvarRegionData {
    pub(crate) fn from_sema(
        sema_expr_region_data: &SemaExprRegionData,
        syn_symobl_region_data: &VariableRegionData,
        db: &::salsa::Db,
    ) -> Self {
        let mut arena = HirEagerComptimeSvarArena::default();
        let terms = sema_expr_region_data.fly_term_region().terms();
        for (inherited_syn_symbol_idx, &fly_term) in sema_expr_region_data
            .symbol_terms()
            .inherited_syn_symbol_key_values()
        {
            let FlyTermBase::Eth(term) = fly_term.base_resolved_inner(terms) else {
                unreachable!()
            };
            match term {
                EthTerm::Symbol(term_symbol) => {
                    let Some(hir_comptime_symbol) = HirTemplateVar::from_eth(term_symbol, db)
                    else {
                        continue;
                    };
                    let name = match syn_symobl_region_data[inherited_syn_symbol_idx].ident() {
                        Some(ident) => HirEagerComptimeSvarName::Ident(ident),
                        None => todo!(),
                    };
                    arena.alloc_one(HirEagerComptimeSvarEntry {
                        name,
                        data: HirEagerComptimeSvarData::Inherited,
                        hir_comptime_symbol,
                    });
                }
                _ => todo!(),
            }
        }
        for (current_syn_symbol_idx, &fly_term) in sema_expr_region_data
            .symbol_terms()
            .current_syn_symbol_key_values()
        {
            let FlyTermBase::Eth(term) = fly_term.base_resolved_inner(terms) else {
                unreachable!()
            };
            match term {
                EthTerm::Symbol(term_symbol) => {
                    let Some(hir_comptime_symbol) = HirTemplateVar::from_eth(term_symbol, db)
                    else {
                        continue;
                    };
                    let current_syn_symbol = &syn_symobl_region_data[current_syn_symbol_idx];
                    let name = match current_syn_symbol.ident() {
                        Some(ident) => HirEagerComptimeSvarName::Ident(ident),
                        None => match current_syn_symbol.data() {
                            CurrentSynSymbolData::TemplateParameter {
                                syn_attrs: _,
                                annotated_variance_token: _,
                                template_parameter_variant,
                            } => match template_parameter_variant {
                                CurrentTemplateParameterSynSymbolVariant::Lifetime {
                                    label_token,
                                } => HirEagerComptimeSvarName::Label(label_token.label()),
                                CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                                    HirEagerComptimeSvarName::Label(label_token.label())
                                }
                                CurrentTemplateParameterSynSymbolVariant::Type {
                                    ident_token: _,
                                } => {
                                    todo!()
                                }
                                CurrentTemplateParameterSynSymbolVariant::Constant {
                                    ident_token: _,
                                    ty_expr_idx: _,
                                } => todo!(),
                                _ => todo!(),
                            },
                            CurrentSynSymbolData::SelfType => todo!(),
                            CurrentSynSymbolData::SelfValue {
                                symbol_modifier_keyword_group: _,
                            } => todo!(),
                            CurrentSynSymbolData::SimpleParenateParameter {
                                ident: _,
                                pattern_symbol_idx: _,
                            } => todo!(),
                            CurrentSynSymbolData::VariadicParenateParameter {
                                symbol_modifier_keyword_group: _,
                                ident_token: _,
                            } => todo!(),
                            CurrentSynSymbolData::LetVariable {
                                ident: _,
                                pattern_symbol_idx: _,
                            } => todo!(),
                            CurrentSynSymbolData::BeVariable {
                                ident: _,
                                pattern_symbol_idx: _,
                            } => todo!(),
                            CurrentSynSymbolData::CaseVariable {
                                ident: _,
                                pattern_symbol_idx: _,
                            } => todo!(),
                            CurrentSynSymbolData::FieldVariable { ident_token: _ } => {
                                todo!()
                            }
                            CurrentSynSymbolData::LoopVariable {
                                ident: _,
                                expr_idx: _,
                            } => todo!(),
                            CurrentSynSymbolData::SimpleLambdaParameter {
                                ident,
                                pattern_symbol_idx,
                            } => todo!(),
                        },
                    };
                    arena.alloc_one(HirEagerComptimeSvarEntry {
                        name,
                        data: HirEagerComptimeSvarData::Current,
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
        hir_comptime_symbol: HirTemplateVar,
    ) -> Option<HirEagerComptimeSvarName> {
        self.arena.iter().find_map(|entry| {
            (entry.hir_comptime_symbol == hir_comptime_symbol).then_some(entry.name)
        })
    }
}
