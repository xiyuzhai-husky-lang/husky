use either::*;
use husky_fluffy_term::{FluffyLifetime, FluffyPlace};
use husky_stack_location::StackLocationIdx;
use husky_term_prelude::TermContract;

use crate::lifetime::HirLifetime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirPlace {
    Const,
    /// reduce to
    /// - ImmutableStackOwned if base type is known to be copyable
    /// - ImmutableReferenced if base type is known to be noncopyable
    StackPure {
        location: StackLocationIdx,
    },
    /// lvalue nonreference
    ImmutableStackOwned {
        location: StackLocationIdx,
    },
    /// lvalue nonreference
    MutableStackOwned {
        location: StackLocationIdx,
    },
    // rvalue
    Transient,
    /// a place accessed through ref
    ///
    /// can be converted to
    /// - `&'a T`;
    ///
    ///     If guard is `Left(stack_location_idx)`
    ///     then `'a` is the time that location is borrowed;
    ///     else `'a` is equal to the lifetime of that guard.
    /// - `T` when `T` is copyable
    Ref {
        /// Guard is overwritten when composed with references.
        ///
        /// To see this, consider the following code
        ///
        /// ```husky
        /// struct A<'a> { x: &'a []i32}
        /// ```
        ///
        /// let `a` be a reference to `A<'b>`, then `a.x` is a valid for `'b` time,
        /// even if `a` is short lived.
        guard: Either<StackLocationIdx, HirLifetime>,
    },
    /// a place accessed through ref mut
    ///
    /// can be converted to
    /// - `&'a mut T`;
    ///
    ///     If guard is `Left(stack_location_idx)`
    ///     then `'a` is the time that location is borrowed;
    ///     else `'a` is equal to the lifetime of that guard.
    /// - `&'a T`;
    ///
    ///     If guard is `Left(stack_location_idx)`
    ///     then `'a` is the time that location is borrowed;
    ///     else `'a` is equal to the lifetime of that guard.
    /// - `T` when `T` is copyable
    RefMut {
        /// Guard is not overwritten when composed with references
        ///
        /// To see this, consider the following code
        ///
        /// ```husky
        /// struct A<'a> { mut x: &'a []i32}
        /// ```
        ///
        /// If `a` is a mutable reference of lifetime `'a` to `A<'b>`, then `a.x` is valid for `'a` time,
        /// even if `b` is long lived. So we should only care about the first lifetime.
        ///
        /// If `a` is a mutable variable on stack of type `A<'b>`, then `a.x` is valid as long as `a` is valid,
        /// even if `b` is long lived. So we should only care about the stack location.
        guard: Either<StackLocationIdx, HirLifetime>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
}

impl HirPlace {
    pub fn from_fluffy(place: FluffyPlace) -> HirPlace {
        match place {
            FluffyPlace::Const => HirPlace::Const,
            FluffyPlace::StackPure { location } => HirPlace::StackPure { location },
            FluffyPlace::ImmutableStackOwned { location } => {
                HirPlace::ImmutableStackOwned { location }
            }
            FluffyPlace::MutableStackOwned { location } => HirPlace::MutableStackOwned { location },
            FluffyPlace::Transient => HirPlace::Transient,
            FluffyPlace::Ref { guard } => HirPlace::Ref {
                guard: hir_place_guard_from_fluffy(guard),
            },
            FluffyPlace::RefMut { guard } => HirPlace::RefMut {
                guard: hir_place_guard_from_fluffy(guard),
            },
            FluffyPlace::Leashed => HirPlace::Leashed,
            FluffyPlace::Todo => HirPlace::Todo,
        }
    }

    pub fn location(self) -> Option<StackLocationIdx> {
        match self {
            HirPlace::StackPure { location }
            | HirPlace::ImmutableStackOwned { location }
            | HirPlace::MutableStackOwned { location }
            | HirPlace::Ref {
                guard: Left(location),
            }
            | HirPlace::RefMut {
                guard: Left(location),
            } => Some(location),
            _ => None,
        }
    }
}

fn hir_place_guard_from_fluffy(
    guard: Either<StackLocationIdx, FluffyLifetime>,
) -> Either<StackLocationIdx, HirLifetime> {
    match guard {
        Left(location) => Left(location),
        Right(lifetime) => Right(HirLifetime::from_fluffy(lifetime)),
    }
}
