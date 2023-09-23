use vec_like::VecPairMap;

use super::*;

pub(crate) struct FluffyTermInstantiation {
    env: FluffyTermInstantiationEnvironment,
    symbol_map: VecPairMap<EtherealTermSymbol, FluffyTerm>,
}

#[derive(Debug, Clone, Copy)]
pub enum FluffyTermInstantiationEnvironment {
    AssociatedFn,
    TypeMethodFn { self_place: Place },
}

impl FluffyTermInstantiation {
    pub(crate) fn new(env: FluffyTermInstantiationEnvironment) -> Self {
        Self {
            env,
            symbol_map: Default::default(),
        }
    }

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
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub(crate) fn env(&self) -> FluffyTermInstantiationEnvironment {
        self.env
    }
}

pub(crate) trait FluffyTermInstantiate: Copy {
    type Target;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target;

    // set flag to true if target is different
    fn instantiate_with_flag(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
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
        expr_idx: SynExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target;
}

impl FluffyTermInstantiate for EtherealTerm {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target {
        if instantiation.symbol_map.len() == 0 {
            return self.into();
        }
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(symbol) => match instantiation.symbol_map.get_entry(symbol) {
                Some((_, instantiated_term)) => *instantiated_term,
                None => match symbol.index(engine.db()).inner() {
                    EtherealTermSymbolIndexInner::Lifetime {
                        attrs,
                        variance,
                        disambiguator,
                    } => todo!(),
                    EtherealTermSymbolIndexInner::Type {
                        attrs,
                        variance,
                        disambiguator,
                    } => todo!(),
                    EtherealTermSymbolIndexInner::Prop { disambiguator } => todo!(),
                    EtherealTermSymbolIndexInner::ConstPathLeading {
                        attrs,
                        disambiguator,
                        ty_path,
                    } => todo!(),
                    EtherealTermSymbolIndexInner::ConstOther {
                        attrs,
                        disambiguator,
                    } => todo!(),
                    EtherealTermSymbolIndexInner::EphemPathLeading {
                        disambiguator,
                        ty_path,
                    } => todo!(),
                    EtherealTermSymbolIndexInner::EphemOther { disambiguator } => todo!(),
                    EtherealTermSymbolIndexInner::SelfType => todo!(),
                    EtherealTermSymbolIndexInner::SelfValue => todo!(),
                    EtherealTermSymbolIndexInner::SelfLifetime => todo!(),
                    EtherealTermSymbolIndexInner::SelfPlace => match instantiation.env() {
                        FluffyTermInstantiationEnvironment::AssociatedFn => todo!(),
                        FluffyTermInstantiationEnvironment::TypeMethodFn { self_place } => {
                            self_place.into()
                        }
                    },
                },
            },
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(_) => self.into(),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(term) => term.instantiate(engine, expr_idx, instantiation),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => term.instantiate(engine, expr_idx, instantiation),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

impl FluffyTermInstantiate for EtherealTermApplication {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
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

impl FluffyTermInstantiate for EtherealTermRitchie {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target {
        let mut flag = false;
        let params: Vec<_> = self
            .parameter_contracted_tys(engine.db())
            .iter()
            .map(|param| param.instantiate_with_flag(engine, expr_idx, instantiation, &mut flag))
            .collect();
        let db = engine.db();
        match flag {
            true => FluffyTerm::new_ritchie(
                engine,
                expr_idx,
                self.ritchie_kind(db),
                params,
                self.return_ty(db).into(),
            )
            .expect("should be okay"),
            false => self.into(),
        }
    }
}
