#![feature(trait_upcasting)]
mod category;
mod contract;
mod curry;
mod db;
mod error;
mod final_destination;
mod item_path;
mod literal;
mod modifier;
mod universe;

pub use self::category::*;
pub use self::contract::*;
pub use self::curry::*;
pub use self::db::*;
pub use self::error::*;
pub use self::final_destination::*;
pub use self::item_path::*;
pub use self::literal::*;
pub use self::modifier::*;
pub use self::universe::*;

use crate::literal::float::{TermF32Literal, TermF64Literal};
use husky_entity_path::*;

#[salsa::jar(db = TermPreludeDb)]
pub struct TermPreludeJar(
    TermI64Literal,
    TermI128Literal,
    TermI256Literal,
    TermISizeLiteral,
    TermU64Literal,
    TermU128Literal,
    TermU256Literal,
    TermUSizeLiteral,
    TermR64Literal,
    TermR128Literal,
    TermR256Literal,
    TermRSizeLiteral,
    TermF32Literal,
    TermF64Literal,
    TermNatLiteral,
    StringLiteralData,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum RitchieKind {
    Type(RitchieTypeKind),
    Trait(RitchieTraitKind),
}

impl RitchieKind {
    pub fn code(self) -> &'static str {
        match self {
            RitchieKind::Type(ritchie_ty_kind) => match ritchie_ty_kind {
                RitchieTypeKind::Fn => "fn(",
                RitchieTypeKind::Gn => "gn(",
            },
            RitchieKind::Trait(ritchie_trai_kind) => match ritchie_trai_kind {
                RitchieTraitKind::Fn => "Fn(",
                RitchieTraitKind::FnMut => "FnMut(",
                RitchieTraitKind::FnOnce => "FnOnce(",
                RitchieTraitKind::Gn => "Gn(",
            },
        }
    }

    pub fn ritchie_ty_kind(self) -> Option<RitchieTypeKind> {
        match self {
            RitchieKind::Type(ritchie_ty_kind) => Some(ritchie_ty_kind),
            RitchieKind::Trait(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieTypeKind {
    Fn,
    Gn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieTraitKind {
    Fn,
    FnMut,
    FnOnce,
    Gn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum TermTypeExpectation {
    FinalDestinationEqsSort,
    FinalDestinationEqsNonSortTypePath(TypePath),
    Any,
}

/// a type path can be interpreted in two different ways:
///
/// - \[explicit curries to\] a type
/// - \[explicit curries to\] a type constructor
///
/// the final curry destination of the two different interpretation are different
///
/// for example, the type of type path `List` can either be
///
/// - `∀ universe u, explicit covariant Sort u -> Sort u`,
///
///     the final curry destination is in universe `Sort u`
/// - `∀ universe u, explicit covariant (E: Sort u) -> () -> List E`,
///
///     the final curry destination is in universe `List E`
/// disambiguate between type itself (or template) and its instance or constructor
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypePathDisambiguation {
    OntologyConstructor,
    /// if type is a unit struct, this will become an instance,
    ///
    /// otherwise constructor
    InstanceConstructor,
}
