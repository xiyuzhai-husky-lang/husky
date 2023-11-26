use either::*;
use maybe_result::*;
use relative_path::{RelativePath, RelativePathBuf};
use smallvec::{Array, SmallVec};
use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fmt,
    rc::Rc,
    sync::Arc,
};
use vec_like::{
    error::InsertEntryRepeatError, AsVecMapEntry, SmallVecMap, SmallVecSet, VecMap, VecSet,
};

use crate::Db;

pub trait DebugWithDb {
    fn debug<'me, 'db>(&'me self, db: &'me Db) -> DebugWith<'me>
    where
        Self: Sized + 'me,
    {
        DebugWith {
            value: BoxRef::Ref(self),
            db,
        }
    }

    fn debug_with<'me, 'db>(&'me self, db: &'me Db) -> DebugWith<'me>
    where
        Self: Sized + 'me,
    {
        DebugWith {
            value: BoxRef::Ref(self),
            db,
        }
    }

    // /// Be careful when using this method inside a tracked function,
    // /// because the default macro generated implementation will read all fields,
    // /// maybe introducing undesired dependencies.
    // fn debug_all<'me, 'db>(&'me self, db: &'me Db) -> DebugWith<'me, Db>
    // where
    //     Self: Sized + 'me,
    // {
    //     DebugWith {
    //         value: BoxRef::Ref(self),
    //         db,
    //         level: true,
    //     }
    // }

    // fn into_debug<'me, 'db>(self, db: &'me Db) -> DebugWith<'me, Db>
    // where
    //     Self: Sized + 'me,
    // {
    //     DebugWith {
    //         value: BoxRef::Box(Box::new(self)),
    //         db,
    //         level: false,
    //     }
    // }

    // /// Be careful when using this method inside a tracked function,
    // /// because the default macro generated implementation will read all fields,
    // /// maybe introducing undesired dependencies.
    // fn into_debug_all<'me, 'db>(self, db: &'me Db) -> DebugWith<'me, Db>
    // where
    //     Self: Sized + 'me,
    // {
    //     DebugWith {
    //         value: BoxRef::Box(Box::new(self)),
    //         db,
    //         level: true,
    //     }
    // }

    /// if `level` is `false` only identity fields should be read, which means:
    ///     - for [#\[salsa::input\]](salsa_macros::input) no fields
    ///     - for [#\[salsa::tracked\]](salsa_macros::tracked) only fields with `#[id]` attribute
    ///     - for [#\[salsa::interned\]](salsa_macros::interned) any field
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result;
}

pub struct DebugWith<'me> {
    value: BoxRef<'me, dyn DebugWithDb + 'me>,
    db: &'me Db,
}

pub enum BoxRef<'me, T: ?Sized> {
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

impl fmt::Debug for DebugWith<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DebugWithDb::fmt(&*self.value, f, self.db)
    }
}

impl DebugWithDb for () {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        f.write_str("()")
    }
}

impl DebugWithDb for u8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl DebugWithDb for u16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl DebugWithDb for u32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl DebugWithDb for u64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl DebugWithDb for usize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<T: ?Sized> DebugWithDb for &T
where
    T: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        T::fmt(self, f, db)
    }
}

impl<T: ?Sized> DebugWithDb for Box<T>
where
    T: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        T::fmt(self, f, db)
    }
}

impl<T> DebugWithDb for Rc<T>
where
    T: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        T::fmt(self, f, db)
    }
}

impl<T: ?Sized> DebugWithDb for Arc<T>
where
    T: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        T::fmt(self, f, db)
    }
}

impl<T> DebugWithDb for [T]
where
    T: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

impl<T> DebugWithDb for Vec<T>
where
    T: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

impl<T: Array> DebugWithDb for SmallVec<T>
where
    <T as Array>::Item: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

impl<T> DebugWithDb for Option<T>
where
    T: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let me = self.as_ref().map(|v| v.debug_with(db));
        fmt::Debug::fmt(&me, f)
    }
}

impl<T, E> DebugWithDb for Result<T, E>
where
    T: DebugWithDb,
    E: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        match self {
            Ok(t) => f.debug_tuple("Ok").field(&t.debug_with(db)).finish(),
            Err(e) => f.debug_tuple("Err").field(&e.debug_with(db)).finish(),
        }
    }
}

pub trait ExpectWithDb {
    type Output;

    fn expect_with_db(self, db: &Db, msg: &str) -> Self::Output;
}

impl<T, E> ExpectWithDb for Result<T, E>
where
    E: DebugWithDb,
{
    type Output = T;

    #[track_caller]
    fn expect_with_db(self, db: &Db, msg: &str) -> Self::Output {
        match self {
            Ok(t) => t,
            Err(e) => panic!("{msg}: {:?}", e.debug(db)),
        }
    }
}

impl<T, E> DebugWithDb for MaybeResult<T, E>
where
    T: DebugWithDb,
    E: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        match self {
            JustOk(t) => f.debug_tuple("JustOk").field(&t.debug_with(db)).finish(),
            JustErr(e) => f.debug_tuple("JustErr").field(&e.debug_with(db)).finish(),
            Nothing => f.write_str("Nothing"),
        }
    }
}

impl<L, R> DebugWithDb for Either<L, R>
where
    L: DebugWithDb,
    R: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        match self {
            Left(l) => f.debug_tuple("Left").field(&l.debug_with(db)).finish(),
            Right(r) => f.debug_tuple("Right").field(&r.debug_with(db)).finish(),
        }
    }
}

impl DebugWithDb for Infallible {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        unreachable!()
    }
}

impl<K, V, S> DebugWithDb for HashMap<K, V, S>
where
    K: DebugWithDb,
    V: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self
            .iter()
            .map(|(k, v)| (k.debug_with(db), v.debug_with(db)));
        f.debug_map().entries(elements).finish()
    }
}

impl<K> DebugWithDb for VecSet<K>
where
    K: PartialEq + Eq + DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.data().iter().map(|v| v.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

impl<K, const N: usize> DebugWithDb for SmallVecSet<K, N>
where
    K: PartialEq + Eq + DebugWithDb,
    [K; N]: Array<Item = K>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.data().iter().map(|v| v.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

impl<K, V> DebugWithDb for VecMap<V>
where
    K: PartialEq + Eq,
    V: AsVecMapEntry<K = K> + DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.data().iter().map(|v| v.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

impl<K, V, const N: usize> DebugWithDb for SmallVecMap<V, N>
where
    K: PartialEq + Eq,
    V: AsVecMapEntry<K = K> + DebugWithDb,
    [V; N]: Array<Item = V>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.data().iter().map(|v| v.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

impl DebugWithDb for RelativePathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

impl DebugWithDb for RelativePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _db: &Db) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

impl<Entry> DebugWithDb for InsertEntryRepeatError<Entry>
where
    Entry: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        f.debug_struct("EntryRepeatError::Insert")
            .field("old", &self.old)
            .field("new", &self.new.debug_with(db))
            .finish()
    }
}

impl<A, B> DebugWithDb for (A, B)
where
    A: DebugWithDb,
    B: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.0.debug_with(db))
            .field(&self.1.debug_with(db))
            .finish()
    }
}

impl<A, B, C> DebugWithDb for (A, B, C)
where
    A: DebugWithDb,
    B: DebugWithDb,
    C: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.0.debug_with(db))
            .field(&self.1.debug_with(db))
            .field(&self.2.debug_with(db))
            .finish()
    }
}

impl<V, S> DebugWithDb for HashSet<V, S>
where
    V: DebugWithDb,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db));
        f.debug_list().entries(elements).finish()
    }
}

/// This is used by the macro generated code.
/// If the field type implements `DebugWithDb`, uses that, otherwise, uses `Debug`.
/// That's the "has impl" trick (https://github.com/nvzqz/impls#how-it-works)
#[doc(hidden)]
pub mod helper {
    use super::{DebugWith, DebugWithDb};
    use crate::Db;
    use std::{fmt, marker::PhantomData};

    pub trait Fallback<T: fmt::Debug> {
        fn salsa_debug<'a, 'b>(a: &'a T, _db: &'b Db) -> &'a dyn fmt::Debug {
            a
        }
    }

    pub struct SalsaDebug<T>(PhantomData<T>);

    impl<T: DebugWithDb> SalsaDebug<T> {
        #[allow(dead_code)]
        pub fn salsa_debug<'a, 'b: 'a>(a: &'a T, db: &'b Db) -> DebugWith<'a> {
            a.debug_with(db)
        }
    }

    impl<Everything, T: fmt::Debug> Fallback<T> for Everything {}
}
