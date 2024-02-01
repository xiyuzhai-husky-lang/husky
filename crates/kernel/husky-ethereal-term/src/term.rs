mod abstraction;
mod application;
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
use husky_declarative_term::term::DeclarativeTerm;
use husky_term_prelude::literal::TermLiteral;
use salsa::DisplayWithDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EtherealTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(TermLiteral),
    Symbol(SymbolEtherealTerm),
    /// the name `rune` is to be distinguishable from runtime variable
    Rune(RuneEtherealTerm),
    EntityPath(ItemPathTerm),
    Category(CategoryTerm),
    Universe(UniverseTerm),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(CurryEtherealTerm),
    /// in memory of Dennis M.Ritchie
    /// a type or trait
    Ritchie(RitchieEtherealTerm),
    /// lambda x => expr
    Abstraction(AbstractionEtherealTerm),

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
    Application(ApplicationEtherealTerm),
    /// (<type> as <trait>)::<ident>
    TypeAsTraitItem(TypeAsTraitItemEtherealTerm),
    /// <type> : <trait>
    TraitConstraint(TraitConstraintEtherealTerm),
}

impl EtherealTerm {
    #[track_caller]
    pub fn ty_from_declarative(
        db: &::salsa::Db,
        declarative_term: DeclarativeTerm,
    ) -> EtherealTermResult<Self> {
        let ty_term = Self::from_declarative(
            db,
            declarative_term,
            TermTypeExpectation::FinalDestinationEqsSort,
        )?;
        match ty_term.raw_ty(db)? {
            RawType::Declarative(DeclarativeTerm::Category(_)) => Ok(ty_term),
            _ => Err(EtherealTermError::ExpectedType {
                expectee: declarative_term,
            }),
        }
    }

    pub fn from_declarative(
        db: &::salsa::Db,
        declarative_term: DeclarativeTerm,
        ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
        Ok(match declarative_term {
            DeclarativeTerm::Literal(literal) => {
                EtherealTerm::from_literal_declarative_term(db, literal, ty_expectation)?
            }
            DeclarativeTerm::Symbol(declarative_term) => {
                SymbolEtherealTerm::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::Rune(declarative_term) => {
                RuneEtherealTerm::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::EntityPath(declarative_term) => match declarative_term {
                ItemPathDeclarativeTerm::Fugitive(path) => ItemPathTerm::Fugitive(path).into(),
                ItemPathDeclarativeTerm::Trait(path) => ItemPathTerm::Trait(path).into(),
                ItemPathDeclarativeTerm::Type(path) => match ty_expectation {
                    TermTypeExpectation::FinalDestinationEqsSort => {
                        ItemPathTerm::TypeOntology(path).into()
                    }
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path_expected) => {
                        if path_expected == path {
                            ItemPathTerm::TypeInstance(path).into()
                        } else {
                            return Err(
                                EtherealTermError::ExpectFinalDestinationEqsNonSortTypePath {
                                    path_expected,
                                    path,
                                },
                            );
                        }
                    }
                    TermTypeExpectation::Any => ItemPathTerm::TypeInstance(path).into(),
                },
                ItemPathDeclarativeTerm::TypeVariant(path) => {
                    ItemPathTerm::TypeVariant(path).into()
                }
            },
            DeclarativeTerm::Category(declarative_term) => declarative_term.into(),
            DeclarativeTerm::Universe(declarative_term) => declarative_term.into(),
            DeclarativeTerm::Curry(declarative_term) => {
                CurryEtherealTerm::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::Ritchie(declarative_term) => {
                RitchieEtherealTerm::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::Abstraction(declarative_term) => {
                AbstractionEtherealTerm::from_declarative(db, declarative_term, ty_expectation)?
                    .into()
            }
            DeclarativeTerm::Application(declarative_term) => {
                // todo: implicit arguments
                ApplicationEtherealTerm::from_declarative(db, declarative_term, ty_expectation)?
            }
            DeclarativeTerm::ApplicationOrRitchieCall(declarative_term) => {
                ethereal_term_from_application_or_ritchie_call_declarative_term(
                    db,
                    declarative_term,
                    ty_expectation,
                )?
            }
            DeclarativeTerm::AssociatedItem(declarative_term) => {
                todo!()
                // EtherealTermSubitem::from_declarative(db, declarative_term, ty_expectation)?
            }
            DeclarativeTerm::TypeAsTraitItem(declarative_term) => {
                TypeAsTraitItemEtherealTerm::from_declarative(db, declarative_term, ty_expectation)?
                    .into()
            }
            DeclarativeTerm::TraitConstraint(declarative_term) => {
                TraitConstraintEtherealTerm::from_declarative(db, declarative_term, ty_expectation)?
                    .into()
            }
            DeclarativeTerm::LeashOrBitNot(toolchain) => match ty_expectation {
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
            DeclarativeTerm::List(declarative_term_list) => {
                ethereal_term_from_list_declarative_term(db, declarative_term_list, ty_expectation)?
            }
            DeclarativeTerm::Wrapper(declarative_term_wrapper) => {
                ethereal_term_from_declarative_term_wrapper(db, declarative_term_wrapper)?
            }
        })
    }

    pub(crate) fn into_declarative(self, db: &::salsa::Db) -> DeclarativeTerm {
        match self {
            EtherealTerm::Literal(slf) => LiteralDeclarativeTerm::Resolved(slf).into(),
            EtherealTerm::Symbol(slf) => SymbolDeclarativeTerm::new(
                db,
                slf.toolchain(db),
                Ok(slf.ty(db).into_declarative(db)),
                slf.index(db).into(),
            )
            .into(),
            EtherealTerm::Rune(slf) => slf.into_declarative(db).into(),
            EtherealTerm::EntityPath(slf) => slf.into(),
            EtherealTerm::Category(slf) => DeclarativeTerm::Category(slf),
            EtherealTerm::Universe(slf) => slf.into(),
            EtherealTerm::Curry(slf) => CurryDeclarativeTerm::new_inner(
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
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::TypeAsTraitItem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    fn reduce(self, db: &::salsa::Db) -> Self {
        match self {
            EtherealTerm::Literal(_)
            | EtherealTerm::Symbol(_)
            | EtherealTerm::Rune(_)
            | EtherealTerm::EntityPath(
                ItemPathTerm::Trait(_)
                | ItemPathTerm::TypeOntology(_)
                | ItemPathTerm::TypeInstance(_)
                | ItemPathTerm::TypeVariant(_),
            )
            | EtherealTerm::Category(_)
            | EtherealTerm::Universe(_) => self,
            EtherealTerm::EntityPath(ItemPathTerm::Fugitive(_)) => todo!(),
            // ad hoc
            EtherealTerm::Curry(_) => self,
            EtherealTerm::Ritchie(slf) => slf.reduce(db).into(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(slf) => slf.reduce(db),
            EtherealTerm::TypeAsTraitItem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_from_application_or_ritchie_call_declarative_term(
    db: &::salsa::Db,
    declarative_term: ApplicationOrRitchieCallDeclarativeTerm,
    term_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    let function =
        EtherealTerm::from_declarative(db, declarative_term.function(db), term_ty_expectation)?;
    match function.raw_ty(db)? {
        RawType::Declarative(DeclarativeTerm::Curry(curry)) => {
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
        RawType::Declarative(DeclarativeTerm::Ritchie(ritchie)) => {
            todo!()
        }
        _ => todo!(),
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_from_list_declarative_term(
    db: &::salsa::Db,
    list: ListDeclarativeTerm,
    term_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    match term_ty_expectation {
        TermTypeExpectation::FinalDestinationEqsSort => {
            let toolchain = list.toolchain(db);
            let term_menu = db.ethereal_term_menu(toolchain);
            let items = list.items(db);
            match items.len() {
                0 => Ok(term_menu.list_ty_ontology()),
                1 => Ok(ApplicationEtherealTerm::new_reduced(
                    db,
                    term_menu.array_ty_ontology(),
                    EtherealTerm::from_declarative(
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

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_from_declarative_term_wrapper(
    db: &::salsa::Db,
    wrapper: WrapperDeclarativeTerm,
) -> EtherealTermResult<EtherealTerm> {
    let inner_ty = EtherealTerm::ty_from_declarative(db, wrapper.inner_ty(db))?;
    match inner_ty.application_expansion(db).function() {
        TermFunctionReduced::TypeOntology(ty_path) => match ty_path.refine(db) {
            Left(PreludeTypePath::Num(_)) | Left(PreludeTypePath::Indirection(_)) => Ok(inner_ty),
            _ => {
                let Some(toolchain) = inner_ty.toolchain(db) else {
                    todo!()
                };
                let leash_ty_ontology = db.ethereal_term_menu(toolchain).leash_ty_ontology();
                Ok(ApplicationEtherealTerm::new_reduced(
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
    assert_eq!(std::mem::size_of::<EtherealTerm>(), 12);
    assert_eq!(
        std::mem::size_of::<Option<EtherealTerm>>(),
        std::mem::size_of::<EtherealTerm>()
    )
}

impl salsa::DebugWithDb for EtherealTerm {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("EtherealTerm(`{}`)", self.display_with(db,)))
    }
}

impl salsa::DisplayWithDb for EtherealTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl EtherealTerm {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            EtherealTerm::Literal(term) => term.show_with_db_fmt(f, db),
            EtherealTerm::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Rune(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::EntityPath(term) => term.show_with_db_fmt(f, db),
            EtherealTerm::Category(term) => f.write_str(&term.to_string()),
            EtherealTerm::Universe(term) => f.write_str(&term.to_string()),
            EtherealTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Application(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::TypeAsTraitItem(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}

/// # rewrite

impl EtherealTerm {
    pub fn substitute(self, substitution: EtherealTermSubstitution, db: &::salsa::Db) -> Self {
        match self {
            EtherealTerm::Literal(_)
            | EtherealTerm::EntityPath(_)
            | EtherealTerm::Category(_)
            | EtherealTerm::Universe(_) => self,
            EtherealTerm::Symbol(symbol) => todo!(),
            EtherealTerm::Rune(slf) => slf.substitute(substitution, db),
            EtherealTerm::Curry(slf) => slf.substitute(substitution, db).into(),
            EtherealTerm::Abstraction(slf) => slf.substitute(substitution, db).into(),
            EtherealTerm::Application(slf) => slf.substitute(substitution, db),
            EtherealTerm::TypeAsTraitItem(slf) => slf.substitute(substitution, db).into(),
            EtherealTerm::TraitConstraint(slf) => slf.substitute(substitution, db).into(),
            EtherealTerm::Ritchie(slf) => slf.substitute(substitution, db).into(),
        }
    }
}

impl EtherealTermInstantiate for EtherealTerm {
    type Output = EtherealTerm;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        match self {
            EtherealTerm::Literal(_)
            | EtherealTerm::EntityPath(_)
            | EtherealTerm::Category(_)
            | EtherealTerm::Universe(_) => self,
            EtherealTerm::Symbol(slf) => slf.instantiate(db, instantiation),
            EtherealTerm::Rune(slf) => slf.instantiate(db, instantiation).into(),
            EtherealTerm::Curry(slf) => slf.instantiate(db, instantiation).into(),
            EtherealTerm::Ritchie(slf) => slf.instantiate(db, instantiation).into(),
            EtherealTerm::Abstraction(slf) => slf.instantiate(db, instantiation).into(),
            EtherealTerm::Application(slf) => slf.instantiate(db, instantiation),
            EtherealTerm::TypeAsTraitItem(slf) => slf.instantiate(db, instantiation).into(),
            EtherealTerm::TraitConstraint(slf) => slf.instantiate(db, instantiation).into(),
        }
    }
}
