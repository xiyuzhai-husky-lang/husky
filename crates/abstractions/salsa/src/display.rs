use crate::Database;
use std::{convert::Infallible, fmt, rc::Rc, sync::Arc};

pub trait DisplayWithDb {
    fn display<'me, 'db>(&'me self, db: &'me dyn Database) -> DisplayWith<'me>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Ref(self),
            db,
        }
    }

    fn display_with<'me, 'db>(&'me self, db: &'me dyn Database) -> DisplayWith<'me>
    where
        Self: Sized + 'me,
    {
        DisplayWith {
            value: BoxRef::Ref(self),
            db,
        }
    }

    /// if `level` is `false` only identity fields should be read, which means:
    ///     - for [#\[salsa::input\]](salsa_macros::input) no fields
    ///     - for [#\[salsa::tracked\]](salsa_macros::tracked) only fields with `#[id]` attribute
    ///     - for [#\[salsa::interned\]](salsa_macros::interned) any field
    fn display_with_db_fmt(&self, f: &mut fmt::Formatter<'_>, db: &dyn Database) -> fmt::Result;
}

pub struct DisplayWith<'me> {
    value: BoxRef<'me, dyn DisplayWithDb + 'me>,
    db: &'me dyn Database,
}

impl<'me> std::fmt::Debug for DisplayWith<'me> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DisplayWithDb::display_with_db_fmt(&*self.value, f, self.db)
    }
}

#[allow(dead_code)]
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

impl fmt::Display for DisplayWith<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DisplayWithDb::display_with_db_fmt(&*self.value, f, self.db)
    }
}

impl<T: ?Sized> DisplayWithDb for &T
where
    T: DisplayWithDb,
{
    fn display_with_db_fmt(&self, f: &mut fmt::Formatter<'_>, db: &dyn Database) -> fmt::Result {
        T::display_with_db_fmt(self, f, db)
    }
}

impl<T: ?Sized> DisplayWithDb for Box<T>
where
    T: DisplayWithDb,
{
    fn display_with_db_fmt(&self, f: &mut fmt::Formatter<'_>, db: &dyn Database) -> fmt::Result {
        T::display_with_db_fmt(self, f, db)
    }
}

impl<T> DisplayWithDb for Rc<T>
where
    T: DisplayWithDb,
{
    fn display_with_db_fmt(&self, f: &mut fmt::Formatter<'_>, db: &dyn Database) -> fmt::Result {
        T::display_with_db_fmt(self, f, db)
    }
}

impl<T: ?Sized> DisplayWithDb for Arc<T>
where
    T: DisplayWithDb,
{
    fn display_with_db_fmt(&self, f: &mut fmt::Formatter<'_>, db: &dyn Database) -> fmt::Result {
        T::display_with_db_fmt(self, f, db)
    }
}

impl DisplayWithDb for Infallible {
    fn display_with_db_fmt(&self, _f: &mut fmt::Formatter<'_>, _db: &dyn Database) -> fmt::Result {
        unreachable!()
    }
}

/// This is used by the macro generated code.
/// If the field type implements `DebugWithDb`, uses that, otherwise, uses `Debug`.
/// That's the "has impl" trick (https://github.com/nvzqz/impls#how-it-works)
#[doc(hidden)]
pub mod helper {
    use crate::Database;

    use super::{DisplayWith, DisplayWithDb};
    use std::{fmt, marker::PhantomData};

    pub trait Fallback<T: fmt::Debug, Db: ?Sized> {
        fn salsa_debug<'a, 'b>(a: &'a T, _db: &'b dyn Database) -> &'a dyn fmt::Debug {
            a
        }
    }

    pub struct SalsaDebug<T, Db: ?Sized>(PhantomData<T>, PhantomData<Db>);

    impl<T: DisplayWithDb, Db: ?Sized> SalsaDebug<T, Db> {
        #[allow(dead_code)]
        pub fn salsa_debug<'a, 'b: 'a>(a: &'a T, db: &'b dyn Database) -> DisplayWith<'a> {
            a.display_with(db)
        }
    }

    impl<Everything, Db: ?Sized, T: fmt::Debug> Fallback<T, Db> for Everything {}
}
