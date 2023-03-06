mod abstraction;
mod application;
mod as_trai_subentity;
mod constraint;
mod curry;
mod ritchie;
mod subentity;
mod symbol;
mod utils;

use std::fmt::{Debug, Display};

pub use self::abstraction::*;
pub use self::application::*;
pub use self::as_trai_subentity::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_word::Identifier;
use salsa::{DebugWithDb, DisplayWithDb};
use utils::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum RawTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(RawTermSymbol),
    Category(TermCategory),
    EntityPath(TermEntityPath),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(RawTermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(RawTermRitchie),
    /// lambda x => expr
    Abstraction(RawTermAbstraction),

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
    Application(RawTermApplication),
    /// ::<ident>
    Subentity(RawTermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(RawTermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(RawTermTraitConstraint),
}

impl RawTerm {
    pub fn from_precise(db: &dyn RawTermDb, precise_term: RawTerm) -> RawTermResult<Self> {
        Ok(match precise_term {
            RawTerm::Literal(literal) => literal.into(),
            // match raw_ty_expectation {
            //     TermTypeExpectation::FinalDestinationEqsSort => todo!(),
            //     TermTypeExpectation::FinalDestinationEqsNonSortTypePath(ty) => {
            //         if ty.prelude_ty_path(db)? != Some(literal.ty()) {
            //             todo!()
            //         }
            //     }
            //     TermTypeExpectation::Any => (),
            // }
            RawTerm::Symbol(precise_term) => RawTermSymbol::from_precise(db, precise_term)?.into(),
            RawTerm::EntityPath(precise_term) => precise_term.into(),
            RawTerm::Category(precise_term) => precise_term.into(),
            RawTerm::Universe(precise_term) => precise_term.into(),
            RawTerm::Curry(precise_term) => RawTermCurry::from_precise(db, precise_term)?.into(),
            RawTerm::Ritchie(precise_term) => {
                RawTermRitchie::from_precise(db, precise_term)?.into()
            }
            RawTerm::Abstraction(precise_term) => {
                RawTermAbstraction::from_precise(db, precise_term).into()
            }
            RawTerm::Application(precise_term) => {
                RawTermApplication::from_precise(db, precise_term)?.into()
            }
            RawTerm::Subentity(precise_term) => {
                RawTermSubentity::from_precise(db, precise_term).into()
            }
            RawTerm::AsTraitSubentity(precise_term) => {
                RawTermAsTraitSubentity::from_precise(db, precise_term).into()
            }
            RawTerm::TraitConstraint(precise_term) => {
                RawTermTraitConstraint::from_precise(db, precise_term).into()
            }
        })
    }

    pub fn ty(self, db: &dyn RawTermDb) -> RawTermResult<Either<RawTerm, PreludeTypePath>> {
        Ok(match self {
            RawTerm::Literal(literal) => Right(literal.ty()),
            RawTerm::Symbol(_) => todo!(),
            RawTerm::Category(_) => todo!(),
            RawTerm::EntityPath(path) => Left(match path {
                TermEntityPath::Form(path) => form_path_ty_unchecked(db, path)?,
                TermEntityPath::Trait(path) => trai_path_ty_unchecked(db, path)?,
                TermEntityPath::TypeOntology(path) => ty_ontology_path_ty_unchecked(db, path)?,
                TermEntityPath::TypeConstructor(path) => {
                    ty_constructor_path_ty_unchecked(db, path)?
                }
            }),
            RawTerm::Universe(_) => todo!(),
            RawTerm::Curry(_) => todo!(),
            RawTerm::Ritchie(_) => todo!(),
            RawTerm::Abstraction(_) => todo!(),
            RawTerm::Application(term) => Left(term.ty(db)?),
            RawTerm::Subentity(_) => todo!(),
            RawTerm::AsTraitSubentity(_) => todo!(),
            RawTerm::TraitConstraint(_) => todo!(),
        })
    }

    /// whether two types are trivially convertible
    pub fn is_ty_trivially_convertible_from(
        self,
        db: &dyn RawTermDb,
        other_ty: Either<Self, PreludeTypePath>,
    ) -> RawTermResult<bool> {
        match other_ty {
            Left(other_ty) if other_ty == self => Ok(true),
            Left(other_ty) => {
                todo!()
            }
            Right(other_ty) => match self {
                RawTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                    Ok(ty_path.prelude_ty_path(db)? == Some(other_ty))
                }
                _ => todo!(),
            },
        }
    }
}

#[test]
fn valid_term_size_works() {
    assert_eq!(
        std::mem::size_of::<RawTerm>(),
        2 * std::mem::size_of::<usize>()
    )
}

impl<Db: RawTermDb + ?Sized> salsa::DebugWithDb<Db> for RawTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!(
            "RawTerm(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl RawTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        match self {
            RawTerm::Literal(literal) => todo!(),
            // literal.show_with_db_fmt(f, db),
            RawTerm::Symbol(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            RawTerm::EntityPath(path) => todo!(),
            // path.show_with_db_fmt(f, db),
            RawTerm::Category(valid_term) => f.write_str(&valid_term.to_string()),
            RawTerm::Universe(valid_term) => f.write_str(&valid_term.to_string()),
            RawTerm::Curry(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Ritchie(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Abstraction(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Application(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Subentity(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            RawTerm::AsTraitSubentity(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            RawTerm::TraitConstraint(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
        }
    }
}
