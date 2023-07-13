use crate::*;
use maybe_result::*;
use vec_like::{SmallVecPairMap, VecPairMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct EtherealTermPartialInstantiation {
    symbol_map: SmallVecPairMap<EtherealTermSymbol, Option<EtherealTerm>, 4>,
}

impl EtherealTermPartialInstantiation {
    /// symbols must be unique
    pub(crate) fn new(symbols: impl Iterator<Item = EtherealTermSymbol>) -> Self {
        Self {
            symbol_map: symbols.map(|symbol| (symbol, None)).collect(),
        }
    }

    /// JustOk(()) means rule is added and everything is compatible
    /// Nothing means something is incompatible
    /// JustErr(_) means something is wrong
    pub fn try_add_rules_from_application(
        &mut self,
        db: &dyn EtherealTermDb,
        src: EtherealTerm,
        dst_arguments: &[EtherealTerm],
    ) -> EtherealTermMaybeResult<()> {
        let src_application_expansion = src.application_expansion(db);
        if src_application_expansion.arguments(db).len() != dst_arguments.len() {
            todo!()
        }
        std::iter::zip(
            src_application_expansion.arguments(db).iter().copied(),
            dst_arguments.iter().copied(),
        )
        .try_for_each(|(src, dst)| self.try_add_rule(src, dst.into()))?;
        JustOk(())
    }

    /// JustOk(()) means rule is added and everything is compatible
    /// Nothing means something is incompatible
    /// JustErr(_) means something is wrong
    pub fn try_add_rule(
        &mut self,
        src: EtherealTerm,
        dst: EtherealTerm,
    ) -> EtherealTermMaybeResult<()> {
        if src == dst {
            return JustOk(());
        }
        match src {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(symbol) => {
                if let Some((_, opt_dst0)) = self.symbol_map.get_entry_mut(symbol) {
                    match opt_dst0 {
                        Some(dst0) => {
                            if dst != *dst0 {
                                todo!()
                            } else {
                                return JustOk(());
                            }
                        }
                        None => {
                            *opt_dst0 = Some(dst);
                            JustOk(())
                        }
                    }
                } else {
                    Nothing
                }
            }
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(_) => todo!(),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub fn try_into_instantiation(self) -> Option<EtherealTermInstantiation> {
        let mut symbol_map = SmallVecPairMap::<EtherealTermSymbol, EtherealTerm, 4>::default();
        for (symbol, mapped) in self.symbol_map.iter() {
            let mapped = (*mapped)?;
            unsafe { symbol_map.insert_new_unchecked((*symbol, mapped)) }
        }
        Some(EtherealTermInstantiation { symbol_map })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct EtherealTermInstantiation {
    symbol_map: SmallVecPairMap<EtherealTermSymbol, EtherealTerm, 4>,
}

impl EtherealTermInstantiation {
    /// assume that symbol is in symbol_map
    /// panic otherwise
    pub fn symbol_mapped(&self, symbol: EtherealTermSymbol) -> EtherealTerm {
        *self
            .symbol_map
            .get_value(symbol)
            .expect("symbol should be in symbol_map")
    }

    // /// assume that symbol is in symbol_map
    // /// panic otherwise
    // pub fn is_symbol_resolved(&self, symbol: EtherealTermSymbol) -> bool {
    //     self.symbol_map[symbol].1.is_some()
    // }
}

pub trait EtherealTermInstantiate: Copy {
    type Target;

    fn instantiate(
        self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealTermInstantiation,
    ) -> Self::Target;
}

pub trait EtherealTermInstantiateRef {
    type Target;

    fn instantiate(
        &self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealTermInstantiation,
    ) -> Self::Target;
}
