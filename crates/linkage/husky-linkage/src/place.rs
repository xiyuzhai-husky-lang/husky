use crate::*;
use husky_hir_ty::place::HirPlace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkagePlace {
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
impl LinkagePlace {
    pub(crate) fn from_hir(place: HirPlace) -> LinkagePlace {
        match place {
            HirPlace::Const => LinkagePlace::Const,
            HirPlace::StackPure => LinkagePlace::StackPure,
            HirPlace::ImmutableStackOwned => LinkagePlace::ImmutableStackOwned,
            HirPlace::MutableStackOwned => LinkagePlace::MutableStackOwned,
            HirPlace::Transient => LinkagePlace::Transient,
            HirPlace::Ref => LinkagePlace::Ref,
            HirPlace::RefMut => LinkagePlace::RefMut,
            HirPlace::Leashed => LinkagePlace::Leashed,
            HirPlace::Todo => LinkagePlace::Todo,
        }
    }
}
