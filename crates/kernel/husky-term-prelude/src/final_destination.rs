use crate::*;

/// final destination of `A1 -> ... -> An` is equal to that of `An`
///
/// final destination of `A1 ... An` is equal to that of `A1`
///
/// final destination of `Sort` is `FinalDestination::Sort`
///
/// final destination of a type path `A` is `FinalDestination::TypePath(A)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TermPreludeDb, jar = TermPreludeJar)]
pub enum FinalDestination {
    Sort,
    TypeOntology,
    AnyOriginal,
    AnyDerived,
    Ritchie(RitchieKind),
}
