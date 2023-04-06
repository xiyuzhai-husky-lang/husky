use super::*;

/// `PlaceQual` qualifies the place of a base type `T`
#[derive(Debug, PartialEq, Eq)]
pub enum LocalPlace {
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
    /// can be converted to
    /// - `&'a T`;
    ///
    ///     If guard is `Left(stack_location_idx)`
    ///     then `'a` is the time that location is borrowed;
    ///     else `'a` is equal to the lifetime of that guard.
    /// - `T` when `T` is copyable
    ImmutableReferenced {
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
        guard: Either<StackLocationIdx, LocalLifetimeIdx>,
    },
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
    MutableReferenced {
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
        guard: Either<StackLocationIdx, LocalLifetimeIdx>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StackLocationIdx {
    ExplicitParameter {
        current_symbol_idx: CurrentSymbolIdx,
    },
    Variable {
        current_symbol_idx: CurrentSymbolIdx,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalLifetimeIdx {}

/// maybe this is comparable with viewtype or viewt@ype in ATS?
#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct LocalTermPlaceType {
    place: LocalPlace,
    ty: LocalTerm,
}
