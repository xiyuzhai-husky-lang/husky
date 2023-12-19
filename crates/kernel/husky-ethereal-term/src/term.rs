mod abstraction;
mod application;
mod as_trai_item;
mod constraint;
mod curry;
mod item;
mod ritchie;
mod rune;
mod symbol;

use std::fmt::{Debug, Display};

pub use self::abstraction::*;
pub use self::application::*;
pub use self::as_trai_item::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::item::*;
pub use self::ritchie::*;
pub use self::rune::*;
pub use self::symbol::*;

use crate::instantiation::*;
use crate::*;
use husky_coword::Ident;
use husky_declarative_term::DeclarativeTerm;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EtherealTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(TermLiteral),
    Symbol(EtherealTermSymbol),
    Rune(EtherealTermRune),
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
                                                    PreludeIntTypePath::U16 => match i.try_into() {
                                                        Ok::<u16, _>(i) => {
                                                            TermLiteral::U16(i).into()
                                                        }
                                                        Err(_) => todo!(), // Or handle the error as appropriate
                                                    },
                                                    PreludeIntTypePath::U32 => match i.try_into() {
                                                        Ok::<u32, _>(i) => {
                                                            TermLiteral::U32(i).into()
                                                        }
                                                        Err(_) => todo!(), // Or handle the error as appropriate
                                                    },
                                                    PreludeIntTypePath::U64 => match i.try_into() {
                                                        Ok::<u64, _>(i) => {
                                                            TermLiteral::U64(todo!()).into()
                                                        }
                                                        Err(_) => todo!(), // Or handle the error as appropriate
                                                    },
                                                    PreludeIntTypePath::U128 => {
                                                        match i.try_into() {
                                                            Ok::<u128, _>(i) => {
                                                                TermLiteral::U128(todo!()).into()
                                                            }
                                                            Err(_) => todo!(), // Or handle the error as appropriate
                                                        }
                                                    }
                                                    PreludeIntTypePath::USize => match i.try_into()
                                                    {
                                                        // use u64 for the sake of cross compilation
                                                        Ok::<u64, _>(i) => TermLiteral::USize(
                                                            TermUSizeLiteral::new(db, i),
                                                        )
                                                        .into(),
                                                        Err(_) => todo!(), // Or handle the error as appropriate
                                                    },
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
            DeclarativeTerm::Rune(declarative_term) => {
                EtherealTermRune::from_declarative(db, declarative_term)?.into()
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

    pub(crate) fn into_declarative(self, db: &::salsa::Db) -> DeclarativeTerm {
        match self {
            EtherealTerm::Literal(lit) => DeclarativeTermLiteral::Resolved(lit).into(),
            EtherealTerm::Symbol(s) => DeclarativeTermSymbol::new(
                db,
                s.toolchain(db),
                Ok(s.ty(db).into_declarative(db)),
                s.index(db).into(),
            )
            .into(),
            EtherealTerm::Rune(v) => DeclarativeTermRune::new(
                Ok(v.ty(db).into_declarative(db)),
                v.idx(db).disambiguator(),
                db,
            )
            .into(),
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

    pub fn from_raw_inner(db: &::salsa::Db, valid_term: DeclarativeTerm) -> Self {
        todo!()
    }

    fn reduce(self, db: &::salsa::Db) -> Self {
        match self {
            EtherealTerm::Literal(_)
            | EtherealTerm::Symbol(_)
            | EtherealTerm::Rune(_)
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

    pub(crate) fn is_reduced(self, db: &::salsa::Db) -> bool {
        self.reduce(db) == self
    }

    pub fn substitute(self, db: &::salsa::Db, substitution: &TermSubstitution) -> Self {
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
    db: &::salsa::Db,
    declarative_term: DeclarativeTermExplicitApplicationOrRitchieCall,
    term_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    let function =
        EtherealTerm::from_declarative(db, declarative_term.function(db), term_ty_expectation)?;
    match function.raw_ty(db)? {
        RawType::Declarative(declarative_ty) => match declarative_ty {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Symbol(_) => todo!(),
            DeclarativeTerm::Rune(_) => todo!(),
            DeclarativeTerm::EntityPath(_) => todo!(),
            DeclarativeTerm::Category(_) => {
                p!(declarative_term.debug(db), function.debug(db));
                todo!()
            }
            DeclarativeTerm::Universe(_) => todo!(),
            DeclarativeTerm::Curry(curry) => {
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
    db: &::salsa::Db,
    declarative_term_list: DeclarativeTermList,
    term_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    match term_ty_expectation {
        TermTypeExpectation::FinalDestinationEqsSort => {
            let toolchain = declarative_term_list.toolchain(db);
            let term_menu = db.ethereal_term_menu(toolchain);
            let items = declarative_term_list.items(db);
            match items.len() {
                0 => Ok(term_menu.list_ty_ontology()),
                1 => Ok(EtherealTermApplication::new_reduced(
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
    declarative_term_wrapper: DeclarativeTermWrapper,
) -> EtherealTermResult<EtherealTerm> {
    let inner_ty = EtherealTerm::ty_from_declarative(db, declarative_term_wrapper.inner_ty(db))?;
    match inner_ty.application_expansion(db).function() {
        TermFunctionReduced::TypeOntology(ty_path) => match ty_path.refine(db) {
            Left(PreludeTypePath::Num(_)) | Left(PreludeTypePath::Indirection(_)) => Ok(inner_ty),
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
            EtherealTerm::Subitem(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::AsTraitSubitem(term) => term.show_with_db_fmt(f, db, ctx),
            EtherealTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl EtherealInstantiate for EtherealTerm {
    type Output = EtherealTerm;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(term) => term.instantiate(db, instantiation),
            EtherealTerm::Rune(_) => todo!(),
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
