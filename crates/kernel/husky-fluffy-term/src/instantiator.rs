use husky_ethereal_signature::ExplicitParameterEtherealSignatureTemplate;
use vec_like::VecPairMap;

use super::*;

#[derive(Default)]
pub(crate) struct Instantiator {
    symbol_map: VecPairMap<EtherealTermSymbol, FluffyTerm>,
}

impl Instantiator {
    /// JustOk(()) means rule is added and everything is compatible
    /// Nothing means something is incompatible
    /// JustErr(_) means something is wrong
    pub(crate) fn try_add_rule(
        &mut self,
        src: EtherealTerm,
        dst: FluffyTerm,
    ) -> FluffyTermMaybeResult<()> {
        match src {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(symbol) => {
                if let Some((_, dst0)) = self.symbol_map.get_entry(symbol) {
                    if dst != *dst0 {
                        todo!()
                    } else {
                        return JustOk(());
                    }
                }
                self.symbol_map.insert_new_unchecked((symbol, dst));
                JustOk(())
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
}

pub(crate) trait Instantiate: Copy {
    type Target;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        instantiator: &mut Instantiator,
    ) -> Self::Target;
}

pub(crate) trait InstantiateRef {
    type Target;

    fn instantiate(
        &self,
        engine: &mut impl FluffyTermEngine,
        instantiator: &mut Instantiator,
    ) -> Self::Target;
}

impl Instantiate for EtherealTerm {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        instantiator: &mut Instantiator,
    ) -> Self::Target {
        if instantiator.symbol_map.len() == 0 {
            return self.into();
        }
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(symbol) => match instantiator.symbol_map.get_entry(symbol) {
                Some((_, instantiated_term)) => *instantiated_term,
                None => todo!(),
            },
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(_) => self.into(),
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
