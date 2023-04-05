use super::*;

/// suppose `T` is base type
#[derive(Debug, PartialEq, Eq)]
pub enum Qual {
    // lvalue nonreference
    ImmutableMovable {
        location: StackLocationIdx,
    },
    // lvalue nonreference
    MutableMovable {
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
        guard: Either<StackLocationIdx, LifetimeIdx>,
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
        guard: Either<StackLocationIdx, LifetimeIdx>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StackLocationIdx {}

#[derive(Debug, PartialEq, Eq)]
pub struct LifetimeIdx {}
