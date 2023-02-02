use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fmt,
    rc::Rc,
    sync::Arc,
};

use vec_like::{AsVecMapEntry, InsertEntryRepeatError, VecMap, VecSet};

pub trait DisplayWithDb<Db: ?Sized> {
    fn display<'me, 'db>(&'me self, db: &'me Db) -> DisplayWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Ref(self),
            db,
            include_all_fields: false,
        }
    }

    fn display_with<'me, 'db>(
        &'me self,
        db: &'me Db,
        include_all_fields: bool,
    ) -> DisplayWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Ref(self),
            db,
            include_all_fields,
        }
    }

    /// Be careful when using this method inside a tracked function,
    /// because the default macro generated implementation will read all fields,
    /// maybe introducing undesired dependencies.
    fn debug_all<'me, 'db>(&'me self, db: &'me Db) -> DisplayWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Ref(self),
            db,
            include_all_fields: true,
        }
    }

    fn into_debug<'me, 'db>(self, db: &'me Db) -> DisplayWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Box(Box::new(self)),
            db,
            include_all_fields: false,
        }
    }

    /// Be careful when using this method inside a tracked function,
    /// because the default macro generated implementation will read all fields,
    /// maybe introducing undesired dependencies.
    fn into_debug_all<'me, 'db>(self, db: &'me Db) -> DisplayWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Box(Box::new(self)),
            db,
            include_all_fields: true,
        }
    }

    /// if `include_all_fields` is `false` only identity fields should be read, which means:
    ///     - for [#\[salsa::input\]](salsa_macros::input) no fields
    ///     - for [#\[salsa::tracked\]](salsa_macros::tracked) only fields with `#[id]` attribute
    ///     - for [#\[salsa::interned\]](salsa_macros::interned) any field
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, include_all_fields: bool) -> fmt::Result;
}

pub struct DisplayWith<'me, Db: ?Sized> {
    value: BoxRef<'me, dyn DisplayWithDb<Db> + 'me>,
    db: &'me Db,
    include_all_fields: bool,
}

enum BoxRef<'me, T: ?Sized> {
    Box(Box<T>),
    Ref(&'me T),
}

impl<T: ?Sized> std::ops::Deref for BoxRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            BoxRef::Box(b) => b,
            BoxRef::Ref(r) => r,
        }
    }
}

impl<D: ?Sized> fmt::Display for DisplayWith<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DisplayWithDb::fmt(&*self.value, f, self.db, self.include_all_fields)
    }
}

impl<Db: ?Sized, T: ?Sized> DisplayWithDb<Db> for &T
where
    T: DisplayWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, include_all_fields: bool) -> fmt::Result {
        T::fmt(self, f, db, include_all_fields)
    }
}

impl<Db: ?Sized, T: ?Sized> DisplayWithDb<Db> for Box<T>
where
    T: DisplayWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, include_all_fields: bool) -> fmt::Result {
        T::fmt(self, f, db, include_all_fields)
    }
}

impl<Db: ?Sized, T> DisplayWithDb<Db> for Rc<T>
where
    T: DisplayWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, include_all_fields: bool) -> fmt::Result {
        T::fmt(self, f, db, include_all_fields)
    }
}

impl<Db: ?Sized, T: ?Sized> DisplayWithDb<Db> for Arc<T>
where
    T: DisplayWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, include_all_fields: bool) -> fmt::Result {
        T::fmt(self, f, db, include_all_fields)
    }
}

impl<Db: ?Sized> DisplayWithDb<Db> for Infallible {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, include_all_fields: bool) -> fmt::Result {
        unreachable!()
    }
}

/// This is used by the macro generated code.
/// If the field type implements `DebugWithDb`, uses that, otherwise, uses `Debug`.
/// That's the "has impl" trick (https://github.com/nvzqz/impls#how-it-works)
#[doc(hidden)]
pub mod helper {
    use super::{DisplayWith, DisplayWithDb};
    use std::{fmt, marker::PhantomData};

    pub trait Fallback<T: fmt::Debug, Db: ?Sized> {
        fn salsa_debug<'a, 'b>(
            a: &'a T,
            _db: &'b Db,
            _include_all_fields: bool,
        ) -> &'a dyn fmt::Debug {
            a
        }
    }

    pub struct SalsaDebug<T, Db: ?Sized>(PhantomData<T>, PhantomData<Db>);

    impl<T: DisplayWithDb<Db>, Db: ?Sized> SalsaDebug<T, Db> {
        #[allow(dead_code)]
        pub fn salsa_debug<'a, 'b: 'a>(
            a: &'a T,
            db: &'b Db,
            include_all_fields: bool,
        ) -> DisplayWith<'a, Db> {
            a.display_with(db, include_all_fields)
        }
    }

    impl<Everything, Db: ?Sized, T: fmt::Debug> Fallback<T, Db> for Everything {}
}
