mod abstraction;
mod application;
mod as_trai_subentity;
mod constraint;
mod curry;
mod ritchie;
mod subentity;
mod symbol;

use std::fmt::{Debug, Display};

pub use self::abstraction::TermAbstraction;
pub use self::application::TermApplication;
pub use self::as_trai_subentity::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_precise_term::PreciseTerm;
use husky_raw_term::RawTerm;
use husky_ty_expectation::TermTypeExpectation;
use husky_valid_term::ValidTerm;
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
        ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        let precise_term = PreciseTerm::from_raw(db, raw_term, ty_expectation)?;
        let valid_term = ValidTerm::from_precise(db, precise_term)?;
        Ok(Self::from_valid(db, valid_term))
    }

    pub fn from_valid(db: &dyn TermDb, valid_term: ValidTerm) -> Self {
        match valid_term {
            ValidTerm::Literal(valid_term) => valid_term.into(),
            ValidTerm::Symbol(valid_term) => TermSymbol::from_valid(db, valid_term).into(),
            ValidTerm::EntityPath(valid_term) => valid_term.into(),
            ValidTerm::Category(valid_term) => valid_term.into(),
            ValidTerm::Universe(universe) => universe.into(),
            ValidTerm::Curry(valid_term) => TermCurry::from_valid(db, valid_term).into(),
            ValidTerm::Ritchie(valid_term) => TermRitchie::from_valid(db, valid_term).into(),
            ValidTerm::Abstraction(valid_term) => {
                TermAbstraction::from_valid(db, valid_term).into()
            }
            ValidTerm::Application(valid_term) => {
                TermApplication::from_valid(db, valid_term).into()
            }
            ValidTerm::Subentity(valid_term) => TermSubentity::from_valid(db, valid_term).into(),
            ValidTerm::AsTraitSubentity(valid_term) => {
                TermAsTraitSubentity::from_valid(db, valid_term).into()
            }
            ValidTerm::TraitConstraint(valid_term) => {
                TermTraitConstraint::from_valid(db, valid_term).into()
            }
        }
    }

    pub fn valid_ty(self) -> TermResult<Either<ValidTerm, PreludeTypePath>> {
        Ok(match self {
            Term::Literal(literal) => Right(literal.ty()),
            Term::Symbol(_) => todo!(),
            Term::Category(_) => todo!(),
            Term::EntityPath(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        })
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
            Term::Literal(term) => todo!(),
            //  term.show_with_db_fmt(f, db),
            Term::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            Term::EntityPath(term) => todo!(),
            // term.show_with_db_fmt(f, db),
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
