use crate::*;
use vec_like::VecPairMap;

pub(crate) struct EtherealInstantiator {
    symbol_map: VecPairMap<EtherealTermSymbol, Option<EtherealTerm>>,
}

impl EtherealInstantiator {
    pub(crate) fn new(implicit_parameters: &ImplicitParameterEtherealSignatures) -> Self {
        Self {
            symbol_map: unsafe {
                VecPairMap::from_iter_assuming_no_repetitions_unchecked(
                    implicit_parameters
                        .iter()
                        .map(|param| (param.symbol(), None)),
                )
            },
        }
    }

    /// JustOk(()) means rule is added and everything is compatible
    /// Nothing means something is incompatible
    /// JustErr(_) means something is wrong
    pub(crate) fn try_add_rules_from_application(
        &mut self,
        db: &dyn EtherealSignatureDb,
        src: EtherealTerm,
        dst_arguments: &[EtherealTerm],
    ) -> EtherealSignatureMaybeResult<()> {
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
    pub(crate) fn try_add_rule(
        &mut self,
        src: EtherealTerm,
        dst: EtherealTerm,
    ) -> EtherealSignatureMaybeResult<()> {
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

    /// assume that symbol is in symbol_map
    /// panic otherwise
    pub(crate) fn is_symbol_resolved(&self, symbol: EtherealTermSymbol) -> bool {
        self.symbol_map[symbol].1.is_some()
    }
}

pub(crate) trait EtherealInstantiate: Copy {
    type Target;

    fn instantiate(
        self,
        db: &dyn EtherealSignatureDb,
        instantiator: &EtherealInstantiator,
    ) -> Self::Target;
}

impl EtherealInstantiate for EtherealTerm {
    type Target = EtherealTerm;

    fn instantiate(
        self,
        db: &dyn EtherealSignatureDb,
        instantiator: &EtherealInstantiator,
    ) -> Self::Target {
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(_) => todo!(),
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
}

pub(crate) trait EtherealInstantiateRef {
    type Target;

    fn instantiate(
        &self,
        db: &dyn EtherealSignatureDb,
        instantiator: &EtherealInstantiator,
    ) -> Self::Target;
}
