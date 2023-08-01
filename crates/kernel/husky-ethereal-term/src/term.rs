mod abstraction;
mod application;
mod as_trai_item;
mod constraint;
mod curry;
mod item;
mod ritchie;
mod symbol;
mod variable;

use std::fmt::{Debug, Display};

pub use self::abstraction::*;
pub use self::application::*;
pub use self::as_trai_item::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::item::*;
pub use self::ritchie::*;
pub use self::symbol::*;
pub use self::variable::*;

use crate::instantiation::*;
use crate::*;
use husky_coword::Ident;
use husky_declarative_term::DeclarativeTerm;
use husky_declarative_ty::{
    ty_instance_constructor_path_declarative_ty, ty_ontology_path_declarative_ty,
};
use husky_entity_path::ItemPath;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EtherealTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(TermLiteral),
    Symbol(EtherealTermSymbol),
    Variable(EtherealTermVariable),
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
    Subitem(EtherealTermSubitem),
    /// (<type> as <trait>)::<ident>
    AsTraitSubitem(EtherealTermAsTraitSubitem),
    /// <type> : <trait>
    TraitConstraint(EtherealTermTraitConstraint),
}

impl EtherealTerm {
    pub fn ty_from_declarative(
        db: &dyn EtherealTermDb,
        declarative_term: DeclarativeTerm,
    ) -> EtherealTermResult<Self> {
        Self::from_declarative(
            db,
            declarative_term,
            TermTypeExpectation::FinalDestinationEqsSort,
        )
    }

    pub fn from_declarative(
        db: &dyn EtherealTermDb,
        declarative_term: DeclarativeTerm,
        ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
        Ok(match declarative_term {
            DeclarativeTerm::Literal(literal) => {
                match literal {
                    DeclarativeTermLiteral::Resolved(literal) => literal.into(),
                    DeclarativeTermLiteral::Unresolved(literal) => match literal {
                        UnresolvedTermLiteral::RegularInteger(i) => match ty_expectation {
                            TermTypeExpectation::FinalDestinationEqsSort => todo!(),
                            TermTypeExpectation::FinalDestinationEqsNonSortTypePath(ty_path) => {
                                match ty_path.prelude_ty_path(db) {
                                    Some(prelude_ty_path) => match prelude_ty_path {
                                        PreludeTypePath::Num(num_ty_path) => match num_ty_path {
                                            PreludeNumTypePath::Int(int_ty_path) => {
                                                match int_ty_path {
                                                    PreludeIntTypePath::I8 => match i.try_into() {
                                                        Ok::<i8, _>(i) => TermLiteral::I8(i).into(),
                                                        Err(_) => todo!(),
                                                    },
                                                    PreludeIntTypePath::I16 => match i.try_into() {
                                                        Ok::<i16, _>(i) => {
                                                            TermLiteral::I16(i).into()
                                                        }
                                                        Err(_) => todo!(),
                                                    },
                                                    PreludeIntTypePath::I32 => match i.try_into() {
                                                        Ok::<i32, _>(i) => {
                                                            TermLiteral::I32(i).into()
                                                        }
                                                        Err(_) => todo!(),
                                                    },
                                                    PreludeIntTypePath::I64 => match i.try_into() {
                                                        Ok::<i64, _>(i) => {
                                                            todo!()
                                                            // TermLiteral::I64(i).into()
                                                        }
                                                        Err(_) => todo!(),
                                                    },
                                                    PreludeIntTypePath::I128 => {
                                                        match i.try_into() {
                                                            Ok::<i128, _>(i) => {
                                                                todo!()
                                                                // TermLiteral::I128(i).into()
                                                            }
                                                            Err(_) => todo!(),
                                                        }
                                                    }
                                                    PreludeIntTypePath::ISize => match i.try_into()
                                                    {
                                                        Ok::<isize, _>(i) => {
                                                            todo!()
                                                            // TermLiteral::ISize(i).into()
                                                        }
                                                        Err(_) => todo!(),
                                                    },
                                                    PreludeIntTypePath::U8 => match i.try_into() {
                                                        Ok::<u8, _>(i) => TermLiteral::U8(i).into(),
                                                        Err(_) => todo!(),
                                                    },
                                                    PreludeIntTypePath::U16 => todo!(),
                                                    PreludeIntTypePath::U32 => todo!(),
                                                    PreludeIntTypePath::U64 => todo!(),
                                                    PreludeIntTypePath::U128 => todo!(),
                                                    PreludeIntTypePath::USize => todo!(),
                                                    PreludeIntTypePath::R8 => todo!(),
                                                    PreludeIntTypePath::R16 => todo!(),
                                                    PreludeIntTypePath::R32 => todo!(),
                                                    PreludeIntTypePath::R64 => todo!(),
                                                    PreludeIntTypePath::R128 => todo!(),
                                                    PreludeIntTypePath::RSize => todo!(),
                                                }
                                            }
                                            PreludeNumTypePath::Float(_) => todo!(),
                                        },
                                        _ => todo!(),
                                    },
                                    None => todo!(),
                                }
                            }
                            TermTypeExpectation::Any => todo!(),
                        },
                    },
                }
                //  TermLiteral::from_declarative(db, declarative_term, ty_expectation)?.into()
            }
            DeclarativeTerm::Symbol(declarative_term) => {
                EtherealTermSymbol::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::Variable(declarative_term) => {
                EtherealTermVariable::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::EntityPath(declarative_term) => match declarative_term {
                DeclarativeTermEntityPath::Fugitive(path) => TermEntityPath::Fugitive(path).into(),
                DeclarativeTermEntityPath::Trait(path) => TermEntityPath::Trait(path).into(),
                DeclarativeTermEntityPath::Type(path) => match ty_expectation {
                    TermTypeExpectation::FinalDestinationEqsSort => {
                        TermEntityPath::TypeOntology(path).into()
                    }
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path_expected) => {
                        if path_expected == path {
                            TermEntityPath::TypeInstance(path).into()
                        } else {
                            return Err(
                                EtherealTermError::ExpectFinalDestinationEqsNonSortTypePath {
                                    path_expected,
                                    path,
                                },
                            );
                        }
                    }
                    TermTypeExpectation::Any => TermEntityPath::TypeInstance(path).into(),
                },
                DeclarativeTermEntityPath::TypeVariant(path) => {
                    TermEntityPath::TypeVariant(path).into()
                }
            },
            DeclarativeTerm::Category(declarative_term) => declarative_term.into(),
            DeclarativeTerm::Universe(declarative_term) => declarative_term.into(),
            DeclarativeTerm::Curry(declarative_term) => {
                EtherealTermCurry::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::Ritchie(declarative_term) => {
                EtherealTermRitchie::from_declarative(db, declarative_term)?.into()
            }
            DeclarativeTerm::Abstraction(declarative_term) => {
                EtherealTermAbstraction::from_declarative(db, declarative_term, ty_expectation)?
                    .into()
            }
            DeclarativeTerm::ExplicitApplication(declarative_term) => {
                // todo: implicit arguments
                EtherealTermApplication::from_declarative(db, declarative_term, ty_expectation)?
            }
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(declarative_term) => {
                ethereal_term_from_declarative_term_explicit_application_or_ritchie_call(
                    db,
                    declarative_term,
                    ty_expectation,
                )?
            }
            DeclarativeTerm::Subitem(declarative_term) => {
                EtherealTermSubitem::from_declarative(db, declarative_term, ty_expectation)?
            }
            DeclarativeTerm::AsTraitSubitem(declarative_term) => {
                EtherealTermAsTraitSubitem::from_declarative(db, declarative_term, ty_expectation)?
                    .into()
            }
            DeclarativeTerm::TraitConstraint(declarative_term) => {
                EtherealTermTraitConstraint::from_declarative(db, declarative_term, ty_expectation)?
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
                ethereal_term_from_declarative_term_list(db, declarative_term_list, ty_expectation)?
            }
            DeclarativeTerm::Wrapper(declarative_term_wrapper) => {
                ethereal_term_from_declarative_term_wrapper(db, declarative_term_wrapper)?
            }
        })
    }

    pub(crate) fn into_declarative(self, db: &dyn EtherealTermDb) -> DeclarativeTerm {
        match self {
            EtherealTerm::Literal(lit) => DeclarativeTermLiteral::Resolved(lit).into(),
            EtherealTerm::Symbol(s) => DeclarativeTermSymbol::new(
                db,
                Ok(s.ty(db).into_declarative(db)),
                s.index(db).into(),
            )
            .into(),
            EtherealTerm::Variable(v) => {
                DeclarativeTermVariable::new(db, Ok(v.ty(db).into_declarative(db)), v.idx(db))
                    .into()
            }
            EtherealTerm::EntityPath(path) => path.into(),
            EtherealTerm::Category(cat) => DeclarativeTerm::Category(cat),
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

    pub fn from_raw_inner(db: &dyn EtherealTermDb, valid_term: DeclarativeTerm) -> Self {
        todo!()
    }

    fn reduce(self, db: &dyn EtherealTermDb) -> Self {
        match self {
            EtherealTerm::Literal(_)
            | EtherealTerm::Symbol(_)
            | EtherealTerm::Variable(_)
            | EtherealTerm::EntityPath(
                TermEntityPath::Trait(_)
                | TermEntityPath::TypeOntology(_)
                | TermEntityPath::TypeInstance(_)
                | TermEntityPath::TypeVariant(_),
            )
            | EtherealTerm::Category(_)
            | EtherealTerm::Universe(_) => self,
            EtherealTerm::EntityPath(TermEntityPath::Fugitive(_)) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(term) => term.reduce(db).into(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => term.reduce(db),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
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
        //     EtherealTerm::Subitem(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::AsTraitSubitem(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::TraitConstraint(term) => term.substitute(db, substitution).into(),
        //     EtherealTerm::Ritchie(_) => todo!(),
        // }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_from_declarative_term_explicit_application_or_ritchie_call(
    db: &dyn EtherealTermDb,
    declarative_term: DeclarativeTermExplicitApplicationOrRitchieCall,
    term_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    let function =
        EtherealTerm::from_declarative(db, declarative_term.function(db), term_ty_expectation)?;
    match function.raw_ty(db)? {
        RawType::Declarative(declarative_ty) => match declarative_ty {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Symbol(_) => todo!(),
            DeclarativeTerm::Variable(_) => todo!(),
            DeclarativeTerm::EntityPath(_) => todo!(),
            DeclarativeTerm::Category(_) => todo!(),
            DeclarativeTerm::Universe(_) => todo!(),
            DeclarativeTerm::Curry(_) => {
                let items = declarative_term.items(db);
                let argument = match items.len() {
                    0 => unreachable!(),
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
            DeclarativeTerm::Ritchie(_) => todo!(),
            DeclarativeTerm::Abstraction(_) => todo!(),
            DeclarativeTerm::ExplicitApplication(_) => todo!(),
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            DeclarativeTerm::Subitem(_) => todo!(),
            DeclarativeTerm::AsTraitSubitem(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            DeclarativeTerm::Wrapper(_) => todo!(),
        },
        RawType::Prelude(_) => todo!(),
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_from_declarative_term_list(
    db: &dyn EtherealTermDb,
    declarative_term_list: DeclarativeTermList,
    term_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    match term_ty_expectation {
        TermTypeExpectation::FinalDestinationEqsSort => {
            let term_menu = db.ethereal_term_menu(declarative_term_list.toolchain(db));
            let items = declarative_term_list.items(db);
            match items.len() {
                0 => Ok(term_menu.list_ty_ontology()),
                1 => Ok(term_menu.array_ty_ontology()),
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

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_from_declarative_term_wrapper(
    db: &dyn EtherealTermDb,
    declarative_term_wrapper: DeclarativeTermWrapper,
) -> EtherealTermResult<EtherealTerm> {
    let inner_ty = EtherealTerm::ty_from_declarative(db, declarative_term_wrapper.inner_ty(db))?;
    match inner_ty.application_expansion(db).function() {
        TermFunctionReduced::TypeOntology(ty_path) => match ty_path.refine(db) {
            Left(PreludeTypePath::Num(_)) | Left(PreludeTypePath::Borrow(_)) => Ok(inner_ty),
            _ => {
                let Some(toolchain) = inner_ty.toolchain(db) else {
                    todo!()
                };
                let leash_ty_ontology = db.ethereal_term_menu(toolchain).leash_ty_ontology();
                Ok(EtherealTermApplication::new_reduced(
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
    // match inner_ty.is_ty_copyable(db)? {
    //     true => Ok(inner_ty),
    //     false => {
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
            EtherealTerm::Variable(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::EntityPath(term) => term.show_with_db_fmt(f, db),
            EtherealTerm::Category(term) => f.write_str(&term.to_string()),
            EtherealTerm::Universe(term) => f.write_str(&term.to_string()),
            EtherealTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Application(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::Subitem(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::AsTraitSubitem(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl EtherealTermInstantiate for EtherealTerm {
    type Target = EtherealTerm;

    fn instantiate(
        self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealTermInstantiation,
    ) -> Self::Target {
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(term) => term.instantiate(db, instantiation),
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(_) => self,
            EtherealTerm::Category(_) => self,
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => term.instantiate(db, instantiation),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}
