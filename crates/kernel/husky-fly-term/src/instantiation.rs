use super::*;
use crate::quary::FlyQuary;
use husky_entity_path::path::{
    assoc_item::AssocItemPath,
    major_item::ty::{PreludeIndirectionTypePath, PreludeTypePath},
    ItemPath, PrincipalItemPath,
};
use husky_eth_term::{
    instantiation::{EthInstantiation, IsPackageEthSignatureData},
    term::{
        application::{EthApplication, TermFunctionReduced},
        ritchie::EthRitchie,
        symbolic_variable::{EthSymbolicVariable, EthTermSymbolIndexImpl},
    },
};
use salsa::fmt::WithFmtContext;
use vec_like::SmallVecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FlyInstantiation {
    path: ItemPath,
    task_ty: Option<EthTerm>,
    env: FlyInstantiationEnvironment,
    symbol_map: SmallVecPairMap<EthSymbolicVariable, FlyTermSymbolResolution, 4>,
    separator: Option<u8>,
}

impl WithFmtContext for FlyInstantiation {
    fn with_fmt_context(
        &self,
        f: impl FnOnce() -> std::fmt::Result,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        use husky_eth_term::fmt::with_eth_term_fmt_context;
        use husky_eth_term::instantiation::item_fmt_context;

        let ctx = item_fmt_context(db, *self.path);
        with_eth_term_fmt_context(ctx, f, db)
    }
}

impl std::ops::Index<EthSymbolicVariable> for FlyInstantiation {
    type Output = FlyTermSymbolResolution;

    fn index(&self, index: EthSymbolicVariable) -> &Self::Output {
        &self.symbol_map[index].1
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlyTermSymbolResolution {
    Explicit(FlyTerm),
    /// means we don't care about it now
    SelfLifetime,
    SelfQuary(FlyQuary),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlyInstantiationEnvironment {
    TypeOntologyConstructor,
    AssocRitchie,
    MethodFn { self_place: FlyQuary },
    MemoizedField,
}

impl FlyInstantiation {
    pub fn from_template_parameters<'db>(
        path: impl Into<PrincipalItemPath>,
        env: FlyInstantiationEnvironment,
        syn_expr_idx: SynExprIdx,
        template_parameters1: &[EthTemplateParameter],
        template_parameters2: Option<&[EthTemplateParameter]>,
        terms: &mut FlyTerms,
        package_signature_data: &'db impl IsPackageEthSignatureData,
        db: &'db ::salsa::Db,
    ) -> Self {
        let separator = template_parameters2
            .is_some()
            .then_some(template_parameters1.len().try_into().unwrap());
        Self {
            path: path.into().into(),
            task_ty: package_signature_data.task_ty(),
            env,
            symbol_map: template_parameters1
                .iter()
                .chain(template_parameters2.unwrap_or_default().iter())
                .map(|param| {
                    let symbol = param.symbol();
                    (
                        symbol,
                        FlyTermSymbolResolution::Explicit(
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
            separator,
        }
    }

    pub(crate) fn from_eth(
        env: FlyInstantiationEnvironment,
        instantiation: &EthInstantiation,
    ) -> Self {
        FlyInstantiation {
            path: instantiation.path(),
            task_ty: instantiation.task_ty(),
            env,
            symbol_map: instantiation
                .symbol_map()
                .iter()
                .map(|&(symbol, term)| (symbol, FlyTermSymbolResolution::Explicit(term.into())))
                .collect(),
            separator: instantiation.separator(),
        }
    }

    pub fn symbol_map(&self) -> &[(EthSymbolicVariable, FlyTermSymbolResolution)] {
        self.symbol_map.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    pub fn symbol_map_splitted(
        &self,
    ) -> (
        &[(EthSymbolicVariable, FlyTermSymbolResolution)],
        Option<&[(EthSymbolicVariable, FlyTermSymbolResolution)]>,
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

    pub fn is_empty(&self) -> bool {
        self.symbol_map.is_empty()
    }
}

pub trait FlyInstantiate: Copy {
    type Target;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target;

    // set flag to true if target is different
    fn instantiate_with_flag(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
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

pub(crate) trait FlyInstantiateRef {
    type Target;

    fn instantiate(
        &self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &mut FlyInstantiation,
    ) -> Self::Target;
}

pub struct FlyTermInstantiationBuilder {
    path: ItemPath,
    task_ty: Option<EthTerm>,
    env: FlyInstantiationEnvironment,
    symbol_map: SmallVecPairMap<EthSymbolicVariable, Option<FlyTermSymbolResolution>, 4>,
    separator: Option<u8>,
}

impl std::ops::Index<EthSymbolicVariable> for FlyTermInstantiationBuilder {
    type Output = Option<FlyTermSymbolResolution>;

    fn index(&self, index: EthSymbolicVariable) -> &Self::Output {
        &self.symbol_map[index].1
    }
}

impl FlyTermInstantiationBuilder {
    pub fn new_associated<'db>(
        path: impl Into<AssocItemPath>,
        env: FlyInstantiationEnvironment,
        impl_block_template_parameters: &[EthTemplateParameter],
        assoc_item_template_parameters: &[EthTemplateParameter],
        package_signature_data: &'db impl IsPackageEthSignatureData,
        db: &'db ::salsa::Db,
    ) -> Self {
        Self {
            path: path.into().into(),
            task_ty: package_signature_data.task_ty(),
            env,
            symbol_map: impl_block_template_parameters
                .iter()
                .chain(assoc_item_template_parameters)
                .map(|param| {
                    let symbol = param.symbol();
                    (
                        symbol,
                        match symbol.index(db).inner() {
                            EthTermSymbolIndexImpl::SelfLifetime => {
                                Some(FlyTermSymbolResolution::SelfLifetime)
                            }
                            EthTermSymbolIndexImpl::SelfPlace => Some(match env {
                                FlyInstantiationEnvironment::AssocRitchie => todo!(),
                                FlyInstantiationEnvironment::MethodFn { self_place } => {
                                    FlyTermSymbolResolution::SelfQuary(self_place)
                                }
                                FlyInstantiationEnvironment::MemoizedField => todo!(),
                                FlyInstantiationEnvironment::TypeOntologyConstructor => todo!(),
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
    pub(crate) fn try_add_rule(&mut self, src: EthTerm, dst: FlyTerm) -> FlyTermMaybeResult<()> {
        match src {
            EthTerm::Literal(_) => todo!(),
            EthTerm::SymbolicVariable(symbol) => {
                let (_, ref mut dst0) = self.symbol_map[symbol];
                match *dst0 {
                    Some(dst0) => match dst0 {
                        FlyTermSymbolResolution::Explicit(dst0) => {
                            if dst != dst0 {
                                todo!()
                                // return JustErr(FlyTermError);
                            } else {
                                return JustOk(());
                            }
                        }
                        FlyTermSymbolResolution::SelfLifetime => todo!(),
                        FlyTermSymbolResolution::SelfQuary(_) => todo!(),
                    },
                    None => *dst0 = Some(FlyTermSymbolResolution::Explicit(dst)),
                }
                JustOk(())
            }
            EthTerm::LambdaVariable(_) => todo!(),
            EthTerm::ItemPath(_) => todo!(),
            EthTerm::Category(_) => todo!(),
            EthTerm::Universe(_) => todo!(),
            EthTerm::Curry(_) => todo!(),
            EthTerm::Ritchie(_) => todo!(),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::Application(_) => todo!(),
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub(crate) fn finish(self, db: &::salsa::Db) -> FlyInstantiation {
        FlyInstantiation {
            path: self.path,
            task_ty: self.task_ty,
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

impl FlyInstantiate for EthTerm {
    type Target = FlyTerm;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        if instantiation.symbol_map.len() == 0 {
            return self.into();
        }
        match self {
            EthTerm::Literal(_) => todo!(),
            EthTerm::SymbolicVariable(symbol) => match instantiation[symbol] {
                resolution => match resolution {
                    FlyTermSymbolResolution::Explicit(term) => term,
                    FlyTermSymbolResolution::SelfLifetime => todo!(),
                    FlyTermSymbolResolution::SelfQuary(place) => place.into(),
                },
            },
            EthTerm::LambdaVariable(_) => todo!(),
            EthTerm::ItemPath(_) => self.into(),
            EthTerm::Category(_) => todo!(),
            EthTerm::Universe(_) => todo!(),
            EthTerm::Curry(_) => todo!(),
            EthTerm::Ritchie(term) => term.instantiate(engine, expr_idx, instantiation),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::Application(term) => term.instantiate(engine, expr_idx, instantiation),
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        }
    }
}

impl FlyInstantiate for EthApplication {
    type Target = FlyTerm;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        let mut flag = false;
        let db = engine.db();
        let application_expansion = self.application_expansion(db);
        let arguments = application_expansion.arguments(db);
        match application_expansion.function() {
            TermFunctionReduced::TypeOntology(path) => match path.refine(db) {
                Left(PreludeTypePath::Indirection(PreludeIndirectionTypePath::At)) => {
                    debug_assert_eq!(arguments.len(), 2);
                    let the_place = arguments[0].instantiate(engine, expr_idx, instantiation);
                    let the_place = match the_place.base() {
                        FlyTermBase::Eth(_) => todo!(),
                        FlyTermBase::Sol(_) => todo!(),
                        FlyTermBase::Hol(_) => todo!(),
                        FlyTermBase::Place => the_place.quary().unwrap(),
                    };
                    let base = arguments[1].instantiate(engine, expr_idx, instantiation);
                    match base.quary() {
                        Some(_) => todo!(),
                        None => base.with_quary(the_place),
                    }
                }
                refined_path => {
                    let arguments = arguments
                        .iter()
                        .map(|argument| {
                            argument.instantiate_with_flag(
                                engine,
                                expr_idx,
                                instantiation,
                                &mut flag,
                            )
                        })
                        .collect();
                    if flag {
                        FlyTerm::new_ty_ontology(
                            db,
                            engine.fly_terms_mut(),
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

impl FlyInstantiate for EthRitchie {
    type Target = FlyTerm;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        let mut flag = false;
        let db = engine.db();
        let params: Vec<_> = self
            .parameter_contracted_tys(db)
            .iter()
            .map(|param| param.instantiate_with_flag(engine, expr_idx, instantiation, &mut flag))
            .collect();
        let return_ty =
            self.return_ty(db)
                .instantiate_with_flag(engine, expr_idx, instantiation, &mut flag);
        match flag {
            true => FlyTerm::new_ritchie(engine, self.ritchie_kind(db), params, return_ty)
                .expect("should be okay"),
            false => self.into(),
        }
    }
}
