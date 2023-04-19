mod abstraction;
mod application;
mod as_trai_subentity;
mod constraint;
mod curry;
mod placeholder;
mod ritchie;
mod subentity;
mod symbol;

use std::fmt::{Debug, Display};

pub use self::abstraction::*;
pub use self::application::*;
pub use self::as_trai_subentity::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::placeholder::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use husky_declarative_term::DeclarativeTerm;
use husky_declarative_ty::{ty_constructor_path_declarative_ty, ty_ontology_path_declarative_ty};
use husky_entity_path::EntityPath;
use husky_ty_expectation::TermTypeExpectation;
use husky_word::Ident;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EtherealTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(EtherealTermSymbol),
    Placeholder(EtherealTermPlaceholder),
    EntityPath(TermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(EtherealTermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(EtherealTermRitchie),
    /// lambda x => expr
    Abstraction(EtherealTermAbstraction),

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
    Application(EtherealTermApplication),
    /// ::<ident>
    Subentity(EtherealTermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(EtherealTermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(EtherealTermTraitConstraint),
}

impl EtherealTerm {
    pub fn from_raw(
        db: &dyn EtherealTermDb,
        raw_term: DeclarativeTerm,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        Self::from_raw_unchecked(db, raw_term, term_ty_expectation)
    }

    pub fn ty_from_raw(db: &dyn EtherealTermDb, raw_term: DeclarativeTerm) -> TermResult<Self> {
        Self::from_raw(db, raw_term, TermTypeExpectation::FinalDestinationEqsSort)
    }

    pub fn ty_from_raw_unchecked(
        db: &dyn EtherealTermDb,
        raw_term: DeclarativeTerm,
    ) -> TermResult<Self> {
        Self::from_raw_unchecked(db, raw_term, TermTypeExpectation::FinalDestinationEqsSort)
    }

    pub(crate) fn from_raw_unchecked(
        db: &dyn EtherealTermDb,
        raw_term: DeclarativeTerm,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        Ok(match raw_term {
            DeclarativeTerm::Literal(literal) => {
                match literal {
                    DeclarativeTermLiteral::Resolved(literal) => literal.into(),
                    DeclarativeTermLiteral::Unresolved(_) => todo!(),
                }
                //  TermLiteral::from_raw_unchecked(db, raw_term, ty_expectation)?.into()
            }
            DeclarativeTerm::Symbol(raw_term) => {
                EtherealTermSymbol::from_raw_unchecked(db, raw_term)?.into()
            }
            DeclarativeTerm::Hole(_) => todo!(),
            DeclarativeTerm::EntityPath(raw_term) => match raw_term {
                DeclarativeTermEntityPath::Form(path) => TermEntityPath::Form(path).into(),
                DeclarativeTermEntityPath::Trait(path) => TermEntityPath::Trait(path).into(),
                DeclarativeTermEntityPath::Type(path) => match term_ty_expectation {
                    TermTypeExpectation::FinalDestinationEqsSort => {
                        TermEntityPath::TypeOntology(path).into()
                    }
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path_expected) => {
                        if path_expected == path {
                            TermEntityPath::TypeConstructor(path).into()
                        } else {
                            return Err(TermError::ExpectFinalDestinationEqsNonSortTypePath {
                                path_expected,
                                path,
                            });
                        }
                    }
                    TermTypeExpectation::Any => TermEntityPath::TypeConstructor(path).into(),
                },
            },
            DeclarativeTerm::Category(raw_term) => raw_term.into(),
            DeclarativeTerm::Universe(raw_term) => raw_term.into(),
            DeclarativeTerm::Curry(raw_term) => {
                EtherealTermCurry::from_raw_unchecked(db, raw_term)?.into()
            }
            DeclarativeTerm::Ritchie(raw_term) => {
                EtherealTermRitchie::from_raw_unchecked(db, raw_term)?.into()
            }
            DeclarativeTerm::Abstraction(raw_term) => {
                EtherealTermAbstraction::from_raw_unchecked(db, raw_term, term_ty_expectation)?
                    .into()
            }
            DeclarativeTerm::ExplicitApplication(raw_term) => {
                // todo: implicit arguments
                EtherealTermApplication::from_raw_unchecked(db, raw_term, term_ty_expectation)?
            }
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(raw_term) => {
                term_from_raw_term_explicit_application_or_ritchie_call_unchecked(
                    db,
                    raw_term,
                    term_ty_expectation,
                )?
            }
            DeclarativeTerm::Subentity(raw_term) => {
                EtherealTermSubentity::from_raw_unchecked(db, raw_term, term_ty_expectation)?
            }
            DeclarativeTerm::AsTraitSubentity(raw_term) => {
                EtherealTermAsTraitSubentity::from_raw_unchecked(db, raw_term, term_ty_expectation)?
                    .into()
            }
            DeclarativeTerm::TraitConstraint(raw_term) => {
                EtherealTermTraitConstraint::from_raw_unchecked(db, raw_term, term_ty_expectation)?
                    .into()
            }
            DeclarativeTerm::LeashOrBitNot(toolchain) => match term_ty_expectation {
                TermTypeExpectation::FinalDestinationEqsSort => {
                    db.term_menu(toolchain).leash_ty_ontology()
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
            DeclarativeTerm::List(raw_term_list) => {
                term_from_raw_term_list_unchecked(db, raw_term_list, term_ty_expectation)?
            }
        })
    }

    pub fn from_raw_inner(db: &dyn EtherealTermDb, valid_term: DeclarativeTerm) -> Self {
        todo!()
    }

    fn reduce(self, db: &dyn EtherealTermDb) -> Self {
        match self {
            EtherealTerm::Literal(_)
            | EtherealTerm::Symbol(_)
            | EtherealTerm::Placeholder(_)
            | EtherealTerm::EntityPath(
                TermEntityPath::Trait(_)
                | TermEntityPath::TypeOntology(_)
                | TermEntityPath::TypeConstructor(_),
            )
            | EtherealTerm::Category(_)
            | EtherealTerm::Universe(_) => self,
            EtherealTerm::EntityPath(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => term.reduce(db),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub(crate) fn is_reduced(self, db: &dyn EtherealTermDb) -> bool {
        self.reduce(db) == self
    }

    pub fn substitute(self, db: &dyn EtherealTermDb, substitution: &TermSubstitution) -> Self {
        todo!()
        // match self {
        //     EtherealTerm::Symbol(symbol) => match symbol == substitution.src() {
        //         true => substitution.dst(),
        //         false => self,
        //     },
        //     EtherealTerm::Literal(_) | EtherealTerm::EntityPath(_) | EtherealTerm::Category(_) | EtherealTerm::Universe(_) => self,
        //     EtherealTerm::Curry(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::Abstraction(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::Application(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::Subentity(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::AsTraitSubentity(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::TraitConstraint(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::Ritchie(_) => todo!(),
        // }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_from_raw_term_explicit_application_or_ritchie_call_unchecked(
    db: &dyn EtherealTermDb,
    raw_term: DeclarativeTermExplicitApplicationOrRitchieCall,
    term_ty_expectation: TermTypeExpectation,
) -> TermResult<EtherealTerm> {
    let function =
        EtherealTerm::from_raw_unchecked(db, raw_term.function(db), term_ty_expectation)?;
    match function.declarative_ty(db)? {
        Left(declarative_ty) => match declarative_ty {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Symbol(_) => todo!(),
            DeclarativeTerm::Hole(_) => todo!(),
            DeclarativeTerm::EntityPath(_) => todo!(),
            DeclarativeTerm::Category(_) => todo!(),
            DeclarativeTerm::Universe(_) => todo!(),
            DeclarativeTerm::Curry(_) => {
                let items = raw_term.items(db);
                let argument = match items.len() {
                    0 => unreachable!(),
                    1 => items[0],
                    _ => todo!(),
                };
                term_uncheck_from_raw_term_application_aux(
                    db,
                    function,
                    argument,
                    term_ty_expectation,
                )
            }
            DeclarativeTerm::Ritchie(_) => todo!(),
            DeclarativeTerm::Abstraction(_) => todo!(),
            DeclarativeTerm::ExplicitApplication(_) => todo!(),
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            DeclarativeTerm::Subentity(_) => todo!(),
            DeclarativeTerm::AsTraitSubentity(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
        },
        Right(_) => todo!(),
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_from_raw_term_list_unchecked(
    db: &dyn EtherealTermDb,
    raw_term_list: DeclarativeTermList,
    term_ty_expectation: TermTypeExpectation,
) -> TermResult<EtherealTerm> {
    match term_ty_expectation {
        TermTypeExpectation::FinalDestinationEqsSort => {
            let term_menu = db.term_menu(raw_term_list.toolchain(db));
            let items = raw_term_list.items(db);
            match items.len() {
                0 => Ok(term_menu.list_ty_ontology()),
                _ => todo!(),
            }
        }
        TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path) => {
            match path.prelude_ty_path(db) {
                Some(PreludeTypePath::List) => {
                    todo!()
                }
                Some(PreludeTypePath::Array) => {
                    todo!()
                }
                Some(PreludeTypePath::Array2d) => {
                    todo!()
                }
                Some(PreludeTypePath::Array3d) => {
                    todo!()
                }
                Some(PreludeTypePath::Array4d) => {
                    todo!()
                }
                Some(PreludeTypePath::Array5d) => {
                    todo!()
                }
                Some(_) | None => todo!(),
            }
        }
        TermTypeExpectation::Any => todo!(),
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

impl<Db: EtherealTermDb + ?Sized> salsa::DebugWithDb<Db> for EtherealTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!(
            "EtherealTerm(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: EtherealTermDb + ?Sized> salsa::DisplayWithDb<Db> for EtherealTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl EtherealTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            EtherealTerm::Literal(term) => term.show_with_db_fmt(f, db),
            EtherealTerm::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Placeholder(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::EntityPath(term) => term.show_with_db_fmt(f, db),
            EtherealTerm::Category(term) => f.write_str(&term.to_string()),
            EtherealTerm::Universe(term) => f.write_str(&term.to_string()),
            EtherealTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Application(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Subentity(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::AsTraitSubentity(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
