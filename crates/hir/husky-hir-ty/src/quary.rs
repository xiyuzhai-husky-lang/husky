use either::*;
use husky_fly_term::{FlyLifetime, FlyQuary};
use husky_place::place::EthPlace;

use crate::{lifetime::HirLifetime, HirQuarySvar};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirQuary {
    Const,
    /// reduce to
    /// - ImmutableOnStack if base type is known to be copyable
    /// - ImmutableReferenced if base type is known to be noncopyable
    StackPure {
        place: EthPlace,
    },
    /// lvalue nonreference
    ImmutableOnStack {
        place: EthPlace,
    },
    /// lvalue nonreference
    MutableOnStack {
        place: EthPlace,
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
        guard: Either<EthPlace, HirLifetime>,
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
        place: EthPlace,
        lifetime: Option<HirLifetime>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
    Svar(HirQuarySvar),
}

impl From<HirQuarySvar> for HirQuary {
    fn from(svar: HirQuarySvar) -> Self {
        HirQuary::Svar(svar)
    }
}

impl HirQuary {
    pub fn from_fly(place: FlyQuary) -> HirQuary {
        match place {
            FlyQuary::Const => HirQuary::Const,
            FlyQuary::StackPure { place } => HirQuary::StackPure { place },
            FlyQuary::ImmutableOnStack { place } => HirQuary::ImmutableOnStack { place },
            FlyQuary::MutableOnStack { place } => HirQuary::MutableOnStack { place },
            FlyQuary::Transient => HirQuary::Transient,
            FlyQuary::Ref { guard } => HirQuary::Ref {
                guard: hir_place_guard_from_fly(guard),
            },
            FlyQuary::RefMut { place, lifetime } => HirQuary::RefMut {
                place,
                lifetime: lifetime.map(HirLifetime::from_fly),
            },
            FlyQuary::Leashed => HirQuary::Leashed,
            FlyQuary::Todo => HirQuary::Todo,
            FlyQuary::EtherealSymbol(_) => todo!(),
        }
    }

    pub fn place(self) -> Option<EthPlace> {
        match self {
            HirQuary::StackPure { place }
            | HirQuary::ImmutableOnStack { place }
            | HirQuary::MutableOnStack { place }
            | HirQuary::Ref { guard: Left(place) }
            | HirQuary::RefMut { place, .. } => Some(place),
            _ => None,
        }
    }
}

fn hir_place_guard_from_fly(guard: Either<EthPlace, FlyLifetime>) -> Either<EthPlace, HirLifetime> {
    match guard {
        Left(place) => Left(place),
        Right(lifetime) => Right(HirLifetime::from_fly(lifetime)),
    }
}
