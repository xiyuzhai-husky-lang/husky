use husky_fluffy_term::FluffyPlace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirPlace {
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

impl HirPlace {
    pub(crate) fn from_fluffy(place: FluffyPlace) -> HirPlace {
        match place {
            FluffyPlace::Const => HirPlace::Const,
            FluffyPlace::StackPure { location } => HirPlace::StackPure,
            FluffyPlace::ImmutableStackOwned { location } => HirPlace::ImmutableStackOwned,
            FluffyPlace::MutableStackOwned { location } => HirPlace::MutableStackOwned,
            FluffyPlace::Transient => HirPlace::Transient,
            FluffyPlace::Ref { guard } => HirPlace::Ref,
            FluffyPlace::RefMut { guard } => HirPlace::RefMut,
            FluffyPlace::Leashed => HirPlace::Leashed,
            FluffyPlace::Todo => HirPlace::Todo,
        }
    }
}
