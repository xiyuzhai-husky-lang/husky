use vec_like::VecPairMap;

use super::*;

#[derive(Default)]
pub(crate) struct FluffyTermInstantiation {
    symbol_map: VecPairMap<EtherealTermSymbol, FluffyTerm>,
}

impl FluffyTermInstantiation {
    // todo: add try_add_rules_from_application as in Etherealinstantiation

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
                unsafe { self.symbol_map.insert_new_unchecked((symbol, dst)) };
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

pub(crate) trait FluffyTermInstantiate: Copy {
    type Target;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target;

    // set flag to true if target is different
    fn instantiate_with_flag(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        instantiation: &mut FluffyTermInstantiation,
        flag: &mut bool,
    ) -> Self::Target
    where
        Self: Into<Self::Target>,
        Self::Target: Eq,
    {
        let target = self.instantiate(engine, expr_idx, instantiation);
        let this: Self::Target = self.into();
        if target != this {
            *flag = true
        }
        target
    }
}

pub(crate) trait InstantiateRef {
    type Target;

    fn instantiate(
        &self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target;
}

impl FluffyTermInstantiate for EtherealTerm {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target {
        if instantiation.symbol_map.len() == 0 {
            return self.into();
        }
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(symbol) => match instantiation.symbol_map.get_entry(symbol) {
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
            EtherealTerm::Application(term) => term.instantiate(engine, expr_idx, instantiation),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

impl FluffyTermInstantiate for EtherealTermApplication {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target {
        let mut flag = false;
        let function = self.function(engine.db()).instantiate_with_flag(
            engine,
            expr_idx,
            instantiation,
            &mut flag,
        );
        let argument = self.argument(engine.db()).instantiate_with_flag(
            engine,
            expr_idx,
            instantiation,
            &mut flag,
        );
        match flag {
            true => FluffyTerm::new_application(engine, expr_idx, function, argument)
                .expect("should be okay"),
            false => self.into(),
        }
    }
}
