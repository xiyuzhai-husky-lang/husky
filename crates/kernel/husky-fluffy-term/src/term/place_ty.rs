use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PlaceType(SolidTerm);

impl PlaceType {
    pub(crate) fn new(region: &mut FluffyTermRegion, place: Place, ty: Term) -> Self {
        region.solid_terms_mut();
        PlaceType(SolidTerm::new())
    }
}

/// maybe this is comparable with viewtype or viewt@ype in ATS?
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct PlaceTypeData {
    place: Place,
    ty: Term,
}

impl Into<FluffyTerm> for PlaceType {
    fn into(self) -> FluffyTerm {
        todo!()
    }
}

/// `PlaceQual` qualifies the place of a base type `T`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
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
        guard: Either<StackLocationIdx, FluffyLifetimeIdx>,
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
        guard: Either<StackLocationIdx, FluffyLifetimeIdx>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StackLocationIdx(LocalSymbolIdx);

impl From<LocalSymbolIdx> for StackLocationIdx {
    fn from(idx: LocalSymbolIdx) -> Self {
        Self(idx)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyLifetimeIdx {}

// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// pub struct PlaceTypeIdx(FluffyTermIdx);

// impl Into<FluffyTerm> for PlaceTypeIdx {
//     fn into(self) -> FluffyTerm {
//         self.0.into()
//     }
// }
