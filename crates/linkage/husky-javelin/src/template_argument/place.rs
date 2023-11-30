use crate::*;
use husky_hir_ty::place::HirPlace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JavelinPlace {
    Const,
    /// reduce to
    /// - ImmutableStackOwned if base type is known to be copyable
    /// - ImmutableReferenced if base type is known to be noncopyable
    StackPure,
    /// lvalue nonreference
    ImmutableStackOwned,
    /// lvalue nonreference
    MutableStackOwned,
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
    Ref,
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
    RefMut,
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
}
impl JavelinPlace {
    pub(crate) fn from_hir(place: HirPlace) -> JavelinPlace {
        match place {
            HirPlace::Const => JavelinPlace::Const,
            HirPlace::StackPure => JavelinPlace::StackPure,
            HirPlace::ImmutableStackOwned => JavelinPlace::ImmutableStackOwned,
            HirPlace::MutableStackOwned => JavelinPlace::MutableStackOwned,
            HirPlace::Transient => JavelinPlace::Transient,
            HirPlace::Ref => JavelinPlace::Ref,
            HirPlace::RefMut => JavelinPlace::RefMut,
            HirPlace::Leashed => JavelinPlace::Leashed,
            HirPlace::Todo => JavelinPlace::Todo,
        }
    }
}
