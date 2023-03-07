mod abstraction;
mod application;
mod as_trai_subentity;
mod constraint;
mod curry;
mod entity_path;
mod ritchie;
mod subentity;
mod symbol;

use std::fmt::{Debug, Display};

pub use self::abstraction::*;
pub use self::application::*;
pub use self::as_trai_subentity::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::entity_path::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_raw_term::RawTerm;
use husky_raw_ty::{ty_constructor_path_raw_ty, ty_ontology_path_raw_ty};
use husky_ty_expectation::TermTypeExpectation;
use husky_word::Identifier;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum Term {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(TermSymbol),
    EntityPath(TermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(TermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(TermRitchie),
    /// lambda x => expr
    Abstraction(TermAbstraction),

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
    Application(TermApplication),
    /// ::<ident>
    Subentity(TermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(TermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(TermTraitConstraint),
}

impl Term {
    pub fn from_raw(
        db: &dyn TermDb,
        raw_term: RawTerm,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        let term = Self::from_raw_unchecked(db, raw_term, term_ty_expectation)?;
        term.check(db)?;
        Ok(term)
    }

    fn check(self, db: &dyn TermDb) -> TermResult<()> {
        match self {
            Term::Literal(_) => Ok(()),
            Term::Symbol(term) => term.check(db),
            Term::EntityPath(_) => Ok(()),
            Term::Category(_) => Ok(()),
            Term::Universe(_) => Ok(()),
            Term::Curry(term) => term.check(db),
            Term::Ritchie(term) => term.check(db),
            Term::Abstraction(term) => term.check(db),
            Term::Application(term) => term.check(db),
            Term::Subentity(term) => term.check(db),
            Term::AsTraitSubentity(term) => term.check(db),
            Term::TraitConstraint(term) => term.check(db),
        }
    }

    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        raw_term: RawTerm,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        Ok(match raw_term {
            RawTerm::Literal(literal) => {
                match literal {
                    RawTermLiteral::Resolved(literal) => literal.into(),
                    RawTermLiteral::Unresolved(_) => todo!(),
                }
                //  TermLiteral::from_raw_unchecked(db, raw_term, ty_expectation)?.into()
            }
            RawTerm::Symbol(raw_term) => TermSymbol::from_raw_unchecked(db, raw_term)?.into(),
            RawTerm::EntityPath(raw_term) => match raw_term {
                RawTermEntityPath::Form(path) => TermEntityPath::Form(path).into(),
                RawTermEntityPath::Trait(path) => TermEntityPath::Trait(path).into(),
                RawTermEntityPath::Type(path) => match term_ty_expectation {
                    TermTypeExpectation::FinalDestinationEqsSort => {
                        TermEntityPath::TypeOntology(path).into()
                    }
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path_expected)
                        if path_expected == path =>
                    {
                        TermEntityPath::TypeConstructor(path).into()
                    }
                    TermTypeExpectation::Any => TermEntityPath::TypeConstructor(path).into(),
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(_) => {
                        return Err(todo!())
                    }
                },
            },
            RawTerm::Category(raw_term) => raw_term.into(),
            RawTerm::Universe(raw_term) => raw_term.into(),
            RawTerm::Curry(raw_term) => TermCurry::from_raw_unchecked(db, raw_term)?.into(),
            RawTerm::Ritchie(raw_term) => TermRitchie::from_raw_unchecked(db, raw_term)?.into(),
            RawTerm::Abstraction(raw_term) => {
                TermAbstraction::from_raw_unchecked(db, raw_term, term_ty_expectation)?.into()
            }
            RawTerm::Application(raw_term) => {
                TermApplication::from_raw_unchecked(db, raw_term, term_ty_expectation)?
            }
            RawTerm::Subentity(raw_term) => {
                TermSubentity::from_raw_unchecked(db, raw_term, term_ty_expectation)?
            }
            RawTerm::AsTraitSubentity(raw_term) => {
                TermAsTraitSubentity::from_raw_unchecked(db, raw_term, term_ty_expectation)?.into()
            }
            RawTerm::TraitConstraint(raw_term) => {
                TermTraitConstraint::from_raw_unchecked(db, raw_term, term_ty_expectation)?.into()
            }
            RawTerm::LeashOrBitNot(_) => match term_ty_expectation {
                TermTypeExpectation::FinalDestinationEqsSort | TermTypeExpectation::Any => {
                    todo!()
                }
                TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path) => {
                    match path.prelude_ty_path(db)? {
                        Some(PreludeTypePath::I8 | PreludeTypePath::I8 | PreludeTypePath::I8) => {
                            todo!()
                        }
                        Some(_) | None => todo!(),
                    }
                }
            },
        })
    }

    pub fn from_raw_inner(db: &dyn TermDb, valid_term: RawTerm) -> Self {
        todo!()
    }

    fn ty_unchecked(self, db: &dyn TermDb) -> TermResult<Either<Term, PreludeTypePath>> {
        Ok(match self.raw_ty(db)? {
            Left(raw_ty) => Left(Term::from_raw_unchecked(
                db,
                raw_ty,
                TermTypeExpectation::FinalDestinationEqsSort,
            )?),
            Right(prelude_ty_path) => Right(prelude_ty_path),
        })
    }

    fn raw_ty(self, db: &dyn TermDb) -> TermResult<Either<RawTerm, PreludeTypePath>> {
        Ok(match self {
            Term::Literal(literal) => Right(literal.ty()),
            // term.raw_ty(db),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => Left(ty_ontology_path_raw_ty(db, path)?),
                TermEntityPath::TypeConstructor(path) => {
                    Left(ty_constructor_path_raw_ty(db, path)?)
                }
            },
            Term::Category(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(term) => Left(term.raw_ty(db)?),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        })
    }

    /// whether two types are trivially convertible
    pub fn is_ty_trivially_convertible_from(
        self,
        db: &dyn TermDb,
        other_ty: Either<Self, PreludeTypePath>,
    ) -> TermResult<bool> {
        match other_ty {
            Left(other_ty) if other_ty == self => Ok(true),
            Left(other_ty) => {
                todo!()
            }
            Right(other_ty) => match self {
                Term::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                    Ok(ty_path.prelude_ty_path(db)? == Some(other_ty))
                }
                _ => todo!(),
            },
        }
    }

    fn reduce(self, db: &dyn TermDb) -> Self {
        match self {
            Term::Literal(_)
            | Term::Symbol(_)
            | Term::EntityPath(
                TermEntityPath::Trait(_)
                | TermEntityPath::TypeOntology(_)
                | TermEntityPath::TypeConstructor(_),
            )
            | Term::Category(_)
            | Term::Universe(_) => self,
            Term::EntityPath(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(term) => term.reduce(db),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }

    pub fn substitute(self, db: &dyn TermDb, substitution: &TermSubstitution) -> Self {
        todo!()
        // match self {
        //     Term::Symbol(symbol) => match symbol == substitution.src() {
        //         true => substitution.dst(),
        //         false => self,
        //     },
        //     Term::Literal(_) | Term::EntityPath(_) | Term::Category(_) | Term::Universe(_) => self,
        //     Term::Curry(term) => term.substitute(db, substitution).into(),
        //     Term::Abstraction(term) => term.substitute(db, substitution).into(),
        //     Term::Application(term) => term.substitute(db, substitution).into(),
        //     Term::Subentity(term) => term.substitute(db, substitution).into(),
        //     Term::AsTraitSubentity(term) => term.substitute(db, substitution).into(),
        //     Term::TraitConstraint(term) => term.substitute(db, substitution).into(),
        //     Term::Ritchie(_) => todo!(),
        // }
    }
}

#[test]
fn term_size_works() {
    assert_eq!(
        std::mem::size_of::<Term>(),
        2 * std::mem::size_of::<usize>()
    );
    assert_eq!(
        std::mem::size_of::<Option<Term>>(),
        std::mem::size_of::<Term>()
    )
}

impl<Db: TermDb + ?Sized> salsa::DebugWithDb<Db> for Term {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        f.write_fmt(format_args!(
            "Term(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for Term {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl Term {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            Term::Literal(term) => term.show_with_db_fmt(f, db),
            Term::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            Term::EntityPath(term) => term.show_with_db_fmt(f, db),
            Term::Category(term) => f.write_str(&term.to_string()),
            Term::Universe(term) => f.write_str(&term.to_string()),
            Term::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Application(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Subentity(term) => term.show_with_db_fmt(f, db, ctx),
            Term::AsTraitSubentity(term) => term.show_with_db_fmt(f, db, ctx),
            Term::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
