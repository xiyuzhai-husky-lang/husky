mod abstraction;
pub mod application;
mod constraint;
mod curry;
mod literal;
mod ritchie;
mod rune;
mod symbol;
mod ty_as_trai_item;

use std::fmt::Debug;

pub use self::abstraction::*;
pub use self::application::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::ritchie::*;
pub use self::rune::*;
pub use self::symbol::*;
pub use self::ty_as_trai_item::*;

use crate::instantiation::*;
use crate::*;
use husky_coword::Ident;
use husky_dec_term::term::DecTerm;
use husky_term_prelude::literal::TermLiteral;
use salsa::DisplayWithDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EthTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(TermLiteral),
    Symbol(SymbolEthTerm),
    /// the name `rune` is to be distinguishable from runtime variable
    Rune(RuneEthTerm),
    EntityPath(ItemPathTerm),
    Category(CategoryTerm),
    Universe(UniverseTerm),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(CurryEthTerm),
    /// in memory of Dennis M.Ritchie
    /// a type or trait
    Ritchie(RitchieEthTerm),
    /// lambda x => expr
    Abstraction(AbstractionEthTerm),

    /// in husky, application is generalized to include composition as a special case;
    ///
    /// when shift is `0`, this is the normal application;
    ///
    /// when shift is `1`, this is composition,
    ///
    /// in general when shift is `n`, this is equavalent to
    ///
    /// use abstraction `n` times, and then apply original argument to them,
    ///
    /// then apply function to the result,
    ///
    /// `\x1 ... \xn -> $function ($argument \x1 ... \xn)`
    Application(ApplicationEthTerm),
    /// (<type> as <trait>)::<ident>
    TypeAsTraitItem(TypeAsTraitItemEthTerm),
    /// <type> : <trait>
    TraitConstraint(TraitConstraintEthTerm),
}

impl EthTerm {
    #[track_caller]
    pub fn ty_from_declarative(db: &::salsa::Db, declarative_term: DecTerm) -> EthTermResult<Self> {
        let ty_term = Self::from_declarative(
            db,
            declarative_term,
            TermTypeExpectation::FinalDestinationEqsSort,
        )?;
        match ty_term.raw_ty(db)? {
            RawType::Declarative(DecTerm::Category(_)) => Ok(ty_term),
            _ => Err(EthTermError::ExpectedType {
                expectee: declarative_term,
            }),
        }
    }

    pub fn from_declarative(
        db: &::salsa::Db,
        declarative_term: DecTerm,
        ty_expectation: TermTypeExpectation,
    ) -> EthTermResult<Self> {
        Ok(match declarative_term {
            DecTerm::Literal(literal) => {
                EthTerm::from_literal_declarative_term(db, literal, ty_expectation)?
            }
            DecTerm::Symbol(declarative_term) => {
                SymbolEthTerm::from_declarative(db, declarative_term)?.into()
            }
            DecTerm::Rune(declarative_term) => {
                RuneEthTerm::from_declarative(db, declarative_term)?.into()
            }
            DecTerm::EntityPath(declarative_term) => match declarative_term {
                ItemPathDecTerm::Fugitive(path) => ItemPathTerm::Fugitive(path).into(),
                ItemPathDecTerm::Trait(path) => ItemPathTerm::Trait(path).into(),
                ItemPathDecTerm::Type(path) => match ty_expectation {
                    TermTypeExpectation::FinalDestinationEqsSort => {
                        ItemPathTerm::TypeOntology(path).into()
                    }
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path_expected) => {
                        if path_expected == path {
                            ItemPathTerm::TypeInstance(path).into()
                        } else {
                            return Err(EthTermError::ExpectFinalDestinationEqsNonSortTypePath {
                                path_expected,
                                path,
                            });
                        }
                    }
                    TermTypeExpectation::Any => ItemPathTerm::TypeInstance(path).into(),
                },
                ItemPathDecTerm::TypeVariant(path) => ItemPathTerm::TypeVariant(path).into(),
            },
            DecTerm::Category(declarative_term) => declarative_term.into(),
            DecTerm::Universe(declarative_term) => declarative_term.into(),
            DecTerm::Curry(declarative_term) => {
                CurryEthTerm::from_declarative(db, declarative_term)?.into()
            }
            DecTerm::Ritchie(declarative_term) => {
                RitchieEthTerm::from_declarative(db, declarative_term)?.into()
            }
            DecTerm::Abstraction(declarative_term) => {
                AbstractionEthTerm::from_declarative(db, declarative_term, ty_expectation)?.into()
            }
            DecTerm::Application(declarative_term) => {
                // todo: implicit arguments
                ApplicationEthTerm::from_declarative(db, declarative_term, ty_expectation)?
            }
            DecTerm::ApplicationOrRitchieCall(declarative_term) => {
                ethereal_term_from_application_or_ritchie_call_declarative_term(
                    db,
                    declarative_term,
                    ty_expectation,
                )?
            }
            DecTerm::AssociatedItem(declarative_term) => {
                todo!()
                // EthTermSubitem::from_declarative(db, declarative_term, ty_expectation)?
            }
            DecTerm::TypeAsTraitItem(declarative_term) => {
                TypeAsTraitItemEthTerm::from_declarative(db, declarative_term, ty_expectation)?
                    .into()
            }
            DecTerm::TraitConstraint(declarative_term) => {
                TraitConstraintEthTerm::from_declarative(db, declarative_term, ty_expectation)?
                    .into()
            }
            DecTerm::LeashOrBitNot(toolchain) => match ty_expectation {
                TermTypeExpectation::FinalDestinationEqsSort => {
                    db.ethereal_term_menu(toolchain).leash_ty_ontology()
                }
                TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path) => {
                    match path.prelude_ty_path(db) {
                        Some(PreludeTypePath::Num(_)) => {
                            todo!()
                        }
                        Some(_) | None => todo!(),
                    }
                }
                TermTypeExpectation::Any => todo!(),
            },
            DecTerm::List(declarative_term_list) => {
                ethereal_term_from_list_declarative_term(db, declarative_term_list, ty_expectation)?
            }
            DecTerm::Wrapper(declarative_term_wrapper) => {
                ethereal_term_from_declarative_term_wrapper(db, declarative_term_wrapper)?
            }
        })
    }

    pub(crate) fn into_declarative(self, db: &::salsa::Db) -> DecTerm {
        match self {
            EthTerm::Literal(slf) => LiteralDecTerm::Resolved(slf).into(),
            EthTerm::Symbol(slf) => SymbolDecTerm::new(
                db,
                slf.toolchain(db),
                Ok(slf.ty(db).into_declarative(db)),
                slf.index(db).into(),
            )
            .into(),
            EthTerm::Rune(slf) => slf.into_declarative(db).into(),
            EthTerm::EntityPath(slf) => slf.into(),
            EthTerm::Category(slf) => DecTerm::Category(slf),
            EthTerm::Universe(slf) => slf.into(),
            EthTerm::Curry(slf) => CurryDecTerm::new_inner(
                db,
                slf.toolchain(db),
                slf.curry_kind(db),
                slf.variance(db),
                slf.parameter_rune(db)
                    .map(|rune| rune.into_declarative(db).into()),
                slf.parameter_ty(db).into_declarative(db),
                slf.return_ty(db).into_declarative(db),
            )
            .into(),
            EthTerm::Ritchie(_) => todo!(),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::Application(_) => todo!(),
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        }
    }

    fn reduce(self, db: &::salsa::Db) -> Self {
        match self {
            EthTerm::Literal(_)
            | EthTerm::Symbol(_)
            | EthTerm::Rune(_)
            | EthTerm::EntityPath(
                ItemPathTerm::Trait(_)
                | ItemPathTerm::TypeOntology(_)
                | ItemPathTerm::TypeInstance(_)
                | ItemPathTerm::TypeVariant(_),
            )
            | EthTerm::Category(_)
            | EthTerm::Universe(_) => self,
            EthTerm::EntityPath(ItemPathTerm::Fugitive(_)) => todo!(),
            // ad hoc
            EthTerm::Curry(_) => self,
            EthTerm::Ritchie(slf) => slf.reduce(db).into(),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::Application(slf) => slf.reduce(db),
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn ethereal_term_from_application_or_ritchie_call_declarative_term(
    db: &::salsa::Db,
    declarative_term: ApplicationOrRitchieCallDecTerm,
    term_ty_expectation: TermTypeExpectation,
) -> EthTermResult<EthTerm> {
    let function =
        EthTerm::from_declarative(db, declarative_term.function(db), term_ty_expectation)?;
    match function.raw_ty(db)? {
        RawType::Declarative(DecTerm::Curry(curry)) => {
            let items = declarative_term.items(db);
            let argument = match items.len() {
                0 => db
                    .declarative_term_menu(curry.toolchain(db))
                    .unwrap()
                    .unit(),
                1 => items[0],
                _ => todo!(),
            };
            term_uncheck_from_declarative_term_application_aux(
                db,
                function,
                argument,
                term_ty_expectation,
            )
        }
        RawType::Declarative(DecTerm::Ritchie(ritchie)) => {
            todo!()
        }
        _ => todo!(),
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn ethereal_term_from_list_declarative_term(
    db: &::salsa::Db,
    list: ListDecTerm,
    term_ty_expectation: TermTypeExpectation,
) -> EthTermResult<EthTerm> {
    match term_ty_expectation {
        TermTypeExpectation::FinalDestinationEqsSort => {
            let toolchain = list.toolchain(db);
            let term_menu = db.ethereal_term_menu(toolchain);
            let items = list.items(db);
            match items.len() {
                0 => Ok(term_menu.list_ty_ontology()),
                1 => Ok(ApplicationEthTerm::new_reduced(
                    db,
                    term_menu.array_ty_ontology(),
                    EthTerm::from_declarative(
                        db,
                        items[0],
                        TermTypeExpectation::FinalDestinationEqsNonSortTypePath(
                            item_path_menu(db, toolchain).usize_ty_path(),
                        ),
                    )?,
                    0,
                )),
                _ => todo!(),
            }
        }
        TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path) => {
            match path.prelude_ty_path(db) {
                Some(PreludeTypePath::List) => {
                    todo!()
                }
                Some(PreludeTypePath::Container(_)) => {
                    todo!()
                }
                Some(_) | None => todo!(),
            }
        }
        TermTypeExpectation::Any => todo!(),
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn ethereal_term_from_declarative_term_wrapper(
    db: &::salsa::Db,
    wrapper: WrapperDecTerm,
) -> EthTermResult<EthTerm> {
    let inner_ty = EthTerm::ty_from_declarative(db, wrapper.inner_ty(db))?;
    match inner_ty.application_expansion(db).function() {
        TermFunctionReduced::TypeOntology(ty_path) => match ty_path.refine(db) {
            Left(PreludeTypePath::Num(_)) | Left(PreludeTypePath::Indirection(_)) => Ok(inner_ty),
            _ => {
                let Some(toolchain) = inner_ty.toolchain(db) else {
                    todo!()
                };
                let leash_ty_ontology = db.ethereal_term_menu(toolchain).leash_ty_ontology();
                Ok(ApplicationEthTerm::new_reduced(
                    db,
                    leash_ty_ontology,
                    inner_ty,
                    0,
                ))
            }
        },
        TermFunctionReduced::Trait(_) => todo!(),
        TermFunctionReduced::Other(_) => todo!(),
    }
}

#[test]
fn term_size_works() {
    // todo: shall we make this smaller?
    // to make this smaller, one can make TermLiteral smaller
    assert_eq!(std::mem::size_of::<EthTerm>(), 12);
    assert_eq!(
        std::mem::size_of::<Option<EthTerm>>(),
        std::mem::size_of::<EthTerm>()
    )
}

impl salsa::DebugWithDb for EthTerm {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("EthTerm(`{}`)", self.display_with(db,)))
    }
}

impl salsa::DisplayWithDb for EthTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl EthTerm {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            EthTerm::Literal(term) => term.show_with_db_fmt(f, db),
            EthTerm::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            EthTerm::Rune(term) => term.show_with_db_fmt(f, db, ctx),
            EthTerm::EntityPath(term) => term.show_with_db_fmt(f, db),
            EthTerm::Category(term) => f.write_str(&term.to_string()),
            EthTerm::Universe(term) => f.write_str(&term.to_string()),
            EthTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            EthTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            EthTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            EthTerm::Application(term) => term.show_with_db_fmt(f, db, ctx),
            EthTerm::TypeAsTraitItem(term) => term.show_with_db_fmt(f, db, ctx),
            EthTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}

/// # rewrite

impl EthTerm {
    pub fn substitute(self, substitution: EthTermSubstitution, db: &::salsa::Db) -> Self {
        match self {
            EthTerm::Literal(_)
            | EthTerm::EntityPath(_)
            | EthTerm::Category(_)
            | EthTerm::Universe(_) => self,
            EthTerm::Symbol(symbol) => todo!(),
            EthTerm::Rune(slf) => slf.substitute(substitution, db),
            EthTerm::Curry(slf) => slf.substitute(substitution, db).into(),
            EthTerm::Abstraction(slf) => slf.substitute(substitution, db).into(),
            EthTerm::Application(slf) => slf.substitute(substitution, db),
            EthTerm::TypeAsTraitItem(slf) => slf.substitute(substitution, db).into(),
            EthTerm::TraitConstraint(slf) => slf.substitute(substitution, db).into(),
            EthTerm::Ritchie(slf) => slf.substitute(substitution, db).into(),
        }
    }
}

impl EthTermInstantiate for EthTerm {
    type Output = EthTerm;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        match self {
            EthTerm::Literal(_)
            | EthTerm::EntityPath(_)
            | EthTerm::Category(_)
            | EthTerm::Universe(_) => self,
            EthTerm::Symbol(slf) => slf.instantiate(db, instantiation),
            EthTerm::Rune(slf) => slf.instantiate(db, instantiation).into(),
            EthTerm::Curry(slf) => slf.instantiate(db, instantiation).into(),
            EthTerm::Ritchie(slf) => slf.instantiate(db, instantiation).into(),
            EthTerm::Abstraction(slf) => slf.instantiate(db, instantiation).into(),
            EthTerm::Application(slf) => slf.instantiate(db, instantiation),
            EthTerm::TypeAsTraitItem(slf) => slf.instantiate(db, instantiation).into(),
            EthTerm::TraitConstraint(slf) => slf.instantiate(db, instantiation).into(),
        }
    }
}
