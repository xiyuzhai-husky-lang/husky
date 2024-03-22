use either::*;
use husky_eth_term::term::svar::EthSvar;
use husky_place::place::{idx::PlaceIdx, EthPlace};
use husky_term_prelude::Contract;
use thiserror::Error;

use crate::FlyLifetime;

/// `PlaceQual` qualifies the place of a base type `T`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlyQuary {
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
        guard: Either<EthPlace, FlyLifetime>,
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
        lifetime: Option<FlyLifetime>,
    },
    /// stored in database
    /// always immutable
    Leashed {
        place_idx: Option<PlaceIdx>,
    },
    Todo,
    #[deprecated(note = "consider more carefully")]
    EtherealSymbol(EthSvar),
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum FlyPlaceError {
    #[error("cannot convert to const")]
    CannotConvertToConst,
}

pub type FlyPlaceResult<T> = Result<T, FlyPlaceError>;

impl FlyQuary {
    pub(crate) fn bind(&self, contract: Contract) -> FlyPlaceResult<()> {
        match (contract, self) {
            (Contract::Const, FlyQuary::Const) => Ok(()),
            (Contract::Const, _) => Err(FlyPlaceError::CannotConvertToConst),
            (Contract::Leash, FlyQuary::Leashed { .. }) => Ok(()),
            (Contract::Leash, _) => todo!("error"),
            (Contract::Pure, _) => Ok(()),
            (Contract::Move, FlyQuary::Const) => Ok(()),
            (Contract::Move, FlyQuary::StackPure { place }) => Ok(()),
            (Contract::Move, FlyQuary::ImmutableOnStack { place }) => Ok(()),
            (Contract::Move, FlyQuary::MutableOnStack { place }) => Ok(()),
            (Contract::Move, FlyQuary::Transient) => Ok(()),
            (Contract::Move, FlyQuary::Ref { guard }) => Ok(()), // ad hoc
            (Contract::Move, FlyQuary::RefMut { .. }) => todo!(),
            (Contract::Move, FlyQuary::Leashed { .. }) => Ok(()),
            (Contract::Move, FlyQuary::Todo) => todo!(),
            (Contract::Borrow, FlyQuary::Const) => todo!(),
            (Contract::Borrow, FlyQuary::StackPure { place }) => todo!(),
            (Contract::Borrow, FlyQuary::ImmutableOnStack { place }) => todo!(),
            (Contract::Borrow, FlyQuary::MutableOnStack { place }) => todo!(),
            (Contract::Borrow, FlyQuary::Transient) => todo!(),
            (Contract::Borrow, FlyQuary::Ref { guard }) => todo!(),
            (Contract::Borrow, FlyQuary::RefMut { .. }) => todo!(),
            (Contract::Borrow, FlyQuary::Leashed { .. }) => todo!(),
            (Contract::Borrow, FlyQuary::Todo) => todo!(),
            (Contract::BorrowMut, FlyQuary::Const) => todo!(),
            (Contract::BorrowMut, FlyQuary::StackPure { place }) => todo!(),
            (Contract::BorrowMut, FlyQuary::ImmutableOnStack { place }) => todo!(),
            (Contract::BorrowMut, FlyQuary::MutableOnStack { place }) => todo!(),
            (Contract::BorrowMut, FlyQuary::Transient) => Ok(()),
            (Contract::BorrowMut, FlyQuary::Ref { guard }) => todo!(),
            (Contract::BorrowMut, FlyQuary::RefMut { .. }) => Ok(()),
            (Contract::BorrowMut, FlyQuary::Leashed { .. }) => todo!(),
            (Contract::BorrowMut, FlyQuary::Todo) => todo!(),
            (Contract::At, FlyQuary::Const) => todo!(),
            (Contract::At, FlyQuary::StackPure { place }) => todo!(),
            (Contract::At, FlyQuary::ImmutableOnStack { place }) => todo!(),
            (Contract::At, FlyQuary::MutableOnStack { place }) => todo!(),
            (Contract::At, FlyQuary::Transient) => todo!(),
            (Contract::At, FlyQuary::Ref { guard }) => todo!(),
            (Contract::At, FlyQuary::RefMut { .. }) => todo!(),
            (Contract::At, FlyQuary::Leashed { .. }) => todo!(),
            (Contract::At, FlyQuary::Todo) => todo!(),
            (Contract::Move, FlyQuary::EtherealSymbol(_)) => todo!(),
            (Contract::Borrow, FlyQuary::EtherealSymbol(_)) => todo!(),
            (Contract::BorrowMut, FlyQuary::EtherealSymbol(_)) => todo!(),
            (Contract::At, FlyQuary::EtherealSymbol(_)) => todo!(),
        }
    }

    pub fn place(self) -> Option<EthPlace> {
        match self {
            FlyQuary::StackPure { place }
            | FlyQuary::ImmutableOnStack { place }
            | FlyQuary::MutableOnStack { place }
            | FlyQuary::Ref { guard: Left(place) }
            | FlyQuary::RefMut { place, .. } => Some(place),
            FlyQuary::EtherealSymbol(svar) => Some(svar.into()),
            FlyQuary::Const
            | FlyQuary::Transient
            | FlyQuary::Leashed { .. }
            | FlyQuary::Todo
            | FlyQuary::Ref { guard: Right(_) } => None,
        }
    }
}
