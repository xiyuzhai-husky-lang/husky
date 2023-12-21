use super::*;
use husky_ethereal_term::instantiation::EtherealInstantiation;
use vec_like::SmallVecPairMap;

#[derive(Debug, PartialEq, Eq, Clone)]
// #[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct FluffyInstantiation {
    env: FluffyInstantiationEnvironment,
    symbol_map: SmallVecPairMap<EtherealTermSymbol, FluffyTermSymbolResolution, 4>,
    separator: Option<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyTermSymbolResolution {
    Explicit(FluffyTerm),
    /// means we don't care about it now
    SelfLifetime,
    SelfPlace(FluffyPlace),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyInstantiationEnvironment {
    TypeOntologyConstructor,
    AssociatedFn,
    MethodFn { self_place: FluffyPlace },
    MemoizedField,
}

impl FluffyInstantiation {
    pub fn from_template_parameters(
        env: FluffyInstantiationEnvironment,
        syn_expr_idx: SynExprIdx,
        template_parameters: &[EtherealTemplateParameter],
        terms: &mut FluffyTerms,
        db: &::salsa::Db,
    ) -> Self {
        Self {
            env,
            symbol_map: template_parameters
                .iter()
                .map(|param| {
                    let symbol = param.symbol();
                    (
                        symbol,
                        FluffyTermSymbolResolution::Explicit(
                            terms
                                .new_hole_from_template_parameter_symbol(
                                    syn_expr_idx.into(),
                                    symbol,
                                    db,
                                )
                                .into(),
                        ),
                    )
                })
                .collect(),
            separator: None,
        }
    }

    pub(crate) fn from_ethereal(
        env: FluffyInstantiationEnvironment,
        instantiation: &EtherealInstantiation,
    ) -> Self {
        FluffyInstantiation {
            env,
            symbol_map: instantiation
                .symbol_map()
                .iter()
                .map(|&(symbol, term)| (symbol, FluffyTermSymbolResolution::Explicit(term.into())))
                .collect(),
            separator: instantiation.separator(),
        }
    }

    pub fn symbol_map(&self) -> &[(EtherealTermSymbol, FluffyTermSymbolResolution)] {
        self.symbol_map.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    pub fn symbol_map_splitted(
        &self,
    ) -> (
        &[(EtherealTermSymbol, FluffyTermSymbolResolution)],
        Option<&[(EtherealTermSymbol, FluffyTermSymbolResolution)]>,
    ) {
        let symbol_map: &[_] = self.symbol_map.as_ref();
        match self.separator {
            Some(separator) => {
                let separator = separator as usize;
                (&symbol_map[0..separator], Some(&symbol_map[separator..]))
            }
            None => (symbol_map, None),
        }
    }
}

pub trait FluffyInstantiate: Copy {
    type Target;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        builder: &mut FluffyInstantiationBuilder,
    ) -> Self::Target;

    // set flag to true if target is different
    fn instantiate_with_flag(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        builder: &mut FluffyInstantiationBuilder,
        flag: &mut bool,
    ) -> Self::Target
    where
        Self: Into<Self::Target>,
        Self::Target: Eq,
    {
        let target = self.instantiate(engine, expr_idx, builder);
        let this: Self::Target = self.into();
        if target != this {
            *flag = true
        }
        target
    }
}

pub(crate) trait FluffyInstantiateRef {
    type Target;

    fn instantiate(
        &self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &mut FluffyInstantiation,
    ) -> Self::Target;
}

pub struct FluffyInstantiationBuilder {
    env: FluffyInstantiationEnvironment,
    symbol_map: SmallVecPairMap<EtherealTermSymbol, Option<FluffyTermSymbolResolution>, 4>,
    separator: Option<u8>,
}

impl std::ops::Index<EtherealTermSymbol> for FluffyInstantiationBuilder {
    type Output = Option<FluffyTermSymbolResolution>;

    fn index(&self, index: EtherealTermSymbol) -> &Self::Output {
        &self.symbol_map[index].1
    }
}

impl FluffyInstantiationBuilder {
    pub fn new_associated(
        env: FluffyInstantiationEnvironment,
        impl_block_template_parameters: &[EtherealTemplateParameter],
        associated_item_template_parameters: &[EtherealTemplateParameter],
        db: &::salsa::Db,
    ) -> Self {
        Self {
            env,
            symbol_map: impl_block_template_parameters
                .iter()
                .chain(associated_item_template_parameters)
                .map(|param| {
                    let symbol = param.symbol();
                    (
                        symbol,
                        match symbol.index(db).inner() {
                            EtherealTermSymbolIndexInner::SelfLifetime => {
                                Some(FluffyTermSymbolResolution::SelfLifetime)
                            }
                            EtherealTermSymbolIndexInner::SelfPlace => Some(match env {
                                FluffyInstantiationEnvironment::AssociatedFn => todo!(),
                                FluffyInstantiationEnvironment::MethodFn { self_place } => {
                                    FluffyTermSymbolResolution::SelfPlace(self_place)
                                }
                                FluffyInstantiationEnvironment::MemoizedField => todo!(),
                                FluffyInstantiationEnvironment::TypeOntologyConstructor => todo!(),
                            }),
                            _ => None,
                        },
                    )
                })
                .collect(),
            separator: Some(
                impl_block_template_parameters
                    .len()
                    .try_into()
                    .expect("within range"),
            ),
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
                let (_, ref mut dst0) = self.symbol_map[symbol];
                match *dst0 {
                    Some(dst0) => match dst0 {
                        FluffyTermSymbolResolution::Explicit(dst0) => {
                            if dst != dst0 {
                                todo!()
                            } else {
                                return JustOk(());
                            }
                        }
                        FluffyTermSymbolResolution::SelfLifetime => todo!(),
                        FluffyTermSymbolResolution::SelfPlace(_) => todo!(),
                    },
                    None => *dst0 = Some(FluffyTermSymbolResolution::Explicit(dst)),
                }
                JustOk(())
            }
            EtherealTerm::Rune(_) => todo!(),
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

    pub(crate) fn finish(self, db: &::salsa::Db) -> FluffyInstantiation {
        FluffyInstantiation {
            env: self.env,
            symbol_map: self
                .symbol_map
                .into_iter()
                .map(|(symbol, resolution)| (symbol, resolution.unwrap()))
                .collect(),
            separator: self.separator,
        }
    }
}

impl FluffyInstantiate for EtherealTerm {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        builder: &mut FluffyInstantiationBuilder,
    ) -> Self::Target {
        if builder.symbol_map.len() == 0 {
            return self.into();
        }
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(symbol) => match builder[symbol] {
                Some(resolution) => match resolution {
                    FluffyTermSymbolResolution::Explicit(term) => term,
                    FluffyTermSymbolResolution::SelfLifetime => todo!(),
                    FluffyTermSymbolResolution::SelfPlace(place) => place.into(),
                },
                None => match symbol.index(engine.db()).inner() {
                    EtherealTermSymbolIndexInner::ExplicitLifetime {
                        attrs,
                        variance,
                        disambiguator,
                    } => todo!(),
                    EtherealTermSymbolIndexInner::ExplicitPlace {
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
                    EtherealTermSymbolIndexInner::SelfPlace => match builder.env {
                        FluffyInstantiationEnvironment::AssociatedFn => todo!(),
                        FluffyInstantiationEnvironment::MethodFn { self_place } => {
                            self_place.into()
                        }
                        FluffyInstantiationEnvironment::MemoizedField => todo!(),
                        FluffyInstantiationEnvironment::TypeOntologyConstructor => todo!(),
                    },
                },
            },
            EtherealTerm::Rune(_) => todo!(),
            EtherealTerm::EntityPath(_) => self.into(),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(term) => term.instantiate(engine, expr_idx, builder),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => term.instantiate(engine, expr_idx, builder),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

impl FluffyInstantiate for EtherealTermApplication {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        builder: &mut FluffyInstantiationBuilder,
    ) -> Self::Target {
        let mut flag = false;
        let db = engine.db();
        let application_expansion = self.application_expansion(db);
        let arguments = application_expansion.arguments(db);
        match application_expansion.function() {
            TermFunctionReduced::TypeOntology(path) => match path.refine(db) {
                Left(PreludeTypePath::Indirection(PreludeIndirectionTypePath::At)) => {
                    debug_assert_eq!(arguments.len(), 2);
                    let the_place = arguments[0].instantiate(engine, expr_idx, builder);
                    let the_place = match the_place.base() {
                        FluffyTermBase::Ethereal(_) => todo!(),
                        FluffyTermBase::Solid(_) => todo!(),
                        FluffyTermBase::Hollow(_) => todo!(),
                        FluffyTermBase::Place => the_place.place().unwrap(),
                    };
                    let base = arguments[1].instantiate(engine, expr_idx, builder);
                    match base.place() {
                        Some(_) => todo!(),
                        None => base.with_place(the_place),
                    }
                }
                refined_path => {
                    let arguments = arguments
                        .iter()
                        .map(|argument| {
                            argument.instantiate_with_flag(engine, expr_idx, builder, &mut flag)
                        })
                        .collect();
                    if flag {
                        FluffyTerm::new_ty_ontology(
                            db,
                            engine.fluffy_terms_mut(),
                            path,
                            refined_path,
                            arguments,
                        )
                    } else {
                        self.into()
                    }
                }
            },
            TermFunctionReduced::Trait(_) => todo!(),
            TermFunctionReduced::Other(_) => todo!(),
        }
    }
}

impl FluffyInstantiate for EtherealTermRitchie {
    type Target = FluffyTerm;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        builder: &mut FluffyInstantiationBuilder,
    ) -> Self::Target {
        let mut flag = false;
        let params: Vec<_> = self
            .parameter_contracted_tys(engine.db())
            .iter()
            .map(|param| param.instantiate_with_flag(engine, expr_idx, builder, &mut flag))
            .collect();
        let db = engine.db();
        match flag {
            true => FluffyTerm::new_ritchie(
                engine,
                self.ritchie_kind(db),
                params,
                self.return_ty(db).into(),
            )
            .expect("should be okay"),
            false => self.into(),
        }
    }
}
