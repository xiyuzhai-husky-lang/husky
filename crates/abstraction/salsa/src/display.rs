use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fmt,
    rc::Rc,
    sync::Arc,
};

use vec_like::{AsVecMapEntry, InsertEntryRepeatError, VecMap, VecSet};

pub struct DisplayFormatLevel(u8);

impl DisplayFormatLevel {
    pub fn root() -> Self {
        DisplayFormatLevel(0)
    }

    pub fn next(&self) -> Self {
        DisplayFormatLevel(self.0 + 1)
    }

    pub fn is_root(&self) -> bool {
        self.0 == 0
    }

    // do not leak this!!
    fn reproduce(&self) -> DisplayFormatLevel {
        DisplayFormatLevel(self.0)
    }
}

pub trait DisplayWithDb<Db: ?Sized> {
    fn display<'me, 'db>(&'me self, db: &'me Db) -> DisplayWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Ref(self),
            db,
            level: DisplayFormatLevel::root(),
        }
    }

    fn display_with<'me, 'db>(
        &'me self,
        db: &'me Db,
        level: DisplayFormatLevel,
    ) -> DisplayWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Ref(self),
            db,
            level,
        }
    }

    /// if `level` is `false` only identity fields should be read, which means:
    ///     - for [#\[salsa::input\]](salsa_macros::input) no fields
    ///     - for [#\[salsa::tracked\]](salsa_macros::tracked) only fields with `#[id]` attribute
    ///     - for [#\[salsa::interned\]](salsa_macros::interned) any field
    fn display_with_db_fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        db: &Db,
        level: DisplayFormatLevel,
    ) -> fmt::Result;
}

pub struct DisplayWith<'me, Db: ?Sized> {
    value: BoxRef<'me, dyn DisplayWithDb<Db> + 'me>,
    db: &'me Db,
    level: DisplayFormatLevel,
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
        DisplayWithDb::display_with_db_fmt(&*self.value, f, self.db, self.level.reproduce())
    }
}

impl<Db: ?Sized, T: ?Sized> DisplayWithDb<Db> for &T
where
    T: DisplayWithDb<Db>,
{
    fn display_with_db_fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        db: &Db,
        level: DisplayFormatLevel,
    ) -> fmt::Result {
        T::display_with_db_fmt(self, f, db, level)
    }
}

impl<Db: ?Sized, T: ?Sized> DisplayWithDb<Db> for Box<T>
where
    T: DisplayWithDb<Db>,
{
    fn display_with_db_fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        db: &Db,
        level: DisplayFormatLevel,
    ) -> fmt::Result {
        T::display_with_db_fmt(self, f, db, level)
    }
}

impl<Db: ?Sized, T> DisplayWithDb<Db> for Rc<T>
where
    T: DisplayWithDb<Db>,
{
    fn display_with_db_fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        db: &Db,
        level: DisplayFormatLevel,
    ) -> fmt::Result {
        T::display_with_db_fmt(self, f, db, level)
    }
}

impl<Db: ?Sized, T: ?Sized> DisplayWithDb<Db> for Arc<T>
where
    T: DisplayWithDb<Db>,
{
    fn display_with_db_fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        db: &Db,
        level: DisplayFormatLevel,
    ) -> fmt::Result {
        T::display_with_db_fmt(self, f, db, level)
    }
}

impl<Db: ?Sized> DisplayWithDb<Db> for Infallible {
    fn display_with_db_fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        db: &Db,
        level: DisplayFormatLevel,
    ) -> fmt::Result {
        unreachable!()
    }
}

/// This is used by the macro generated code.
/// If the field type implements `DebugWithDb`, uses that, otherwise, uses `Debug`.
/// That's the "has impl" trick (https://github.com/nvzqz/impls#how-it-works)
#[doc(hidden)]
pub mod helper {
    use crate::DisplayFormatLevel;

    use super::{DisplayWith, DisplayWithDb};
    use std::{fmt, marker::PhantomData};

    pub trait Fallback<T: fmt::Debug, Db: ?Sized> {
        fn salsa_debug<'a, 'b>(
            a: &'a T,
            _db: &'b Db,
            _level: DisplayFormatLevel,
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
            level: DisplayFormatLevel,
        ) -> DisplayWith<'a, Db> {
            a.display_with(db, level)
        }
    }

    impl<Everything, Db: ?Sized, T: fmt::Debug> Fallback<T, Db> for Everything {}
}
