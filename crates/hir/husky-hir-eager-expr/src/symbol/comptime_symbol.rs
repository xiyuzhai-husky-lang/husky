use super::*;
use husky_coword::Ident;
use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::FluffyTermBase;
use husky_hir_ty::HirComptimeSymbol;
use husky_sema_expr::SemaExprRegionData;
use husky_syn_expr::{
    CurrentSynSymbolData, CurrentSynSymbolKind, CurrentTemplateParameterSynSymbolVariant,
    SynExprRegionData, SynSymbolRegionData,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerComptimeSymbolRegionData {
    arena: HirEagerComptimeSymbolArena,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerComptimeSymbolEntry {
    name: HirEagerComptimeSymbolName,
    data: HirEagerComptimeSymbolData,
    hir_comptime_symbol: HirComptimeSymbol,
}

pub type HirEagerComptimeSymbolArena = Arena<HirEagerComptimeSymbolEntry>;
pub type HirEagerComptimeSymbolIdx = ArenaIdx<HirEagerComptimeSymbolEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub enum HirEagerComptimeSymbolName {
    SelfType,
    Ident(Ident),
    Label(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerComptimeSymbolData {
    Inherited,
    Current,
}

impl HirEagerComptimeSymbolEntry {
    pub fn name(&self) -> HirEagerComptimeSymbolName {
        self.name
    }

    pub fn data(&self) -> &HirEagerComptimeSymbolData {
        &self.data
    }

    pub fn symbol(&self) -> HirComptimeSymbol {
        self.hir_comptime_symbol
    }
}

impl HirEagerComptimeSymbolRegionData {
    pub(crate) fn from_sema(
        sema_expr_region_data: &SemaExprRegionData,
        syn_symobl_region_data: &SynSymbolRegionData,
        db: &dyn HirEagerExprDb,
    ) -> Self {
        let mut arena = HirEagerComptimeSymbolArena::default();
        let terms = sema_expr_region_data.fluffy_term_region().terms();
        for (inherited_syn_symbol_idx, &fluffy_term) in sema_expr_region_data
            .symbol_terms()
            .inherited_syn_symbol_key_values()
        {
            let FluffyTermBase::Ethereal(term) = fluffy_term.base_resolved_inner(terms) else {
                unreachable!()
            };
            match term {
                EtherealTerm::Symbol(term_symbol) => {
                    let Some(hir_comptime_symbol) =
                        HirComptimeSymbol::from_ethereal(term_symbol, db)
                    else {
                        continue;
                    };
                    let name = match syn_symobl_region_data[inherited_syn_symbol_idx].ident() {
                        Some(ident) => HirEagerComptimeSymbolName::Ident(ident),
                        None => todo!(),
                    };
                    arena.alloc_one(HirEagerComptimeSymbolEntry {
                        name,
                        data: HirEagerComptimeSymbolData::Inherited,
                        hir_comptime_symbol,
                    });
                }
                _ => todo!(),
            }
        }
        for (current_syn_symbol_idx, &fluffy_term) in sema_expr_region_data
            .symbol_terms()
            .current_syn_symbol_key_values()
        {
            let FluffyTermBase::Ethereal(term) = fluffy_term.base_resolved_inner(terms) else {
                unreachable!()
            };
            match term {
                EtherealTerm::Symbol(term_symbol) => {
                    let Some(hir_comptime_symbol) =
                        HirComptimeSymbol::from_ethereal(term_symbol, db)
                    else {
                        continue;
                    };
                    let current_syn_symbol = &syn_symobl_region_data[current_syn_symbol_idx];
                    let name = match current_syn_symbol.ident() {
                        Some(ident) => HirEagerComptimeSymbolName::Ident(ident),
                        None => match current_syn_symbol.data() {
                            CurrentSynSymbolData::TemplateParameter {
                                syn_attrs,
                                annotated_variance_token,
                                template_parameter_variant,
                            } => match template_parameter_variant {
                                CurrentTemplateParameterSynSymbolVariant::Lifetime {
                                    label_token,
                                } => HirEagerComptimeSymbolName::Label(label_token.label()),
                                CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                                    HirEagerComptimeSymbolName::Label(label_token.label())
                                }
                                CurrentTemplateParameterSynSymbolVariant::Type { ident_token } => {
                                    todo!()
                                }
                                CurrentTemplateParameterSynSymbolVariant::Constant {
                                    ident_token,
                                    ty_expr_idx,
                                } => todo!(),
                                _ => todo!(),
                            },
                            CurrentSynSymbolData::SelfType => todo!(),
                            CurrentSynSymbolData::SelfValue {
                                symbol_modifier_keyword_group,
                            } => todo!(),
                            CurrentSynSymbolData::ParenateRegularParameter {
                                ident,
                                pattern_symbol_idx,
                            } => todo!(),
                            CurrentSynSymbolData::ParenateVariadicParameter {
                                symbol_modifier_keyword_group,
                                ident_token,
                            } => todo!(),
                            CurrentSynSymbolData::LetVariable {
                                ident,
                                pattern_symbol_idx,
                            } => todo!(),
                            CurrentSynSymbolData::BeVariable {
                                ident,
                                pattern_symbol_idx,
                            } => todo!(),
                            CurrentSynSymbolData::CaseVariable {
                                ident,
                                pattern_symbol_idx,
                            } => todo!(),
                            CurrentSynSymbolData::FieldVariable { ident_token } => {
                                todo!()
                            }
                            CurrentSynSymbolData::LoopVariable { ident, expr_idx } => todo!(),
                        },
                    };
                    arena.alloc_one(HirEagerComptimeSymbolEntry {
                        name,
                        data: HirEagerComptimeSymbolData::Current,
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
        hir_comptime_symbol: HirComptimeSymbol,
    ) -> Option<HirEagerComptimeSymbolName> {
        self.arena.iter().find_map(|entry| {
            (entry.hir_comptime_symbol == hir_comptime_symbol).then_some(entry.name)
        })
    }
}
